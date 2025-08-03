use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use shared::models::claims_model::Claims;

use crate::infrastructures::jwt_infra::verify_token;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthenMiddleware;

// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for AuthenMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenMiddlewareWorker<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenMiddlewareWorker { service }))
    }
}

pub struct AuthenMiddlewareWorker<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenMiddlewareWorker<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());

        // เช็คและ decode token
        let Some(cookie) = req.cookie("session") else {
            return Box::pin(async { Err(ErrorUnauthorized("No token")) });
        };

        let token = cookie.value();

        let claims = match verify_token(token, "secret") {
            Some(claims) => claims, // ✅ ตอนนี้ claims คือ struct Claims
            None => return Box::pin(async { Err(ErrorUnauthorized("Invalid token")) }),
        };

        // ✅ insert Claims เข้า request extension
        req.extensions_mut().insert::<Claims>(claims);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

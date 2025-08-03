use actix_web::{
    cookie::{Cookie, SameSite},
    post, web, HttpResponse, Responder,
};
use shared::models::auth_model::AuthModel;

use crate::{
    infrastructures::jwt_infra::issue_token,
    repositories::auth_repository::{MockUserRepo, UserRepository},
    services::auth_service::AuthService,
};

#[post("/auth")]
pub async fn authenticate(info: web::Json<AuthModel>) -> impl Responder {
    let secret = "secret";

    let username = &info.username;
    let password = &info.password;

    let repo = MockUserRepo::new();
    let check = AuthService::new(repo);

    match check.verify_password(username, password) {
        Ok(mut claims) => {
            let token = issue_token(secret, &mut claims);
            let cookie = Cookie::build("session", token)
                .http_only(true)
                .secure(true)
                .same_site(SameSite::None)
                .path("/")
                .finish();
            HttpResponse::Ok()
                .cookie(cookie) // ✅ ใช้ cookie แบบ type-safe
                .finish()
        }
        Err(err) => HttpResponse::Unauthorized().body(err.to_string()),
    }
}

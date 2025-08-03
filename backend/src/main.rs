use actix_cors::Cors;
use actix_web::middleware::Logger;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use backend::handlers::auth_handler::authenticate;
use backend::middlewares::auth_middelware::AuthenMiddleware;

#[get("/health")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Backend is running")
}

// #[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://localhost:8997");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // เพิ่ม logger middleware
            .wrap(
                Cors::default()
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://localhost:8080") // ✅ ต้องตรง origin frontend
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials() // ✅ เพื่อให้ browser รับ/ส่ง cookie ได้
                    .max_age(3600),
            )
            .service(
                web::resource("/test/hello")
                    .wrap(AuthenMiddleware)
                    .route(web::get().to(hello)), // ✅ ใช้ web::get().to(...) แทน route(handler)
            )
            .service(authenticate)
            .service(index)
    })
    .bind(("127.0.0.1", 8997))?
    .run()
    .await
}

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use shared::models::login_model::LoginModel;

#[post("/auth")]
async fn authenticate(info: web::Json<LoginModel>) -> impl Responder {
    let username = &info.username;

    let password = &info.password;

    if username == "admin" && password == "secret" {
        HttpResponse::Ok()
            .append_header((
                "Set-Cookie",
                "session=abc123; HttpOnly; SameSite=None; Secure; Path=/",
            ))
            .finish()
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

#[get("/health")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Backend is running")
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
            .service(authenticate)
            .service(index)
    })
    .bind(("127.0.0.1", 8997))?
    .run()
    .await
}

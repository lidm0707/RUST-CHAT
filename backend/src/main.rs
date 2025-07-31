use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct AuthRequest {
    username: String,
    password: String,
}

#[post("/auth")]
async fn authenticate(info: web::Json<AuthRequest>) -> impl Responder {
    let username = &info.username;
    let password = &info.password;

    // ตัวอย่างตรวจสอบง่าย ๆ (ควรแทนด้วยระบบฐานข้อมูลหรืออื่น ๆ)
    if username == "admin" && password == "secret" {
        HttpResponse::Ok().body("Authentication successful")
    } else {
        HttpResponse::Unauthorized().body("Invalid username or password")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server on http://localhost:8999");

    HttpServer::new(|| App::new().service(authenticate))
        .bind("127.0.0.1:8999")?
        .run()
        .await
}

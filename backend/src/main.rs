use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use shared::models::login_model::LoginModel;

#[post("/auth")]
async fn authenticate(info: web::Json<LoginModel>) -> impl Responder {
    let username = &info.user;
    let password = &info.password;

    if username == "admin" && password == "secret" {
        HttpResponse::Ok().body("Authentication successful")
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
    println!("Starting server on http://localhost:8999");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // ถ้าต้องการจำกัดโดเมนให้เปลี่ยนตรงนี้
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(authenticate)
            .service(index)
    })
    .bind("127.0.0.1:8999")?
    .run()
    .await
}

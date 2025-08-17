use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::middleware::Logger;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_ws::Session;
use backend::handlers::agent_login_handler::agent_login_ss_handler;
use backend::handlers::auth_handler::authenticate;
use backend::handlers::ws_room_handler::chat_ws;
use backend::middlewares::auth_middelware::AuthenMiddleware;
use backend::models::appstate::AppState;
use backend::models::chatrooms::Workspace;

#[get("/health")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Backend is running")
}

// #[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello test")
}

async fn me() -> impl Responder {
    HttpResponse::Ok().body("me 1")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(AppState::new());
    let data = web::Data::new(state.clone()); // ✅ ใช้ Data::new()
    println!("Starting server on http://127.0.0.1:8997");
    // ตั้งค่า RUST_LOG ก่อน init
    std::env::set_var("RUST_LOG", "debug,actix_web=debug"); // ตั้งให้ log debug ของ actix_web
    env_logger::init(); // ✅ เรียกหลัง set env
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    // .allowed_origin("http://127.0.0.1:8080")
                    // .allowed_origin("http://localhost:8080") // ✅ ต้องตรง origin frontend
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials() // ✅ เพื่อให้ browser รับ/ส่ง cookie ได้
                    .max_age(3600),
            )
            .app_data(data.clone()) // ส่งให้ App
            .service(agent_login_ss_handler)
            // .service(agent_logout)
            .service(chat_ws)
            .service(
                web::scope("/api")
                    .wrap(AuthenMiddleware)
                    .route("/me", web::get().to(me))
                    .route("/hello ", web::get().to(hello)), // ✅ ใช้ web::get().to(...) แทน route(handler)
            )
            .service(authenticate)
            .service(index)
    })
    .bind(("127.0.0.1", 8997))?
    .run()
    .await
}

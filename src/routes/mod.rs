use actix_web::{web, HttpResponse};

pub mod todos;

use crate::routes::todos::todos;

pub fn routes(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/")
      .route(web::get().to(|| HttpResponse::Ok().body("Rust Back-end is running.")))
  );
  cfg.service(
    web::scope("/api")
      .route("", web::get().to(|| HttpResponse::Ok().body("Welcome API")))
      .configure(todos)
  );
}

use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};

mod controllers;

use crate::controllers::todo;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();
  println!("Starting http server: 127.0.0.1:3000");

  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .service(web::resource("/").route(web::get().to(|| {
        HttpResponse::Ok().body("Welcome")
      })))
      .service(
        web::scope("/api")
          .service(
            web::resource("/todo")
              .route(web::get().to(todo::get_todo))
              .route(web::post().to(todo::add_todo))
          ).service(
            web::resource("/todo/{id}")
              .route(web::get().to(todo::get_todo_by_id))
              .route(web::delete().to(todo::delete_todo_by_id))
              .route(web::patch().to(todo::update_todo_by_id))
          )
      )
  })
  .bind("127.0.0.1:3000")?
  .run()
  .await
}
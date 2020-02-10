mod controllers;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};
use crate::routes::routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

  let port = 3000;

  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  println!("Starting http server: 127.0.0.1:{}", port);

  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .configure(routes)
  })
  .bind(format!("127.0.0.1:{}", port))?
  .run()
  .await
}

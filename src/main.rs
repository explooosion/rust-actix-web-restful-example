mod controllers;
mod routes;

use actix_web::{middleware::Logger, web, App, HttpServer};
use crate::routes::routes;
use crate::controllers::todo::{AppState, Todo};
use std::sync::Mutex;

async fn _index(data: web::Data<AppState>) -> String {
  let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
  // *counter += 1; // <- access counter inside MutexGuard
  *counter += 1;
  format!("Request number: {}", counter) // <- response with count
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let app_state = web::Data::new(AppState {
   counter: Mutex::new(0),
   todos: Mutex::new(
    vec![
      Todo { id: 1, name: format!("吃飯"), done: false },
      Todo { id: 2, name: format!("睡覺"), done: true },
      Todo { id: 3, name: format!("洗澡"), done: false },
    ]
   ),
  });

  HttpServer::new(move || {
    App::new()
      .wrap(Logger::default())
      // .data(AppState {
      //   app_name: String::from("Actix-web"),
      // })
      // .data(AppStateWithCounter {
      //   counter: Mutex::new(0),
      // })
      .app_data(app_state.clone()) // <- register the created data
      .configure(routes)
  })
  .bind("127.0.0.1:3000")?
  .run()
  .await
}

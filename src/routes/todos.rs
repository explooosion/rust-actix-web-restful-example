use actix_web::web;
use crate::controllers::todo;

pub fn todos(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/todos")
    .route(web::get().to(todo::get_todo))
    .route(web::post().to(todo::add_todo)),
  );
  cfg.service(
    web::resource("/todos/{id}")
    .route(web::get().to(todo::get_todo_by_id))
    .route(web::delete().to(todo::delete_todo_by_id))
    .route(web::patch().to(todo::update_todo_by_id)),
  );
}

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
  pub id: i32,
  pub name: String,
  pub done: bool,
}

pub struct AppState {
  // pub app_name: String,
  pub counter: Mutex<i32>,
  pub todos: Mutex<Vec<Todo>>,
}

pub async fn get_todo() -> impl Responder {
  HttpResponse::Ok().json(
    vec![
      Todo { id: 1, name: format!("吃飯"), done: false },
      Todo { id: 2, name: format!("睡覺"), done: true },
      Todo { id: 3, name: format!("洗澡"), done: false },
    ]
  )
  // let app_name = &data.app_name; // <- get app_name
  // HttpResponse::Ok().body(format!("Hello {}!", app_name))
}

pub async fn add_todo(todo: web::Json<Todo>) -> impl Responder {
  HttpResponse::Ok().json(Todo {
    id: todo.id,
    name: todo.name.to_string(),
    done: todo.done,
  })
}

pub async fn get_todo_by_id(data: web::Data<AppState>) -> impl Responder {
  let mut _todos = data.todos.lock().unwrap(); // <- get counter's MutexGuard
  // let todo = &_todos[0];
  HttpResponse::Ok().json(&_todos[0])
}

pub async fn delete_todo_by_id(todo: web::Json<Todo>) -> impl Responder {
  HttpResponse::Ok().json(Todo {
    id: todo.id,
    name: todo.name.to_string(),
    done: todo.done,
  })
}

pub async fn update_todo_by_id(todo: web::Json<Todo>) -> impl Responder {
  HttpResponse::Ok().json(Todo {
    id: todo.id,
    name: todo.name.to_string(),
    done: todo.done,
  })
}

// #[get("/hello")]
// async fn index3(req: HttpRequest) -> Result<HttpResponse> {
//     println!("{:?}", req);
//     HttpResponse::Ok().body("Welcome").async
// }

// async fn index4(info: web::Path<(i32, String)>) -> Result<String> {
//     Ok(format!("Welcome {}, userid {}!", info.1, info.0))
// }

// async fn index6(req: HttpRequest) -> Result<String> {
//     let name: String =
//         req.match_info().get("friend").unwrap().parse().unwrap();
//     let userid: i32 = req.match_info().query("userid").parse().unwrap();

//     Ok(format!("Welcome {}, userid {}!", name, userid))
// }

// async fn index7(info: web::Query<Info>) -> Result<String> {
//   Ok(format!("Welcome {}!", info.userid))
// }

// async fn index8(info: web::Json<Info>) -> Result<String> {
//   Ok(format!("Welcome {}!", info.userid))
// }

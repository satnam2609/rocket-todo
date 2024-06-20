use super::DbConn;
use crate::model::*;
use crate::repositories::*;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use serde_json::{json, Value};

#[rocket::get("/todos")]
pub async fn get_todos(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    TodoRepository::list_todos(&mut db, 100)
        .await
        .map(|tasks| json!(tasks))
        .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::post("/todos", format = "json", data = "<new_todo>")]
pub async fn create_todos(
    mut db: Connection<DbConn>,
    new_todo: Json<super::TempTodo>,
    _user: User,
) -> Result<Custom<Value>, Custom<Value>> {
    let new_todo = new_todo.into_inner();
    let new_todo = NewTodo {
        title: new_todo.title,
        description: new_todo.description,
        user_id: _user.id,
    };

    TodoRepository::create_one(&mut db, new_todo)
        .await
        .map(|todo| Custom(Status::Created, json!(todo)))
        .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::get("/todos/<id>")]
pub async fn get_todo(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    TodoRepository::find_one(&mut db, id)
        .await
        .map(|todo| json!(todo))
        .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::put("/todos/<id>", format = "json", data = "<todo>")]
pub async fn update_todo(
    mut db: Connection<DbConn>,
    id: i32,
    todo: Json<Todo>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    TodoRepository::update_one(&mut db, id, todo.into_inner())
        .await
        .map(|todo| json!(todo))
        .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::delete("/todos/<id>")]
pub async fn delete_todo(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    TodoRepository::delete_one(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
}

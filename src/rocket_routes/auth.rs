use super::CacheConn;
use super::DbConn;
use crate::auth;
use crate::auth::Credentials;
use crate::model::*;
use crate::repositories::*;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use serde_json::{json, Value};

#[rocket::post("/register", format = "json", data = "<new_user>")]
pub async fn register(
    mut db: Connection<DbConn>,
    new_user: Json<NewUser>,
) -> Result<Custom<Value>, Custom<Value>> {
    let new_user = new_user.into_inner();
    let result = UserRespository::find_by_email(&mut db, &new_user.email).await;
    if result.is_ok() {
        return Err(Custom(
            Status::Conflict,
            json!({ "error": "Email already exists"}),
        ));
    }
    let hashed_password = auth::generate_hash(new_user.password).unwrap();

    let new_user = NewUser {
        password: hashed_password,
        ..new_user
    };

    UserRespository::create_one(&mut db, new_user)
        .await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))
}

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>,
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>,
) -> Result<Value, Custom<Value>> {
    let credentials = credentials.into_inner();

    let user = UserRespository::find_by_email(&mut db, &credentials.email)
        .await
        .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))?;

    //verify password
    let session_id = auth::authorize_password(&user, credentials)
        .map_err(|_e| Custom(Status::InternalServerError, json!("Wrong Credentials")))?;

    cache
        .set_ex::<String, i32, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map_err(|_e| Custom(Status::InternalServerError, json!("Error")))?;

    Ok(json!({
     "token":session_id
    }))
}

 
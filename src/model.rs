use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Todo {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    pub user_id: i32,
    pub description: Option<String>,
    pub completed: bool,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=todos)]
pub struct NewTodo {
    pub title: String,
    pub description: Option<String>,
    pub user_id: i32,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]

pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_deserializing)]
    pub password: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

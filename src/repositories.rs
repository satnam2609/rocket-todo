use crate::model::*;
use crate::schema::*;
use diesel::prelude::*;
use diesel::query_dsl::methods::FilterDsl;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

pub struct TodoRepository;

impl TodoRepository {
    pub async fn create_one(c: &mut AsyncPgConnection, new_todo: NewTodo) -> QueryResult<Todo> {
        diesel::insert_into(todos::table)
            .values(new_todo)
            .get_result(c)
            .await
    }

    pub async fn list_todos(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Todo>> {
        todos::table.limit(limit).load(c).await
    }

    pub async fn find_one(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Todo> {
        todos::table.find(id).get_result(c).await
    }

    pub async fn update_one(c: &mut AsyncPgConnection, id: i32, todo: Todo) -> QueryResult<Todo> {
        diesel::update(todos::table.find(id))
            .set((
                todos::user_id.eq(todo.user_id),
                todos::title.eq(todo.title),
                todos::description.eq(todo.description),
                todos::completed.eq(todo.completed),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete_one(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(todos::table.find(id)).execute(c).await
    }
}

pub struct UserRespository;

impl UserRespository {
    pub async fn create_one(c: &mut AsyncPgConnection, new_user: NewUser) -> QueryResult<User> {
        diesel::insert_into(users::table)
            .values(new_user)
            .get_result(c)
            .await
    }

    pub async fn list_users(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<User>> {
        users::table.limit(limit).load(c).await
    }

    pub async fn get_one(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }

    pub async fn update_one(c: &mut AsyncPgConnection, id: i32, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set((
                users::email.eq(user.email),
                users::password.eq(user.password),
            ))
            .get_result(c)
            .await
    }

    pub async fn find_by_email(c: &mut AsyncPgConnection, email: &String) -> QueryResult<User> {
        diesel::QueryDsl::filter(users::table, users::email.eq(email))
            .get_result(c)
            .await
    }

    pub async fn delete_one(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users::table.find(id)).execute(c).await
    }
}

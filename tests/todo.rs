use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_todos() {
    //Setup
    let client = common::get_logged_in_client("someone@gmail.com", "1234");
    let todo_1 = common::create_todo(&client, "Leg day", "Squats, leg extensions");
    let todo_2 = common::create_todo(&client, "Back day", "Bent over rows, Pull ups");

    let response = client
        .get(format!("{}/todos", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&todo_1));
    assert!(json.as_array().unwrap().contains(&todo_2));

    common::delete_todo(&client, todo_1);
    common::delete_todo(&client, todo_2);
}

#[test]
fn test_create_todo() {
    let client = common::get_logged_in_client("someone@gmail.com", "1234");
    let todo = json!({
        "title":"Rust practice",
        "description":"Rocket framework practice"
    });

    let response = client
        .post(format!("{}/todos", common::APP_HOST))
        .json(&todo)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let todo: Value = response.json().unwrap();
    assert_eq!(
        todo,
        json!({
            "id":todo["id"],
            "title":"Rust practice",
            "description":"Rocket framework practice",
            "completed":false,
            "user_id":todo["user_id"],
        "created_at":todo["created_at"]
        })
    );

    common::delete_todo(&client, todo);
}

#[test]
fn test_get_todo() {
    let client = common::get_logged_in_client("someone@gmail.com", "1234");

    let todo = common::create_todo(&client, "Leg day ", "Squats, leg extensions");

    let response = client
        .get(format!("{}/todos/{}", common::APP_HOST, todo["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let responsed_todo: Value = response.json().unwrap();

    assert_eq!(responsed_todo, todo);

    common::delete_todo(&client, todo);
}

#[test]
fn test_update_test() {
    let client = common::get_logged_in_client("someone@gmail.com", "1234");

    let todo = common::create_todo(&client, "Leg day ", "Squats, leg extensions");

    let response = client
        .put(format!("{}/todos/{}", common::APP_HOST, todo["id"]))
        .json(&json!({
            "completed":true,
            "title":"Leg day",
            "description":"Squats, leg extensions",
            "id":todo["id"],
            "user_id":todo["user_id"],
            "created_at":todo["created_at"]
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let todo: Value = response.json().unwrap();
    assert_eq!(
        todo,
        json!({
            "completed":true,
            "title":"Leg day",
            "description":"Squats, leg extensions",
            "id":todo["id"],
            "user_id":todo["user_id"],
            "created_at":todo["created_at"]
        })
    );

    common::delete_todo(&client, todo);
}

#[test]
fn test_delete_todos() {
    let client = common::get_logged_in_client("someone@gmail.com", "1234");

    let todo = common::create_todo(&client, "Leg day ", "Squats, leg extensions");

    let response = client
        .delete(format!("{}/todos/{}", common::APP_HOST, todo["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

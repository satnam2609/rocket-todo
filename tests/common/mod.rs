pub const APP_HOST: &str = "http://127.0.0.1:8000";

use reqwest::StatusCode;
use reqwest::{
    blocking::{Client, ClientBuilder},
    header,
};

use serde_json::{json, Value};

pub fn create_todo(client: &Client, title: &str, description: &str) -> Value {
    let todo = json!({
        "title":title,
        "description":description
    });

    let response = client
        .post(format!("{}/todos", APP_HOST))
        .json(&todo)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_todo(client: &Client, todo: Value) {
    let response = client
        .delete(format!("{}/todos/{}", APP_HOST, todo["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn get_logged_in_client(email: &str, password: &str) -> Client {
    let client = Client::new();

    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&json!({
            "email":email,
            "password":password
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());

    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

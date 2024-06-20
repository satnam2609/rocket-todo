use reqwest::{blocking::Client, StatusCode};
use rocket::form::validate::Len;
use serde_json::{json, Value};
use std::process::Command;

pub mod common;

// #[test]
// fn test_register() {
//     //Setup
//     let client = Client::new();

//     let response = client
//         .post(format!("{}/register", common::APP_HOST))
//         .json(&json!({
//             "email":"example@gmail.com",
//             "password":"12345"
//         }))
//         .send()
//         .unwrap();

//     assert_eq!(response.status(), StatusCode::CREATED);

//     let user: Value = response.json().unwrap();

//     assert_eq!(
//         user,
//         json!({
//             "id":user["id"],
//             "email":"example@gmail.com",
//             "password":user["password"],
//             "created_at":user["created_at"]
//         })
//     );
// }

#[test]
fn test_login() {
    let client = Client::new();

    let response = client
        .post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "email":"satnamm143@gmail.com",
            "password":"1234"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);
}

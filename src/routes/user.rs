use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::services;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserResponseError {
    pub status: String,
}

#[get("/<user_id>")]
pub async fn get_user_by_id(user_id: i32) -> Json<User> {
    Json(services::user::get_user_by_id(user_id).await)
}

#[post("/", format = "json", data = "<user>")]
pub async fn create_user(user: Json<User>) -> Json<i32> {
    Json(services::user::create_user(user).await)
}

#[post("/login", format = "json", data = "<user>")]
pub async fn login_user(user: Json<UserLogin>,) 
-> Result<Json<UserResponse>, status::Unauthorized<Json<UserResponseError>>> {
    match services::user::login_user(user).await {
        Some(user) => Ok(Json(user)),
        None => Err(status::Unauthorized(Some(Json(UserResponseError {
            status: "Unauthorized".to_string(),
        })))),
    }
}

//#[get("/<user_id>/sessions")]
//fn get_user_sessions(user_id: u32) -> String {
//    Json(services::user::get_user_sessions(user_id))

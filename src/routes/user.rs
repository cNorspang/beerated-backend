use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

use crate::services;


#[derive(Debug,Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(Debug,Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserLogin {
    pub email: String,
    pub password: String,
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
pub async fn login_user(user: Json<UserLogin>) -> Json<i32> {
    Json(services::user::login_user(user).await)
}
//#[get("/<user_id>/sessions")]
//fn get_user_sessions(user_id: u32) -> String {
//    Json(services::user::get_user_sessions(user_id))

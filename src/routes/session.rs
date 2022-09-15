use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};

use crate::services;

#[derive(Debug,Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub session_id: u32,
}

#[get("/<session_id>")]
pub fn get_session_by_id(session_id: u32) -> Json<Session> {
    Json(services::session::get_session_by_id(session_id))
}

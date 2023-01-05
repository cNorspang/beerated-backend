use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use crate::services;

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Beer {
    pub id: u32,
    pub name: String,
    pub brewery: String,
    pub submitter_id: u32,
    pub alcohol_content: f32,
    pub score: f32,
}

#[get("/<beer_id>")]
pub fn get_beer_by_id(beer_id: u32) -> Json<Beer> {
    Json(services::beer::get_beer_by_id(beer_id))
}

#[get("/by_user/<user_id>")]
pub async fn get_beers_submitting_user_id(user_id: u32) -> Json<Vec<Beer>> {
    Json(services::beer::get_beer_by_submitter_id(user_id).await)
}
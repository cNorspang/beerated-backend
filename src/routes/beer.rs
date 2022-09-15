use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize };

use crate::services;


#[derive(Debug,Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Beer {
    pub id: u32,
    pub name: String,
    pub brewery: String,
    pub submitter_id: u32,
    pub alcohol_content: f32,
    pub score: f32
}

#[get("/<beer_id>")]
pub fn get_beer_by_id(beer_id: u32) -> Json<Beer> {
    Json(services::beer::get_beer_by_id(beer_id))
}

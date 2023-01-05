use crate::routes::beer::Beer;
use sqlx::{PgPool, Row};
use std::env;

pub fn get_beer_by_id(beer_id: u32) -> Beer {
    let id = beer_id;
    let name = "Westmalle Tripple";
    let brewery = "Brouwerij der Trappisten van Westmalle";
    let submitter_id = 1;
    let alcohol_content = 9.5;
    let score = 8.2;

    let beer = Beer {
        id,
        name: name.to_string(),
        brewery: brewery.to_string(),
        submitter_id,
        alcohol_content,
        score,
    };

    beer
}

pub async fn get_beer_by_submitter_id(user_id: u32) -> Vec<Beer> {
    let id = user_id;
    let sql_string = "SELECT * FROM beers WHERE submitter_id = $1";
    let pool = PgPool::connect(
        env::var("PSQL_CONNECTION_STRING")
            .expect("no env file")
            .as_str(),
    )
    .await
    .unwrap();

    let rows = sqlx::query(sql_string)
        .bind(id)
        .fetch_all(&pool)
        .await
        .unwrap();

    let mut beers: Vec<Beer> = Vec::new();

    for item in rows {
        let beer = Beer{
            id: item.get("id"),
            name: item.get("name"),
            brewery: item.get("brewery"),
            submitter_id: item.get("submitter_id"),
            alcohol_content: item.get("alchol_content"),
            score: item.get("score")
        };

        beers.push(beer)
    }

    beers
}

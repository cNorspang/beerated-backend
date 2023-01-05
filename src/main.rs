#[macro_use]
extern crate rocket;

mod routes;
mod services;

use dotenv::dotenv;

use routes::user;

use routes::beer;

use routes::session::get_session_by_id;


#[get("/")]
fn index() -> &'static str {
    "Welcome to Beerated!"
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build()
        .mount("/api", routes![index])
        .mount(
            "/api/beers", 
            routes![beer::get_beer_by_id,beer::get_beers_submitting_user_id])
        .mount(
            "/api/users",
            routes![user::get_user_by_id, user::create_user, user::login_user],
        )
        .mount("/api/sessions", routes![get_session_by_id])
}

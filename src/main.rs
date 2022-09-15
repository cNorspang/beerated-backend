#[macro_use] extern crate rocket;


mod routes;
mod services;

use routes::user::get_user_by_id;
use routes::user::create_user;
use routes::user::login_user;

use routes::beer::get_beer_by_id;

use routes::session::get_session_by_id;

#[get("/")]
fn index() -> &'static str {
    "Welcome to Beerated!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![index])
        .mount("/api/beers", routes![get_beer_by_id])
        .mount("/api/users", routes![get_user_by_id, create_user, login_user])
        .mount("/api/sessions", routes![get_session_by_id])
}

use crate::routes::user::{User, UserLogin, UserResponse};
use pwhash::bcrypt;
use rocket::serde::json::Json;
use sqlx::postgres::PgPool;
use sqlx::Row;
use std::env;


pub async fn get_user_by_id(user_id: i32) -> User {
    let pool =
        PgPool::connect(env::var("PSQL_CONNECTION_STRING").expect("Missing doenv").as_str())
            .await
            .unwrap();

    let id = user_id;

    let sql_string = "SELECT email, user_id, username FROM users WHERE user_id = $1";
    let row = sqlx::query(sql_string)
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let user = User {
        name: row.get("username"),
        id: Some(row.get("user_id")),
        email: row.get("email"),
        password: None,
    };

    user
}

pub async fn create_user(user: Json<User>) -> i32 {
    let name = user.name.clone();
    let email = user.email.clone();
    let password = bcrypt::hash(user.password.clone().unwrap()).unwrap();

    let pool =
        PgPool::connect(env::var("PSQL_CONNECTION_STRING").expect("Missing doenv").as_str())
            .await
            .unwrap();

    let sql_string =
        "INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING user_id";

    let id = sqlx::query(sql_string)
        .bind(&name)
        .bind(&password)
        .bind(&email)
        .fetch_one(&pool)
        .await
        .unwrap();

    let id: i32 = id.get(0);
    id
}

pub async fn login_user(user: Json<UserLogin>) -> Option<UserResponse> {
    let email = user.email.clone();
    let password = user.password.clone();
    println!("{}", password);

    // Fix connection string
    let pool =
        PgPool::connect(env::var("PSQL_CONNECTION_STRING").expect("Missing doenv").as_str())
            .await
            .unwrap();

    let sql_string = "SELECT * FROM users WHERE email = $1";

    let row = sqlx::query(sql_string)
        .bind(email)
        .fetch_one(&pool)
        .await
        .unwrap();

    if bcrypt::verify(&password, row.get("password")) {
        let id: i32 = row.get("user_id");
        let name: String = row.get("username");
        let email: String = row.get("email");
        let status: String = "Success".to_string();
        let user = UserResponse { id, name, email,  status};

        Some(user)
    } else {
        None
    }
}

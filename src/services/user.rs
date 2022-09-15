use crate::routes::user::{User, UserLogin};
use rocket::serde::json::Json;
use sqlx::postgres::PgPool;
use sqlx::Row;
use pwhash::bcrypt;

pub async fn get_user_by_id(user_id: i32) -> User {

    let pool = PgPool::connect("postgresql://rollo:1234@192.168.0.115:5432/beerated?sslmode=disable").await.unwrap();

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


    let pool = PgPool::connect("postgresql://rollo:1234@192.168.0.115:5432/beerated?sslmode=disable").await.unwrap();

    let sql_string = "INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING user_id";

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

pub async fn login_user(user: Json<UserLogin>) -> i32 {
    let email = user.email.clone();
    let password = bcrypt::hash(user.password.clone()).unwrap();

    // Fix connection string
    let pool = PgPool::connect("postgres://rollo:ll@192.168.0.115:5432/beerated").await.unwrap();

    let sql_string = "SELECT * FROM users WHERE email = $1 AND password = $2 RETURNING user_id";

    let row = sqlx::query(sql_string)
        .bind(email)
        .bind(password)
        .fetch_one(&pool)
        .await
        .unwrap();

    let id: i32 = row.get(0);
    id
    
}

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct UserModel{
    id: i32,
    username: String,
    email: String,
    password: String,
    age: i32,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    id: i32,
    username: String,
    email: String,
    age: i32,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>
}
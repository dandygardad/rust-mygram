use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PhotoModel{
    id: i32,
    title: String,
    caption: String,
    photo_url: String,
    user_id: i32,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoResponse {
    id: i32,
    title: String,
    caption: String,
    photo_url: String,
    user_id: i32,
    created_at: Option<chrono::DateTime<Utc>>,
    updated_at: Option<chrono::DateTime<Utc>>
}
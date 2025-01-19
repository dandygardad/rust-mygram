use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PhotoModel{
    pub id: i32,
    pub title: String,
    pub caption: String,
    pub photo_url: String,
    pub user_id: i32,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PhotoResponse {
    pub id: i32,
    pub title: String,
    pub caption: String,
    pub photo_url: String,
    pub user_id: i32,
    pub created_at: Option<chrono::DateTime<Utc>>,
    pub updated_at: Option<chrono::DateTime<Utc>>
}
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// NOTE FOR ME TO LEARN
// - struct for sqlx use i8 and response use bool
// - nullable column need to use Option<T> so it can be null or T

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct SocialMediaModel {
    pub id: i32,
    pub name: String,
    pub social_media_url: String,
    pub user_id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
pub struct SocialMediaResponse {
    pub id: i32,
    pub name: String,
    pub social_media_url: String,
    pub user_id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>
}
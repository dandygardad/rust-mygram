use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// NOTE FOR ME TO LEARN
// struct for sqlx use i8 and response use bool

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct SocialMediaModel {
    id: i32,
    name: String,
    social_media_url: String,
    user_id: i32,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
pub struct SocialMediaResponse {
    id: i32,
    name: String,
    social_media_url: String,
    user_id: i32,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>
}
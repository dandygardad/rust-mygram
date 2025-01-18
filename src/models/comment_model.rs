use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CommentModel {
    id: i32,
    user_id: i32,
    photo_id: i32,
    message: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
pub struct CommentResponse {
    id: i32,
    user_id: i32,
    photo_id: i32,
    message: String,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
    deleted_at: Option<chrono::DateTime<chrono::Utc>>
}
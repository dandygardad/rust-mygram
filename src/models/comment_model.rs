use serde::{Deserialize, Serialize};

// For sqlx
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct CommentModel {
    pub id: i32,
    pub user_id: i32,
    pub photo_id: i32,
    pub message: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>
}

// For json response
#[derive(Debug, Deserialize, Serialize)]
pub struct CommentResponse {
    pub id: i32,
    pub user_id: i32,
    pub photo_id: i32,
    pub message: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>
}
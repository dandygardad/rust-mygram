use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateComment{
    pub photo_id: i32,
    pub message: String
}

pub struct UpdateComment{
    pub photo_id: i32,
    pub message: Option<String>
}

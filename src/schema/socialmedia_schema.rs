use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSocialMedia {
    name: String,
    social_media_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateSocialMedia {
    name: Option<String>,
    social_media_url: Option<String>
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreatePhoto {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    pub photo_url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdatePhoto {
    pub title: Option<String>,
    pub caption: Option<String>
}
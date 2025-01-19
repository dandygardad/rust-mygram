use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub age: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginUser{
    pub username: String,
    pub password: String
}
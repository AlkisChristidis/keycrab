use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordResponse {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct PasswordCreateResponse {
    pub domain: String,
    pub username: String,
    pub password: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterRequest {
    // Email address (maybe use a regex to validate)
    pub email: String,
    // Hash at service logic
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    // Return UUID
    pub user_id: String,
    // Success or error message (with error code)
    pub message: String,
    // JWT Token
    pub token: Option<String>,
}

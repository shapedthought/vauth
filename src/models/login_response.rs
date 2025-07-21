use serde::{Deserialize, Serialize};

/// Response structure for login requests to the Veeam REST API.
/// Contains the access token, token type, refresh token, and expiration time.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub expires_in: i32,
}

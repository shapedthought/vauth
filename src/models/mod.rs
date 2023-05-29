use serde::{Deserialize, Serialize};

/// VProfile sets the profile for the Veeam REST API.
pub enum VProfile {
    VBR,
    VB365,
    VBAWS,
    VBAZURE,
    VBGCP,
    VONE,
    ENTMAN,
}

/// VProfile can be converted from a string.
impl From<String> for VProfile {
    fn from(s: String) -> Self {
        match s.as_str() {
            "VBR" => VProfile::VBR,
            "VB365" => VProfile::VB365,
            "VBAWS" => VProfile::VBAWS,
            "VBAZURE" => VProfile::VBAZURE,
            "VBGCP" => VProfile::VBGCP,
            "VONE" => VProfile::VONE,
            "ENTMAN" => VProfile::ENTMAN,
            _ => VProfile::VBR,
        }
    }
}

/// VProfile used to authenticate to the Veeam REST API.S
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub url: String,
    pub port: String,
    pub api_version: String,
    pub x_api_version: String,
}

impl Profile {
    #[doc(hidden)]
    pub fn new(
        name: String,
        url: String,
        port: String,
        api_version: String,
        x_api_version: String,
    ) -> Self {
        Profile {
            name,
            url,
            port,
            api_version,
            x_api_version,
        }
    }

    /// Get the default profile for the Veeam REST API.
    pub fn get_profile(profile: VProfile) -> Profile {
        match profile {
            VProfile::VB365 => Profile::new(
                "VB365".to_string(),
                ":4443/v7/Token".to_string(),
                "4443".to_string(),
                "v7".to_string(),
                "".to_string(),
            ),
            VProfile::VBAWS => Profile::new(
                "VBAWS".to_string(),
                ":11005/api/v1/token".to_string(),
                "11005".to_string(),
                "v1".to_string(),
                "1.1-rev1".to_string(),
            ),
            VProfile::VBR => Profile::new(
                "VBR".to_string(),
                ":9419/api/oauth2/token".to_string(),
                "9419".to_string(),
                "v1".to_string(),
                "1.1-rev0".to_string(),
            ),
            VProfile::VBAZURE => Profile::new(
                "VBAZURE".to_string(),
                "/api/oauth2/token".to_string(),
                "".to_string(),
                "v5".to_string(),
                "".to_string(),
            ),
            VProfile::VBGCP => Profile::new(
                "VBGCP".to_string(),
                ":13140/api/v1/token".to_string(),
                "13140".to_string(),
                "v1".to_string(),
                "1.2-rev0".to_string(),
            ),
            VProfile::VONE => Profile::new(
                "VONE".to_string(),
                ":1239/api/token".to_string(),
                "1239".to_string(),
                "v2.1".to_string(),
                "".to_string(),
            ),
            VProfile::ENTMAN => Profile::new(
                "ENTMAN".to_string(),
                ":9398/api/sessionMngr/?v=latest".to_string(),
                "9398".to_string(),
                "".to_string(),
                "".to_string(),
            ),
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Creds {
    pub grant_type: String,
    pub username: String,
    pub password: String,
}

impl Creds {
    pub fn new(username: String, password: String) -> Self {
        Creds {
            grant_type: "password".to_string(),
            username,
            password,
        }
    }
}

#[doc(hidden)]
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub refresh_token: String,
    pub expires_in: i32,
}
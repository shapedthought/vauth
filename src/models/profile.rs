use serde::{Deserialize, Serialize};
use crate::{check_valid_ip, LogInError};
use super::vprofile::VProfile;

/// Profile used to authenticate to the Veeam REST API.
/// It contains the name of the profile, URL, port, API version, and X-API-Version.
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub url: String,
    pub port: String,
    pub api_version: String,
    pub x_api_version: String,
}

impl Profile {
    /// Builds the URL for the Veeam REST API based on the profile.
    /// It takes the address and end point as parameters and returns a formatted URL.
    pub fn build_url(&self, address: &String, end_point: &String) -> Result<String, LogInError> {
        if !check_valid_ip(address) {
            return Err(LogInError::IpAddressError);
        }

        match self.name.to_uppercase().as_str() {
            "VBAZURE" => Ok(format!(
                "https://{}/api/{}/{}",
                address, self.api_version, end_point
            )),
            "VBR" => Ok(format!(
                "https://{}:{}/api/{}/{}",
                address, self.port, self.api_version, end_point
            )),
            "VB365" => Ok(format!(
                "https://{}:{}/{}/{}",
                address, self.port, self.api_version, end_point
            )),
            "VBAWS" => Ok(format!(
                "https://{}:{}/api/{}/{}",
                address, self.port, self.api_version, end_point
            )),
            "VBGCP" => Ok(format!(
                "https://{}:{}/api/{}/{}",
                address, self.port, self.api_version, end_point
            )),
            "VONE" => Ok(format!(
                "https://{}:{}/api/{}/{}",
                address, self.port, self.api_version, end_point
            )),
            "ENTMAN" => Ok(format!(
                "https://{}:{}/api/{}",
                address, self.port, end_point
            )),
            _ => Err(LogInError::IpAddressError),
        }
    }

    #[deprecated(since = "0.1.0", note = "Use VProfile::<enum>.profile_data() instead")]
    /// Returns the profile data for the given VProfile.
    /// This method is deprecated and will be removed in future versions.
    #[allow(deprecated)]
    pub fn get_profile(v_profile: VProfile) -> Self {
        v_profile.profile_data()
    }

    /// Builds the authentication headers for the reqwest client.
    /// It takes a token as a parameter and returns a HeaderMap with the necessary headers.
    pub fn build_auth_headers(&self, token: &String) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::ACCEPT, "application/json".parse().unwrap());
        headers.insert(reqwest::header::CONTENT_TYPE, "application/json".parse().unwrap());

        if self.name == "ENTMAN" {
            headers.insert("X-RestSvcSessionId", token.parse().unwrap());
        } else {
            let bearer = format!("Bearer {}", token);
            headers.insert("Authorization", bearer.parse().unwrap());
            headers.insert("X-Api-Version", self.x_api_version.parse().unwrap());
        }

        headers
    }
}
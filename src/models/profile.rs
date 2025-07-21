use super::vprofile::VProfile;
use crate::{check_valid_ip, LogInError, LoginResponse};
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

/// Enum representing different profile types for Veeam REST API.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProfileType {
    VBAZURE,
    VBR,
    VBAWS,
    VBGCP,
    VONE,
    VB365,
    ENTMAN,
    UNKNOWN,
}

/// Profile used to authenticate to the Veeam REST API.
/// It contains the name of the profile, URL, port, API version, and X-API-Version.
#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub profile_type: ProfileType,
    pub name: String,
    pub url: String,
    pub port: String,
    pub api_version: String,
    pub x_api_version: Option<String>,
}

impl Profile {
    /// Creates a new Profile instance.
    /// This method initializes a Profile with the given parameters.
    pub fn new(
        name: String,
        url: String,
        port: String,
        api_version: String,
        x_api_version: Option<String>,
    ) -> Self {
        Profile {
            profile_type: ProfileType::UNKNOWN,
            name,
            url,
            port,
            api_version,
            x_api_version,
        }
    }

    /// Builds the URL for the Veeam REST API based on the profile.
    /// It takes the address and end point as parameters and returns a formatted URL.
    pub fn build_url(&self, address: &String, end_point: &String) -> Result<String, LogInError> {
        if !check_valid_ip(address) {
            return Err(LogInError::IpAddressError);
        }

        match self.profile_type {
            ProfileType::VBAZURE => Ok(format!(
                "https://{}/api/{}/{}",
                address, self.api_version, end_point
            )),
            ProfileType::VBR | ProfileType::VBAWS | ProfileType::VBGCP | ProfileType::VONE => Ok(format!(
                "https://{}:{}/api/{}/{}",
                address, self.port, self.api_version, end_point
            )),
            ProfileType::VB365 => Ok(format!(
                "https://{}:{}/{}/{}",
                address, self.port, self.api_version, end_point
            )),
            ProfileType::ENTMAN => Ok(format!(
                "https://{}:{}/api/{}",
                address, self.port, end_point
            )),
            ProfileType::UNKNOWN => Err(LogInError::OtherError(
                "Unknown profile type, manual endpoint construction required".to_string(),
            ))
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
    /// It takes a token as a parameter and returns a Result<HeaderMap> with the necessary headers.
    /// Note that this is a breaking change from the previous version.
    pub fn build_auth_headers(
        &self,
        token: &String,
    ) -> Result<reqwest::header::HeaderMap, reqwest::header::InvalidHeaderValue> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        if self.profile_type == ProfileType::ENTMAN {
            headers.insert("X-RestSvcSessionId", HeaderValue::from_str(token)?);
        } else {
            let bearer = format!("Bearer {}", token);
            headers.insert("Authorization", HeaderValue::from_str(&bearer)?);
            if let Some(x_api_version) = &self.x_api_version {
                headers.insert("X-Api-Version", HeaderValue::from_str(x_api_version)?);
            }
        }

        Ok(headers)
    }

    /// Builds the authentication headers using a login response.
    /// It takes a LoginResponse as a parameter and returns a Result<HeaderMap> with the necessary headers.
    /// Note that this is a breaking change from the previous version.
    pub fn build_auth_headers_from_response(
        &self,
        login_response: &LoginResponse,
    ) -> Result<reqwest::header::HeaderMap, reqwest::header::InvalidHeaderValue> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );

        if self.name == "ENTMAN" {
            headers.insert(
                "X-RestSvcSessionId",
                HeaderValue::from_str(&login_response.access_token)?,
            );
        } else {
            let bearer = format!("Bearer {}", login_response.access_token);
            headers.insert("Authorization", bearer.parse()?);
            if let Some(x_api_version) = &self.x_api_version {
                headers.insert("X-Api-Version", HeaderValue::from_str(x_api_version)?);
            }
        }

        Ok(headers)
    }
}

use reqwest::header::{HeaderMap, ACCEPT, CONTENT_TYPE};
use crate::models::profile::Profile;
use super::error::LogInError;

pub fn check_valid_ip(address: &str) -> bool {
    address.parse::<std::net::IpAddr>().is_ok()
}

/// Helper function to build the url for the reqwest client
///
/// # Arguments
///
/// * `address` - The IP address of the Veeam server
/// * `end_point` - The API endpoint to be called, but not including the API version e.g. /api/v1/backups > backups
/// * `profile` - The profile to be used for the request
pub fn build_url(
    address: &String,
    end_point: &String,
    profile: &Profile,
) -> Result<String, LogInError> {
    return profile
        .build_url(address, end_point)
        .map_err(|_| LogInError::IpAddressError);
}

/// Helper function to build Auth Headers, this is useful for when you still have a valid token
/// and want to make a request without having to create a new client.
/// # Arguments
/// * `token` - The access token to be used for authentication
/// * `profile` - The profile to be used for the request
/// # Returns
/// A HeaderMap containing the necessary headers for authentication
pub fn build_auth_headers(token: &String, profile: &Profile) -> HeaderMap {
    let mut headermap = HeaderMap::new();
    headermap.insert(ACCEPT, "application/json".parse().unwrap());
    headermap.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    if profile.name == *"ENTMAN" {
        headermap.insert("X-RestSvcSessionId", token.parse().unwrap());
    } else {
        let bearer = format!("Bearer {}", token);
        headermap.insert("Authorization", bearer.parse().unwrap());
        headermap.insert("X-Api-Version", profile.x_api_version.parse().unwrap());
    }
    headermap
}
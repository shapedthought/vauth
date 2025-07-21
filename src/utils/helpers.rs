use super::error::LogInError;
use crate::models::profile::Profile;
use reqwest::header::HeaderMap;

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
    return profile.build_url(address, end_point)
}

/// Helper function to build Auth Headers, this is useful for when you still have a valid token
/// and want to make a request without having to create a new client.
/// # Arguments
/// * `token` - The access token to be used for authentication
/// * `profile` - The profile to be used for the request
/// # Returns
/// A HeaderMap containing the necessary headers for authentication
pub fn build_auth_headers(token: &String, profile: &Profile) -> Result<HeaderMap, reqwest::header::InvalidHeaderValue> {
    return profile.build_auth_headers(token)
}

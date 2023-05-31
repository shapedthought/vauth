//! # VAuth - Veeam Authentication Library
//!
//! _Note that this library is unofficial and not endorsed or supported by Veeam_
//! 
//! This library is used to authenticate to Veeam Backup Product REST APIs.
//! It supports authentication to Veeam Backup & Replication, Veeam Backup for Microsoft Office 365, VONE and the Veeam Cloud Backup Products (AWS, AZURE & GCP).
//! 
//! The library is designed as a wrapper around the reqwest library and provides a simple interface to authenticate to the Veeam REST APIs.
//! 
//! The library uses the builder pattern to create a reqwest client with the required authentication headers.
//! 
//! Note that the library requires the VEEAM_API_PASSWORD environmental variable to be set. This is the password that is used to authenticate to the Veeam REST API.
//! 
//! This library is not intended to be a full featured library for the Veeam REST APIs, and there is no intention to turn it into one.
//! 
//! ## Usage
//! 
//! ```rust
//! use vauth::{VServerBuilder, Profile, VProfile, build_url, LoginResponse};
//! use serde_json::Value;
//! use reqwest::Client;
//! use anyhow::Result;
//! use std::env;
//! 
//! #[tokio::main]
//! async fn  main() -> Result<()> {
//!     let mut profile = Profile::get_profile(VProfile::ENTMAN);
//! 
//!     // Used for testing
//!     dotenvy::dotenv()?;
//! 
//!     let username = env::var("VEEAM_API_USERNAME").unwrap();
//! 
//!     let address = env::var("VEEAM_API_ADDRESS").unwrap();
//! 
//!     let (client, _login_response) = VServerBuilder::new(&address, username)
//!         .insecure()
//!         .build(&mut profile)
//!         .await?;
//! 
//!     let endpoint = build_url(&address, &"backups".to_string(), &profile)?;
//! 
//!     let response: Value = client.get(&endpoint).send().await?.json().await?;
//! 
//!     println!("{:#?}", response);
//! 
//!     Ok(())
//! }
//! ```
//! 
//! When the build method is called, the library will attempt to authenticate to the Veeam REST API and return a reqwest client.
//! 
//! You can then use the reqwest client to make requests to the Veeam REST API.
//! 
//! There is also a helper function to build the URL for the Veeam REST API.
//! 
//! Note that the library uses async/await and therefore requires the use of the tokio runtime.
//! 
//! ## Default Profiles
//! 
//! The library has default profiles for each API which I will try to keep up to date.
//! 
//! The default profiles are:
//! 
//! | Profile            | Port  | API Version | X-API Version |
//! | ------------------ | ----- | ----------- | ------------- |
//! | VBR                | 9419  | v1          | 1.1-rev1      |
//! | Enterprise Manager | 9398  | -           | -             |
//! | VB365              | 4443  | v7          | -             |
//! | VBAWS              | 11005 | v1          | 1.1-rev1      |
//! | VBGCP              | 13140 | v1          | 1.2-rev0      |
//! | VBAZURE            | -     | v5          | -             |
//! | VONE               | 1239  | v2.1        | -             |
//! 
//! Last updated: 30/05/2023
//! 
//! You can modify the defaults using the available methods before building the client.
//! 
//! ```no run
//! let client: Client = VServerBuilder::new(&address, username)
//!     .insecure()
//!     .port("1234".to_string())
//!     .api_version("v2".to_string())
//!     .x_api_version("2.0-rev0".to_string())
//! 
//! ```
//! 
//! ## Creating a Custom Profile
//! 
//! You can create a custom profile by using the Profile struct.
//! 
//! ```no run
//! let mut profile = Profile::new(
//!     "NEW_PROFILE",
//!     ":1234/api/oauth2/token'",
//!     "v1",
//!     "1.0-rev0",
//! );
//! ```
//! 
//! The Profile struct has the following fields:
//! 
//! | Field         | Description                                                                                          |
//! | ------------- | ---------------------------------------------------------------------------------------------------- |
//! | name          | The name of the profile. This is used to identify the profile.                                       |
//! | url           | The URL used to authenticate which must have the colon at the start                                  |
//! | api_version   | The API version, this is used to construct the URLs e.g. http://address:port/api/API_VERSION/...     |
//! | x_api_version | This is the X-API-Version header value.                                                              |
//! 
//! This can then be passed to the build method.
//! 
//! ## Build URL
//! 
//! The library provides a helper function to build the URL for the Veeam REST API.
//! 
//! The function takes an endpoint parameter which is a shortened version of the URL.
//! 
//! For example, to get a list of backups from VBR, the endpoint would be "backups", normally the full URL would be:
//! 
//! ```no run
//! https://<address>:<port>/api/v1/backups
//! ```
//! 
//! You can use the helper function to build the URL:
//! 
//! ```no run
//! let endpoint = build_url(&address, &"backups".to_string(), &profile)?;
//! ```
//! 
//! ## Authentication
//! 
//! The library uses OAuth2 to authenticate to all the APIs except Enterprise Manager which uses Basic Authentication.
//! 
//! See Veeam's documentation for more information on the authentication process.

mod models;
pub use models::*;
use reqwest::{
    header::{HeaderMap, ACCEPT, CONTENT_LENGTH, CONTENT_TYPE},
};
// use serde_urlencoded;
use std::{env, time::Duration, net::IpAddr, str::FromStr};
use thiserror::Error;
use regex::Regex;

/// Returns a reqwest client with the required authentication headers.
pub struct VServerBuilder {
    address: String,
    username: String,
    insecure: Option<bool>,
    timeout: Option<u64>,
    api_version: Option<String>,
    x_api_version: Option<String>,
    port: Option<String>,
}


/// LogInError is used to return errors from the build method.
#[derive(Error, Debug)]
pub enum LogInError {
    #[error("The VEEAM_API_PASSWORD environmental variable is missing")]
    EnvError(#[from] env::VarError),
    #[error("IP Address is not valid")]
    IpAddressError,
    #[error("Username cannot be empty")]
    UsernameEmpty,
    #[error("Password cannot be empty")]
    PasswordEmpty,
    #[error("IP Address cannot be empty")]
    IpAddressEmpty,
    #[error("No refresh token")]
    NoRefreshToken,
    #[error("Error in sending request `{0:?}`")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Status Code Error `{0}`")]
    StatusCodeError(reqwest::StatusCode),
}

#[doc(hidden)]
pub fn check_valid_ip(address: &str) -> bool {
    IpAddr::from_str(address).is_ok()
}

impl VServerBuilder {

    /// Create a new VServerBuilder
    pub fn new(address: &String, username: String) -> Self {
        VServerBuilder {
            address: address.to_string(),
            username,
            insecure: None,
            timeout: None,
            api_version: None,
            x_api_version: None,
            port: None,
        }
    }
    /// Set the Client to use insecure connections
    pub fn insecure(&mut self) -> &mut Self {
        self.insecure = Some(true);
        self
    }

    /// Manually set the timeout for the client; default is 30 seconds
    pub fn timeout(&mut self, value: u64) -> &mut Self {
        self.timeout = Some(value);
        self
    }

    /// Manually set the API version for the client, e.g v1, v2, v3
    pub fn api_version(&mut self, value: String) -> &mut Self {
        self.api_version = Some(value);
        self
    }

    /// Manually set the X-API-Version for the client, e.g 1.1-rev0, 1.2-rev0
    pub fn x_api_version(&mut self, value: String) -> &mut Self {
        self.x_api_version = Some(value);
        self
    }

    /// Manually set the port for the client, e.g 1234
    pub fn port(&mut self, value: String) -> &mut Self {
        self.port = Some(value);
        self
    }

    /// Build the reqwest client, this takes a mutable reference to a Profile and will attempt to authenticate to the Veeam REST API.
    /// It will return a tuple with both the client and the login response struct. 
    /// The login response struct contains the token and refresh token which you can save for
    /// future use. 
    pub async fn build(&mut self, profile: &mut Profile) -> Result<(reqwest::Client, LoginResponse), LogInError> {
        if self.username.is_empty() {
            return Err(LogInError::UsernameEmpty);
        }

        let api_pass_res = env::var("VEEAM_API_PASSWORD");

        let api_pass = match api_pass_res {
            Ok(r) => r,
            Err(e) => return Err(LogInError::EnvError(e)),
        };

        if api_pass.is_empty() {
            return Err(LogInError::PasswordEmpty);
        }

        if self.address.is_empty() {
            return Err(LogInError::IpAddressEmpty);
        }

        if !check_valid_ip(&self.address) {
            return Err(LogInError::IpAddressError);
        }

        if let Some(api_version) = &self.api_version {
            let re = Regex::new("v[0-9]").unwrap();
            re.replace(&profile.url, api_version);
            profile.api_version = api_version.to_string();
        }

        if let Some(x_api_version) = &self.x_api_version {
            profile.x_api_version = x_api_version.to_string();
        }

        if let Some(port) = &self.port {
            let re = Regex::new("[0-9]{2,}").unwrap();
            re.replace(&profile.url, port.as_str());
            profile.port = port.to_string();
        }

        let insecure = self.insecure.unwrap_or(false);
        let timeout_val = self.timeout.unwrap_or(30);

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_val))
            .danger_accept_invalid_certs(insecure)
            .build()?;

        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        let auth_url = format!("https://{}{}", self.address, profile.url);

        let response: reqwest::Response = if profile.name != "ENTMAN" {
            let creds = Creds::new(self.username.clone(), api_pass);
            let creds_urlenc = serde_urlencoded::to_string(&creds).unwrap();
            headers.insert("X-Api-Version", profile.x_api_version.parse().unwrap());
            headers.insert(
                CONTENT_TYPE,
                "application/x-www-form-urlencoded".parse().unwrap(),
            );
            client
                .post(auth_url)
                .body(creds_urlenc)
                .headers(headers)
                .send()
                .await?
        } else {
            headers.insert(CONTENT_LENGTH, "0".parse().unwrap());
            client
                .post(auth_url)
                .basic_auth(self.username.clone(), Some(api_pass))
                .headers(headers)
                .send()
                .await?
        };

        let res_data: LoginResponse;

        if response.status().is_success() {
            if profile.name == "ENTMAN" {
                let token = response
                    .headers()
                    .get("X-RestSvcSessionId")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                res_data = LoginResponse {
                    access_token: token.clone(),
                    refresh_token: token,
                    expires_in: 900,
                    token_type: String::from(""),
                }
            } else {
                res_data = response.json().await?
            }
        } else {
            return Err(LogInError::StatusCodeError(response.status()));
        }


        let bearer: String = if profile.name != *"ENTMAN" {
            format!("Bearer {}", res_data.access_token.as_str().trim())
        } else {
            res_data.access_token.as_str().trim().to_owned()
        };

        let mut req_header = HeaderMap::new();
        req_header.insert(ACCEPT, "application/json".parse().unwrap());
        req_header.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        if profile.name == *"ENTMAN" {
            req_header.insert("X-RestSvcSessionId", bearer.parse().unwrap());
        } else {
            req_header.insert("Authorization", bearer.parse().unwrap());
            req_header.insert("X-Api-Version", profile.x_api_version.parse().unwrap());
        }

        let req_builder = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_val))
            .danger_accept_invalid_certs(insecure)
            .default_headers(req_header)
            .build()?;

        Ok((req_builder, res_data))
    }

}

/// Helper function to build the url for the reqwest client
/// 
/// # Arguments
/// 
/// * `address` - The IP address of the Veeam server
/// * `end_point` - The API endpoint to be called, but not including the API version e.g. /api/v1/backups > backups
/// * `profile` - The profile to be used for the request
pub fn build_url(address: &String, end_point: &String, profile: &Profile) -> Result<String, LogInError> {

    if !check_valid_ip(address) {
        return Err(LogInError::IpAddressError);
    }

    match profile.name.to_uppercase().as_str() {
        "VBAZURE" => Ok(format!(
            "https://{}/api/{}/{}",
            address, profile.api_version, end_point
        )),
        "VBR" => Ok(format!(
            "https://{}:{}/api/{}/{}",
            address, profile.port, profile.api_version, end_point
        )),
        "VB365" => Ok(format!(
            "https://{}:{}/{}/{}",
            address, profile.port, profile.api_version, end_point
        )),
        "VBAWS" => Ok(format!(
            "https://{}/api/{}/{}",
            address, profile.api_version, end_point
        )),
        "VBGCP" => Ok(format!(
            "https://{}/api/{}/{}",
            address, profile.api_version, end_point
        )),
        "VONE" => Ok(format!(
            "https://{}:{}/api/{}/{}",
            address, profile.port, profile.api_version, end_point
        )),
        "ENTMAN" => Ok(format!(
            "https://{}:{}/api/{}",
            address, profile.port, end_point
        )),
        _ => Ok(format!(
            "https://{}:{}{}/{}",
            address, profile.port, profile.url, end_point
        )),
    }

}

/// Helper function to build Auth Headers, this is useful for when you still have a valid token
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_build_url() {
        let address = String::from("192.168.0.123");
        let end_point = String::from("backups");
        let vprofile = VProfile::VBR;
        let profile = Profile::get_profile(vprofile);
        let url = build_url(&address, &end_point, &profile).unwrap();
        assert_eq!(url, "https://192.168.0.123:9419/api/v1/backups");
    }

    #[test]
    fn test_build_profile() {
        let vprofile = VProfile::VBR;
        let profile = Profile::get_profile(vprofile);
        assert!(profile.name == "VBR"); 
        assert!(profile.port == "9419");
        assert!(profile.url == ":9419/api/oauth2/token");
        assert!(profile.api_version == "v1"); 
        assert!(profile.x_api_version == "1.1-rev0");
    }

    #[tokio::test]
    async fn test_entman_with_request() {
        dotenvy::dotenv().unwrap();
        let mut profile = Profile::get_profile(VProfile::ENTMAN);
        let username = env::var("VEEAM_API_USERNAME").unwrap();
        let address = env::var("VEEAM_API_ADDRESS").unwrap();

        let (client, _res) = VServerBuilder::new(&address, username).insecure().build(&mut profile).await.unwrap();

        let url = build_url(&address, &String::from("jobs"), &profile).unwrap();

        let response = client.get(&url).send().await.unwrap();

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_vbr_with_request() {
        dotenvy::dotenv().unwrap();
        let mut profile = Profile::get_profile(VProfile::VBR);
        let username = env::var("VEEAM_API_USERNAME").unwrap();
        let address = env::var("VEEAM_API_ADDRESS").unwrap();

        let (client, _res) = VServerBuilder ::new(&address, username).insecure().build(&mut profile).await.unwrap();

        let url = build_url(&address, &String::from("jobs"), &profile).unwrap();

        let response = client.get(&url).send().await.unwrap();

        assert!(response.status().is_success());
    }

    #[tokio::test]
    async fn test_vb365_with_request() {
        dotenvy::dotenv().unwrap();
        let mut profile = Profile::get_profile(VProfile::VB365);
        let username = env::var("VEEAM_API_USERNAME").unwrap();
        let address = env::var("VB365_API_ADDRESS").unwrap();

        let mut vserver = VServerBuilder::new(&address, username);

        let (client, _res) = vserver.insecure().build(&mut profile).await.unwrap();

        let url = build_url(&address, &String::from("Jobs"), &profile).unwrap();

        let response = client.get(&url).send().await.unwrap();

        assert!(response.status().is_success());
    }
}

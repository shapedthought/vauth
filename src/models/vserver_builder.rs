use std::{env, time::Duration};
use regex::Regex;
use reqwest::header::{HeaderMap, ACCEPT, CONTENT_LENGTH, CONTENT_TYPE};

use crate::{check_valid_ip, Creds, LogInError};

use super::{LoginResponse, Profile};


/// The `VServerBuilder` struct is used to build a reqwest client for Veeam REST API authentication.
/// This struct is deprecated and will be removed in future versions. Use `VClientBuilder` instead.
#[deprecated(
    since = "1.0.0",
    note = "Use VClientBuilder instead. VServerBuilder will be removed in future versions."
)]

pub struct VServerBuilder {
    address: String,
    username: String,
    insecure: Option<bool>,
    timeout: Option<u64>,
    api_version: Option<String>,
    x_api_version: Option<String>,
    port: Option<String>,
}

/// The `VServerBuilder` struct provides methods to configure the client, such as setting the API version, timeout, and whether to use insecure connections.
#[allow(deprecated)]
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
    pub async fn build(
        &mut self,
        profile: &mut Profile,
    ) -> Result<(reqwest::Client, LoginResponse), LogInError> {
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
            let creds = Creds::new(&self.username, &api_pass);
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
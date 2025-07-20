use std::env;

use thiserror::Error;

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
    #[error("Other Error `{0}`")]
    OtherError(String),
    #[error("Anyhow Error `{0}`")]
    AnyhowError(#[from] anyhow::Error),
}
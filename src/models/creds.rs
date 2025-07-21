use serde::{Deserialize, Serialize};

/// Struct representing credentials for authentication
/// with a username and password.
#[derive(Debug, Serialize, Deserialize)]
pub struct Creds<'a> {
    pub grant_type: &'static str,
    pub username: &'a str,
    pub password: &'a str,
}

/// Default implementation for the `Creds` struct.
impl <'a> Default for Creds<'a> {
    fn default() -> Self {
        Creds {
            grant_type: "password",
            username: "",
            password: "",
        }
    }
}

/// Implementation of methods for the `Creds` struct.
impl<'a> Creds<'a> {
    pub fn new(username: &'a str, password: &'a str) -> Self {
        Creds {
            grant_type: "password",
            username,
            password,
        }
    }
}
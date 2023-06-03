# VAuth - Veeam Authentication Library

_Note that this library is unofficial and not endorsed or supported by Veeam_

This library is used to authenticate to Veeam Backup Product REST APIs.
It supports authentication to Veeam Backup & Replication, Veeam Backup for Microsoft Office 365, VONE and the Veeam Cloud Backup Products (AWS, AZURE & GCP).

The library is designed as a wrapper around the reqwest library and provides a simple interface to authenticate to the Veeam REST APIs.

The library uses the builder pattern to create a reqwest client with the required authentication headers.

Note that the library requires the VEEAM_API_PASSWORD environmental variable to be set. This is the password that is used to authenticate to the Veeam REST API.

 This library is not intended to be a full featured library for the Veeam REST APIs, and there is no intention to turn it into one.

## Installation

Run the following command to add the library to your project:

```
cargo add vauth
```
## Usage

Login with direct use of the client.

```rust
use vauth::{VServerBuilder, Profile, VProfile, build_url, LoginResponse};
use serde_json::Value;
use reqwest::Client;
use anyhow::Result;
use std::env;
use std::fs::{self, File};
use std::io::Write;

#[tokio::main]
async fn  main() -> Result<()> {
    let mut profile = Profile::get_profile(VProfile::VB365);

     // Used for testing
    dotenvy::dotenv()?;

    let username = env::var("VEEAM_API_USERNAME").unwrap();

    let address = env::var("VB365_API_ADDRESS").unwrap();

    let (client, login_response) = VServerBuilder::new(&address, username)
        .insecure()
        .build(&mut profile)
        .await?;

    // Save the token for later use
    let mut json_file = fs::File::create(&"token.json".to_string())?;
    let token_string = serde_json::to_string_pretty(&login_response)?;
    json_file.write_all(token_string.as_bytes())?;

    let endpoint = build_url(&address, &"Jobs".to_string(), &profile)?;

    let response: Value = client.get(&endpoint).send().await?.json().await?;

    println!("{:#?}", response);

    Ok(())
}
 ```

When the build method is called, the library will attempt to authenticate to the Veeam REST API and return the reqwest client around
the authentication response struct as a tuple.

You can then use the reqwest client to make requests to the Veeam REST API.

There are helper functions to both build the URL for the Veeam REST API as well as use an
existing toke to build authented headers.

Note that the library uses async/await and therefore requires the use of the tokio runtime.

### Reusing a saved response struct.

```rust
use vauth::{Profile, VProfile, build_url, LoginResponse, build_auth_headers};
use serde_json::Value;
use reqwest::Client;
use anyhow::Result;
use std::env;
use std::fs;

#[tokio::main]
async fn  main() -> Result<()> {
    let mut profile = Profile::get_profile(VProfile::VB365);

    // Used for testing
    dotenvy::dotenv()?;
    let address = env::var("VB365_API_ADDRESS")?;
    let token_path = env::var("TOKEN_PATH")?;
    
    let login_response: LoginResponse = {
        let data = fs::read_to_string(token_path)?;
        serde_json::from_str(&data)?
    };

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;

    let headers = build_auth_headers(&login_response.access_token, &profile);

    let endpoint = build_url(&address, &"Jobs".to_string(), &profile)?;

    let response: Value = client.get(&endpoint).headers(headers).send().await?.json().await?;

    println!("{:#?}", response);

    Ok(())
}
```

## Default Profiles

The library has default profiles for each API which I will try to keep up to date.

The default profiles are:

| Profile            | Port  | API Version | X-API Version |
| ------------------ | ----- | ----------- | ------------- |
| VBR                | 9419  | v1          | 1.1-rev1      |
| Enterprise Manager | 9398  | -           | -             |
| VB365              | 4443  | v7          | -             |
| VBAWS              | 11005 | v1          | 1.1-rev1      |
| VBGCP              | 13140 | v1          | 1.2-rev0      |
| VBAZURE            | -     | v5          | -             |
| VONE               | 1239  | v2.1        | -             |

Last updated: 30/05/2023

You can modify the defaults using the available methods before building the client.

```no run
let client: Client = VServerBuilder::new(&address, username)
    .insecure()
    .port("1234".to_string())
    .api_version("v2".to_string())
    .x_api_version("2.0-rev0".to_string())

```

## Creating a Custom Profile

You can create a custom profile by using the Profile struct.

```no run
let mut profile = Profile::new(
    "NEW_PROFILE",
    ":1234/api/oauth2/token'",
    "v1",
    "1.0-rev0",
);
```

The Profile struct has the following fields:

| Field         | Description                                                                                          |
| ------------- | ---------------------------------------------------------------------------------------------------- |
| name          | The name of the profile. This is used to identify the profile.                                       |
| url           | The URL used to authenticate which must have the colon at the start                                  |
| api_version   | The API version, this is used to construct the URLs e.g. http://address:port/api/API_VERSION/...     |
| x_api_version | This is the X-API-Version header value.                                                              |

This can then be passed to the build method.

## Build URL

The library provides a helper function to build the URL for the Veeam REST API.

The function takes an endpoint parameter which is a shortened version of the URL.

For example, to get a list of backups from VBR, the endpoint would be "backups", normally the full URL would be:

```no run
https://<address>:<port>/api/v1/backups
```

You can use the helper function to build the URL:

```no run
let endpoint = build_url(&address, &"backups".to_string(), &profile)?;
```

## Build Authentication Headers

The library also provides a helper function to build the authentication headers from the saved
response struct.

```no run
let headers = build_auth_headers(&access_token, &profile);
```

This can then be used directly with a reqwest client.

## Authentication

The library uses OAuth2 to authenticate to all the APIs except Enterprise Manager which uses Basic Authentication.

See Veeam's documentation for more information on the authentication process.


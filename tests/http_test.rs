use std::env;
use std::fs::{self, File};
use std::io::Write;
use vauth::models::login_response::LoginResponse;
use vauth::models::vprofile::VProfile;
use vauth::VClientBuilder;

#[tokio::test]
async fn test_entman_with_request() {
    dotenvy::dotenv().unwrap();
    let mut profile = VProfile::ENTMAN.profile_data();
    let username = env::var("VEEAM_API_USERNAME").unwrap();
    let address = env::var("VEEAM_API_ADDRESS").unwrap();

    let (client, _res) = VClientBuilder::new(&address, &username)
        .insecure()
        .build(&mut profile)
        .await
        .unwrap();

    let url = profile.build_url(&address, &String::from("jobs")).unwrap();

    let response = client.get(&url).send().await.unwrap();

    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_vbr_with_request() {
    dotenvy::dotenv().unwrap();
    let mut profile = VProfile::VBR.profile_data();
    let username = env::var("VEEAM_API_USERNAME").unwrap();
    let address = env::var("VEEAM_API_ADDRESS").unwrap();

    let (client, _res) = VClientBuilder::new(&address, &username)
        .insecure()
        .build(&mut profile)
        .await
        .unwrap();

    let url = profile.build_url(&address, &String::from("jobs")).unwrap();

    let auth_headers = profile.build_auth_headers_from_response(&_res).unwrap();

    let response = client.get(&url).headers(auth_headers).send().await.unwrap();

    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_vb365_with_request() {
    dotenvy::dotenv().unwrap();
    let mut profile = VProfile::VB365.profile_data();
    let username = env::var("VEEAM_API_USERNAME").unwrap();
    let address = env::var("VB365_API_ADDRESS").unwrap();

    let mut vclient = VClientBuilder::new(&address, &username);

    let (client, res) = vclient.insecure().build(&mut profile).await.unwrap();

    let mut json_file = File::create(&"token.json".to_string()).unwrap();
    let token_string = serde_json::to_string_pretty(&res).unwrap();
    json_file.write_all(token_string.as_bytes()).unwrap();

    let url = profile.build_url(&address, &String::from("Jobs")).unwrap();

    let auth_headers = profile.build_auth_headers(&res.access_token).unwrap();

    let response = client.get(&url).headers(auth_headers).send().await.unwrap();

    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_vb365_use_token() {
    dotenvy::dotenv().unwrap();
    let profile = VProfile::VB365.profile_data();

    dotenvy::dotenv().unwrap();

    let address = env::var("VB365_API_ADDRESS").unwrap();
    let token_path = env::var("TOKEN_PATH").unwrap();

    let login_response: LoginResponse = {
        let data = fs::read_to_string(token_path).unwrap();
        serde_json::from_str(&data).unwrap()
    };

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let url = profile.build_url(&address, &String::from("Jobs")).unwrap();

    let headers = profile
        .build_auth_headers(&login_response.access_token)
        .unwrap();

    let response = client.get(&url).headers(headers).send().await.unwrap();

    assert!(response.status().is_success());
}

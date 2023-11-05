// This module is a rather early implementation!!!
// That means you can not use this module!
// All of errors will be handled at frontend.

use std::collections::HashMap;

use reqwest::{header::*, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// These constants are URLs that will receive POST (some are GET) requests and response data.
const FETCH_MICROSOFT_OAUTH2_TOKEN: &str = "https://login.live.com/oauth20_token.srf";
const XBOX_USER_AUTHENTICATE: &str = "https://user.auth.xboxlive.com/user/authenticate";
const XSTS_USER_AUTHORIZE: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
const FETCH_MINECRAFT_ACCESS_TOKEN: &str =
    "https://api.minecraftservices.com/authentication/login_with_xbox";
const CHECK_IF_PLAYER_OWN_MINECRAFT: &str =
    "https://api.minecraftservices.com/entitlements/mcstore";
const FETCH_MINECRAFT_UUID_AND_USERNAME: &str =
    "https://api.minecraftservices.com/minecraft/profile";

#[derive(Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MinecraftProfile {
    // The Minecraft Access Token.
    pub access_token: String,
    // The UUID which is frequently used to verify a player's identity.
    pub uuid: String,
    // The username which will be displayed in the game.
    pub username: String,
}

pub async fn request_microsoft_oauth2_token(
    authorization_code: &str,
) -> Result<String, reqwest::Error> {
    // The request header.
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );

    // The parameters.
    let mut map = HashMap::new();
    map.insert("client_id", "00000000402b5328");
    map.insert("code", authorization_code);
    map.insert("grant_type", "authorization_code");
    map.insert("redirect_uri", "https://login.live.com/oauth20_desktop.srf");
    map.insert("scope", "service::user.auth.xboxlive.com::MBI_SSL");

    let client = Client::new();
    client
        .post(FETCH_MICROSOFT_OAUTH2_TOKEN)
        .headers(headers)
        .form(&map)
        .send()
        .await?
        .text()
        .await
}

pub async fn request_xbox_authentication(access_token: &str) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let map = json!(
        {
        "Properties": {
            "AuthMethod": "RPS",
            "SiteName": "user.auth.xboxlive.com",
            "RpsTicket": access_token,
        },
        "RelyingParty": "http://auth.xboxlive.com",
        "TokenType": "JWT"
        }
    );

    println!("{:#?}", map);

    send_request(Some(headers), Some(map), XBOX_USER_AUTHENTICATE).await
}

#[warn(non_snake_case)]
pub async fn request_xsts_authorization(xbox_token: &str) -> Result<String, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

    let map = json!(
        {
        "Properties": {
            "SandboxId": "RETAIL",
            "UserTokens": [
                xbox_token,
            ]
        },
        "RelyingParty": "rp://api.minecraftservices.com/",
        "TokenType": "JWT"
        }
    );

    println!("{:#?}", map);

    send_request(Some(headers), Some(map), XSTS_USER_AUTHORIZE).await
}

impl MinecraftProfile {
    pub fn new() -> Self {
        Self {
            access_token: String::new(),
            uuid: String::new(),
            username: String::new(),
        }
    }

    pub async fn request_minecraft_access_token(
        &mut self,
        xsts_token: &str,
        uhs: &str,
    ) -> Result<String, reqwest::Error> {
        let map = json!(
            {
                "identityToken": format!("XBL3.0 x={};{}", uhs, xsts_token)
            }
        );

        send_request(None, Some(map), FETCH_MINECRAFT_ACCESS_TOKEN).await
    }

    pub async fn check_if_player_own_minecraft(&self) -> Result<(), reqwest::Error> {
        let client = Client::new();

        let _response = client
            .get(CHECK_IF_PLAYER_OWN_MINECRAFT)
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .text()
            .await?;
        Ok(())
    }

    pub async fn request_minecraft_uuid_and_username(&self) -> Result<String, reqwest::Error> {
        let client = Client::new();

        let response = client
            .get(FETCH_MINECRAFT_UUID_AND_USERNAME)
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .text()
            .await?;
        Ok(response)
    }
}

// Send requests and return results.
pub async fn send_request<T: Serialize>(
    headers: Option<HeaderMap>,
    map: Option<T>,
    target_link: &str,
) -> Result<String, reqwest::Error> {
    let client = Client::new();

    match (headers, map) {
        (Some(headers), None) => {
            client
                .post(target_link)
                .headers(headers)
                .send()
                .await?
                .text()
                .await
        }
        (None, Some(map)) => {
            client
                .post(target_link)
                .json(&map)
                .send()
                .await?
                .text()
                .await
        }
        (Some(headers), Some(map)) => {
            client
                .post(target_link)
                .headers(headers)
                .json(&map)
                .send()
                .await?
                .text()
                .await
        }
        _ => unreachable!(),
    }
}

// Transfer responses into JSON, fetch necessary fields and store them in the instance of structure.
// Use turbofish syntax to simplify data processing.
pub fn parse_response(response: &str) -> Result<Value, serde_json::Error> {
    serde_json::from_str::<Value>(response)
}

pub fn fetch_data(result: Value, content: &str) -> Option<String> {
    let data = result.get(content)?.to_string();
    Some(data)
}

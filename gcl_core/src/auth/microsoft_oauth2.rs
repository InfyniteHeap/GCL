use std::collections::HashMap;

use super::webview_login::OAuth2Token;

use reqwest::{header::*, Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::runtime::Runtime;

// These structures are used to store data, which will be used to fetch Minecraft launching parameters later.
#[derive(Default)]
pub struct AccessToken {
    pub token: String,
}
#[derive(Default)]
pub struct XboxIdentity {
    pub token: String,
    pub uhs: String,
}
#[derive(Default)]
pub struct XstsIdentity {
    pub token: String,
}
#[derive(Default, Serialize, Deserialize)]
pub struct MinecraftLaunchParas {
    pub access_token: String,
    pub username: String,
    pub uuid: String,
}

impl AccessToken {
    // Create a blank instance of structure. Fields can not be known in advance.
    pub fn new() -> Self {
        AccessToken {
            token: String::new(),
        }
    }

    // Use "reqwest::Error" instead of "std::error::Error" to make errors more specific.
    pub fn fetch_access_token(&mut self, auth_code: &OAuth2Token) -> Result<(), Error> {
        const TARGET_LINK: &str = "https://login.live.com/oauth20_token.srf";
        const TOKEN: &str = "access_token";

        // Create a hashmap that will be sent with POST request.
        let mut map = HashMap::new();
        map.insert("client_id", "00000000402b5328");
        map.insert("code", &auth_code.code);
        map.insert("grant_type", "authorization_code");
        map.insert("redirect_uri", "https://login.live.com/oauth20_desktop.srf");
        map.insert("scope", "service::user.auth.xboxlive.com::MBI_SSL");

        // Create headers that will be sent with POST request.
        let mut headers = HeaderMap::new();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/x-www-form-urlencoded"),
        );

        // Get responses in String form and fetch the specific data in it.
        let response = send_request(Some(headers), Some(map), TARGET_LINK)?;
        let access_token = fetch_data(&response, TOKEN);

        self.token = access_token;
        Ok(())
    }
}

impl XboxIdentity {
    pub fn new() -> Self {
        XboxIdentity {
            token: String::new(),
            uhs: String::new(),
        }
    }

    pub fn fetch_xbox_token(&mut self, access_token: &AccessToken) -> Result<(), Error> {
        const TARGET_LINK: &str = "https://user.auth.xboxlive.com/user/authenticate";
        const XBOX_TOKEN: &str = "Token";
        const XBOX_UHS: &str = "uhs";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let map = json!({
            "Properties": {
                "AuthMethod": "RPS",
                "SiteName": "user.auth.xboxlive.com",
                "RpsTicket": format!("d={}", &access_token.token)
            },
            "RelyingParty": "https://auth.xboxlive.com",
            "TokenType": "JWT"
        });

        let response = send_request(Some(headers), Some(map), TARGET_LINK)?;
        let xbl_token = fetch_data(&response, XBOX_TOKEN);
        let xbl_uhs = fetch_data(&response, XBOX_UHS);

        self.token = xbl_token;
        self.uhs = xbl_uhs;
        Ok(())
    }
}

impl XstsIdentity {
    pub fn new() -> Self {
        XstsIdentity {
            token: String::new(),
        }
    }

    pub fn fetch_xsts_token(&mut self, xbl_token: &XboxIdentity) -> Result<(), Error> {
        const TARGET_LINK: &str = "https://xsts.auth.xboxlive.com/xsts/authorize";
        const XSTS_TOKEN: &str = "Token";

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        let map = json!({
            "Properties": {
                "SandboxId": "RETAIL",
                "UserTokens": ["{}", &xbl_token.token]
            },
            "RelyingParty": "rp://api.minecraftservices.com/",
            "TokenType": "JWT"
        });

        let response = send_request(Some(headers), Some(map), TARGET_LINK)?;
        let xsts_token = fetch_data(&response, XSTS_TOKEN);

        self.token = xsts_token;
        Ok(())
    }
}

impl MinecraftLaunchParas {
    pub fn new() -> Self {
        MinecraftLaunchParas {
            access_token: String::new(),
            username: String::new(),
            uuid: String::new(),
        }
    }

    pub fn fetch_mc_access_token(
        &mut self,
        xbox_uhs: &XboxIdentity,
        xsts_token: &XstsIdentity,
    ) -> Result<(), Error> {
        const TARGET_LINK: &str =
            "https://api.minecraftservices.com/authentication/login_with_xbox";
        const ACCESS_TOKEN: &str = "access_token";

        let map = json!({
            "identityToken": format!("XBL3.0 x=<{}>;<{}>", &xbox_uhs.uhs, &xsts_token.token)
        });

        let response = send_request(None, Some(map), TARGET_LINK)?;
        let mc_access_token = fetch_data(&response, ACCESS_TOKEN);

        self.access_token = mc_access_token;
        Ok(())
    }

    pub fn fetch_mc_username_uuid(&mut self) -> Result<(), Error> {
        const TARGET_LINK: &str = "https://api.minecraftservices.com/minecraft/profile";

        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer token"));

        let response = send_request::<String>(Some(headers), None, TARGET_LINK)?;

        let error = serde_json::from_str::<Value>(&response)
            .expect("Failed to fetch responses!")
            .get("errorType")
            .is_some();

        if error {
            eprintln!("You don't own Minecraft!");
        } else {
            const NAME: &str = "name";
            const ID: &str = "id";

            let mc_username = fetch_data(&response, NAME);
            let mc_uuid = fetch_data(&response, ID);

            self.username = mc_username;
            self.uuid = mc_uuid;
        }

        Ok(())
    }
}

// Send requests and return results.
pub fn send_request<T>(
    headers: Option<HeaderMap>,
    map: Option<T>,
    target_link: &str,
) -> Result<String, Error>
where
    T: Serialize,
{
    let client = Client::new();
    let tokio_rt = Runtime::new().expect("Failed to create runtime!");
    let mut response = String::new();

    if headers.is_none() {
        if let Some(map) = map {
            let temp = tokio_rt.block_on(client.post(target_link).json(&map).send())?;
            response = tokio_rt.block_on(temp.text())?;
        }
    } else if map.is_none() {
        if let Some(headers) = headers {
            let temp = tokio_rt.block_on(client.post(target_link).headers(headers).send())?;
            response = tokio_rt.block_on(temp.text())?;
        }
    } else if let Some(headers) = headers {
        if let Some(map) = map {
            let temp =
                tokio_rt.block_on(client.post(target_link).headers(headers).json(&map).send())?;
            response = tokio_rt.block_on(temp.text())?;
        }
    }

    Ok(response)
}

// Transfer responses into JSON, fetch necessary fields and store them in the instance of structure.
// Use turbofish syntax to simplify data processing.
pub fn fetch_data(response: &str, content: &str) -> String {
    let data = serde_json::from_str::<Value>(response)
        .expect("Failed to fetch responses!")
        .get(content)
        .expect("Failed to get JSON!")
        .to_string();
    data
}

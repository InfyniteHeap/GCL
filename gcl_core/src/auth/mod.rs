//! # Microsoft Oauth2 Module
//!
//! This module is used to complete the process of signing in Microsoft account
//! and verifing whether this account owned Minecraft.
//!
//! To finish the verifaciation process, sign in your account first.

mod microsoft_oauth2;
mod webview_login;

use crate::{file::*, path::constant::*};
use microsoft_oauth2::*;

use log::*;
use reqwest::Error;
use serde_json::to_string;
use webview_login::OAuth2Token;

pub fn oauth2() {
    // Create blank structures.
    let code = OAuth2Token::new();

    let mut access_t = AccessToken::new();
    let mut xbox_t = XboxIdentity::new();
    let mut xsts_t = XstsIdentity::new();
    let mut mc_launch_paras = MinecraftLaunchParas::new();

    // Todo: Explicitly handle errors instead of passing them through.
    match process_oauth2(
        &code,
        &mut access_t,
        &mut xbox_t,
        &mut xsts_t,
        &mut mc_launch_paras,
    ) {
        Ok(_) => (),
        Err(err) => error!("Failed in {err}"),
    }

    let paras_json = to_string(&mc_launch_paras).expect("Failed to transfer instance into JSON!");

    create_file(MC_LAUNCH_PARAS);

    todo!()
}

fn process_oauth2(
    code: &OAuth2Token,
    access_t: &mut AccessToken,
    xbox_t: &mut XboxIdentity,
    xsts_t: &mut XstsIdentity,
    mc_launch_paras: &mut MinecraftLaunchParas,
) -> Result<(), Error> {
    access_t.fetch_access_token(code)?;
    xbox_t.fetch_xbox_token(access_t)?;
    xsts_t.fetch_xsts_token(xbox_t)?;
    mc_launch_paras.fetch_mc_access_token(xbox_t, xsts_t)?;
    mc_launch_paras.fetch_mc_username_uuid()?;
    Ok(())
}

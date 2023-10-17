mod game_dl_addr;
mod mods_dl_addr;

use std::fs::*;

use crate::path;
use game_dl_addr::*;
use mods_dl_addr::*;

use reqwest::{get, Error};
use tokio::runtime::Runtime;

pub fn select_download_res(option: i32) -> &'static str {
    match option {
        0 => OFFICIAL_RES,
        1 => BANGBANG93_RES,
        2 => MCBBS_RES,
        _ => unreachable!(),
    }
}

pub fn download_metadata(option: i32) -> Result<(), Error> {
    const METADATA_PATH: &str = "/versions/launcher.json";
    const ADDR_SUFFIX: &str = "/mc/game/version_manifest_v2.json";

    // Todo: Handle errors loaclly instead of passing it though the calling function.
    let url = select_download_res(option).to_string() + ADDR_SUFFIX;
    let tokio_rt = Runtime::new().expect("Failed to create runtime!");

    let temp = tokio_rt.block_on(get(url))?;
    let response = tokio_rt.block_on(temp.text())?;

    todo!()
}

pub fn download_ver_metadata() {
    todo!()
}

pub fn download_mc_files() {
    todo!()
}

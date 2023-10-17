pub mod constant;

use std::{fs::*, io::*, path::*};

use constant::*;

// Check if there had existed Minecraft folders.
// If so, trun to next steps, or create a folder.
#[cfg(target_os = "windows")]
pub fn detect_or_create_mc_dir() -> &'static str {
    if metadata(WINDOWS_DEFAULT_PATH).is_ok() {
        WINDOWS_DEFAULT_PATH
    } else {
        create_dir_all(LAUNCHER_PATH).expect("Failed to create directory!");
        LAUNCHER_PATH
    }
}
#[cfg(target_os = "macos")]
pub fn detect_or_create_mc_dir() -> &'static str {
    if metadata(MACOS_DEFAULT_PATH).is_ok() {
        MACOS_DEFAULT_PATH
    } else {
        create_dir_all(MACOS_LAUNCHER_PATH).expect("Failed to create directory!");
        MACOS_LAUNCHER_PATH
    }
}
#[cfg(target_os = "linux")]
pub fn detect_or_create_mc_dir() -> &'static str {
    if metadata(LINUX_DEFAULT_PATH).is_ok() {
        LINUX_DEFAULT_PATH
    } else {
        create_dir_all(LAUNCHER_PATH).expect("Failed to create directory!");
        LAUNCHER_PATH
    }
}

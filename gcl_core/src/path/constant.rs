// Set directory where will store all of Minecraft files.
pub const WINDOWS_DEFAULT_PATH: &str = "%AppData%/Roaming/.minecraft";
pub const MACOS_DEFAULT_PATH: &str = "~/Library/Application Support/minecraft";
pub const LINUX_DEFAULT_PATH: &str = "~/.minecraft";

// If GCL does not find Minecraft folders, these constants will be used.
pub const LAUNCHER_PATH: &str = "./.minecraft";
pub const MACOS_LAUNCHER_PATH: &str = "./minecraft";

// Set specific files that will store info.
pub const MC_LAUNCH_PARAS: &str = "mc_launch_paras.json";
pub const LAUNCHER_OPTIONS: &str = "launcher_options.json";
pub const MC_FILES_INFO: &str = "mc_files_info.json";

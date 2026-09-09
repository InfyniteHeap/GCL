const COMPANY_NAME: &str = "InfyniteHeap";
const FILE_DESCRIPTION: &str = "A beautiful, fast and memory-safe Minecraft launcher.";
const BINARY_NAME: &str = "GCL";
const LEGAL_COPYRIGHT: &str = "Copyright \u{00A9} 2024-present InfyniteHeap.";
const PRODUCT_NAME: &str = "Grid Craft Launcher";

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn parse_version(version: &str) -> [&str; 4] {
    let mut parts = ["0"; 4];
    let mut idx = 0;

    'outer: for part in version.split('.') {
        let mut digits = 0;
        for c in part.chars() {
            if !c.is_ascii_digit() {
                parts[idx] = &part[..digits];

                break 'outer;
            } else {
                digits += 1;
            }
        }

        if !part.is_empty() {
            parts[idx] = part;
            idx += 1;
        }
    }

    parts
}

fn main() {
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    {
        windows_reactor_setup::as_self_contained();

        embed_resources();
    }
    #[cfg(not(all(target_os = "windows", target_env = "msvc")))]
    compile_error!("This project must be built on Windows with MSVC toolchain!");
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn embed_resources() {
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or("0.0.0".into());
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");

    let manifest_path = build_manifest(&version, &out_dir);
    build_icon(&out_dir);
    let rc_path = build_resource_file(&version, &out_dir);

    // Manifest and resource file must be separately embedded in binary
    println!(
        "cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}",
        manifest_path.display()
    );
    embed_resource::compile(rc_path, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn build_manifest(version: &str, out_dir: &str) -> std::path::PathBuf {
    let raw_manifest = include_str!("res/gcl.exe.manifest");
    let manifest = raw_manifest.replace("{{VERSION}}", &parse_version(version).join("."));

    let manifest_path = std::path::Path::new(out_dir).join("gcl.exe.manifest");
    std::fs::write(&manifest_path, manifest).expect("failed to write generated gcl.exe.manifest");

    manifest_path
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn build_icon(out_dir: &str) {
    let raw_icon = include_bytes!("assets/logo.ico");

    let icon_path = std::path::Path::new(out_dir).join("logo.ico");
    std::fs::write(&icon_path, raw_icon).expect("failed to write generated logo.ico");
}

#[cfg(all(target_os = "windows", target_env = "msvc"))]
fn build_resource_file(version: &str, out_dir: &str) -> std::path::PathBuf {
    let is_pre_release = version.split('+').next().unwrap_or_default().contains('-');

    let file_version = parse_version(version).join(",");
    let file_flags = match (std::env::var("PROFILE").as_deref(), is_pre_release) {
        (Ok("debug"), true) => "VS_FF_DEBUG | VS_FF_PRERELEASE",
        (Ok("debug"), false) => "VS_FF_DEBUG",
        (_, true) => "VS_FF_PRERELEASE",
        _ => "0",
    };

    let raw_rc = include_str!("res/gcl.rc");
    let rc = format!("\u{FEFF}{raw_rc}")
        .replace("{{FILE_VERSION}}", &file_version)
        .replace("{{FILE_FLAGS}}", file_flags)
        .replace("{{COMPANY_NAME}}", COMPANY_NAME)
        .replace("{{FILE_DESCRIPTION}}", FILE_DESCRIPTION)
        .replace("{{BINARY_NAME}}", BINARY_NAME)
        .replace("{{LEGAL_COPYRIGHT}}", LEGAL_COPYRIGHT)
        .replace("{{PRODUCT_NAME}}", PRODUCT_NAME)
        .replace("{{VERSION}}", version);

    let rc_path = std::path::Path::new(out_dir).join("gcl.rc");
    std::fs::write(&rc_path, rc).expect("failed to write generated gcl.rc");

    rc_path
}

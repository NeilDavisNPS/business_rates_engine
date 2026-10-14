use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/index");

    let commit_hash = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "dev".to_string());

    let build_number = Command::new("git")
        .args(["rev-list", "--count", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "0".to_string());

    let build_timestamp = chrono::Utc::now().to_rfc3339();

    println!("cargo:rustc-env=APP_COMMIT_HASH={}", commit_hash);
    println!("cargo:rustc-env=APP_BUILD_NUMBER={}", build_number);
    println!("cargo:rustc-env=APP_BUILD_TIMESTAMP={}", build_timestamp);
}


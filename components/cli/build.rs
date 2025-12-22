//! Build script for yt-rs-backend.
//!
//! Captures build-time information for version output.

use std::process::Command;

fn main() {
    // Rerun if git HEAD changes
    println!("cargo::rerun-if-changed=.git/HEAD");

    // Build host
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    println!("cargo::rustc-env=BUILD_HOST={hostname}");

    // Build time (UTC)
    let build_time = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string();
    println!("cargo::rustc-env=BUILD_TIME={build_time}");

    // Git commit (short hash)
    let commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    println!("cargo::rustc-env=BUILD_COMMIT={commit}");
}

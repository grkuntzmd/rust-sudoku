extern crate chrono;
use chrono::{DateTime, Utc};
use std::process::Command;

fn main() {
    let now: DateTime<Utc> = Utc::now();

    // git describe --always --long
    let git_hash = Command::new("git")
        .arg("describe")
        .arg("--always")
        .arg("--long")
        .output()
        .map(|r| String::from(String::from_utf8_lossy(&r.stdout)))
        .expect("git hash not available");

    println!(
        "cargo:rustc-env=BUILD_TIMESTAMP={}\ncargo:rustc-env=GIT_HASH={}",
        now.to_rfc2822(),
        git_hash
    );
}

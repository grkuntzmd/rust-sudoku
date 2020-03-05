extern crate chrono;
use chrono::{DateTime, Utc};

fn main() {
    let now: DateTime<Utc> = Utc::now();

    println!("cargo:rustc-env=BUILDINFO={:?}", now.to_rfc2822());
}
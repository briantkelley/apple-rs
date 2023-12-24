//! Automatically link `CoreFoundation.framework` when using this crate.

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_VENDOR");

    let target_vendor_is_apple = env::var("CARGO_CFG_TARGET_VENDOR")
        .ok()
        .map(|vendor| vendor == "apple")
        .unwrap_or_default();

    if target_vendor_is_apple {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
    }
}

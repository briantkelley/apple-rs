//! Automatically link `libicucore.dylib` when using this crate.

use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_TARGET_VENDOR");

    let target_vendor_is_apple =
        env::var("CARGO_CFG_TARGET_VENDOR").is_ok_and(|vendor| vendor == "apple");

    if target_vendor_is_apple {
        println!("cargo:rustc-link-lib=icucore");
    }
}

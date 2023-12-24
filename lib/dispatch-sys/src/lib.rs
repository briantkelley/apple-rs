//! Direct FFI bindings to Apple's Dispatch Clang module.
//!
//! The Clang module definition can be found in the iOS and macOS SDKs included with Xcode at
//! `$SDKROOT/usr/include/dispatch/module.modulemap`. These FFI bindings are derived from
//! [libdispatch-1462.0.4](https://github.com/apple-oss-distributions/libdispatch/tree/libdispatch-1462.0.4).

#![no_std]
#![allow(clippy::undocumented_unsafe_blocks, missing_docs, non_camel_case_types)]

mod base;
mod once;

pub use base::*;
pub use once::*;

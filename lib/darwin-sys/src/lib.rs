//! Direct FFI bindings to Apple's Darwin Clang module.
//!
//! The Clang module definition can be found in the iOS and macOS SDKs included with Xcode at
//! `$SDKROOT/usr/include/module.modulemap`.

#![no_std]
#![allow(missing_docs)]

mod posix;

pub use posix::*;

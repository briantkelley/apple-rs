//! Direct FFI bindings to Apple's ICU Clang module.
//!
//! The Clang module definition can be found in the iOS and macOS SDKs included with Xcode at
//! `$SDKROOT/usr/include/unicode/module.modulemap`, or in the Apple OSS Distributions
//! [ICU](https://github.com/apple-oss-distributions/ICU/blob/ICU-74000.403/modules/ICU.modulemap)
//! GitHub project.
//!
//! Ideally, in the medium term, this crate will be replaced with a canonical ICU sys crate, perhaps
//! Google's [`rust_icu`](https://github.com/google/rust_icu). However, Apple's distribution of the
//! `common` library is only a subset of the typical distribution. Apple's public repository refers to
//! the available APIs as
//! "[minimal](https://github.com/apple-oss-distributions/ICU/blob/ICU-74000.403/minimalapis.txt)" and
//! exposes them through a dynamic library named
//! [`icucore`](https://github.com/apple-oss-distributions/ICU/blob/ICU-74000.403/modules/ICU.modulemap#L26).
//!
//! To build the ideal canonical `common-sys` crate, we'll need to document how Apple's distribution
//! diverges from the typical distribution and consider how to configure the sys crate to reflect
//! Apple's publicly available APIs.

#![no_std]
#![allow(non_camel_case_types)]

mod uchar;
mod umachine;
mod uversion;

pub use uchar::*;
pub use umachine::*;
pub use uversion::*;

//! # corefoundation
//!
//! Core Foundation is a framework that provides fundamental software services useful to application
//! services, application environments, and to applications themselves. Core Foundation also
//! provides abstractions for common data types, facilitates internationalization with Unicode
//! string storage, and offers a suite of utilities such as plug-in support, XML property lists, URL
//! resource access, and preferences.
//!
//! This crate aims to provide idiomatic Rust bindings to Apple's `CoreFoundation` Clang module
//! (located at `$SDKROOT/System/Library/Frameworks/CoreFoundation.framework/Modules/module.modulemap`)
//! that mirror [The Rust Standard Library](https://doc.rust-lang.org/std/) as closely as possible.

#![allow(clippy::redundant_pub_crate)]
#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]

mod base;

pub use base::convert::{ExpectFrom, FromUnchecked};
pub use base::ffi;
pub use base::object::Object;

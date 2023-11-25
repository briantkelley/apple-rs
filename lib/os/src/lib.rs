//!
//! # os
//!
//! Idiomatic Rust bindings to Apple's OS Clang module (located at
//! `$SDKROOT/usr/include/module.modulemap`).

#![no_std]

#[cfg(feature = "experimental")]
#[macro_use]
mod macros;
#[cfg(feature = "experimental")]
mod sys;

#[cfg(feature = "experimental")]
pub mod activity;
#[cfg(feature = "experimental")]
pub mod log;
#[cfg(feature = "experimental")]
pub mod trace_base;

#[cfg(feature = "experimental")]
pub use macros::paste;

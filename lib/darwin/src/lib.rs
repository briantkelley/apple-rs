//!
//! # darwin
//!
//! Idiomatic Rust bindings to Apple's Darwin Clang module (located at
//! `$SDKROOT/usr/include/module.modulemap`).

#![no_std]

#[cfg(feature = "experimental")]
mod _sys;

#[cfg(feature = "experimental")]
pub mod c;
#[cfg(feature = "experimental")]
pub mod io;
#[cfg(feature = "experimental")]
pub mod posix;
#[cfg(feature = "experimental")]
pub mod sys;

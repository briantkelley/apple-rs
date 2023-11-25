//! # dispatch
//!
//! Execute code concurrently on multicore hardware by submitting work to dispatch queues managed by
//! the system.
//!
//! This crate aims to proivde idiomatic Rust bindings to Apple's Dispatch Clang module (located at
//! `$SDKROOT/usr/include/dispatch/module.modulemap`), in the same way Apple's Swift overlay for
//! the [Dispatch framework](https://developer.apple.com/documentation/DISPATCH) provides idiomatic
//! Swift bindings.

#![no_std]

mod lazy_static;
#[cfg(feature = "experimental")]
mod object;
mod once;
#[cfg(feature = "experimental")]
mod queue;
#[cfg(feature = "experimental")]
mod sys;

pub use lazy_static::*;
#[cfg(feature = "experimental")]
pub use object::Object;
pub use once::*;
#[cfg(feature = "experimental")]
pub use queue::Queue;

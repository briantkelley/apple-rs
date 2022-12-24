//! Typically FFI bindings are split into two crates: one crate is the raw C interface expressed in
//! Rust with the idiomatic crate's name is suffixed with `-sys`, and the other is the idiomatic Rust
//! interface built on the raw C interface crate.
//!
//! In lieu of creating `-sys` crates, this repository opts for `sys` modules with `public(crate)`
//! visibility. But, the Darwin Clang module contains a `sys` submodule, which conflicts with this
//! convention. So, for this crate, the system interface is available in this `_sys` module.

pub(crate) mod sys;
pub(crate) mod unistd;

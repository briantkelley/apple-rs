//! Facilities to simplify safe crossing of the Rust/foreign interface boundary.

pub mod convert;
pub use retain_release::ffi::ForeignFunctionInterface;

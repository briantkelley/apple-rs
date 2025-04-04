//! Convenience utilities for implementing \*-sys crates and Rust bindings.

/// Defines an opaque type compatible with opaque C struct types for use in defining foreign types
/// in Rust.
///
/// By default, the type is `!Send` and `!Sync`, but these traits may be implemented if supported by
/// the foreign type.
///
/// The type is also `!Unpin` as it is illogical to move out of an opaque struct.
///
/// See [The Rustonomicon][] for more information.
///
/// [The Rustonomicon]: https://doc.rust-lang.org/nomicon/ffi.html#representing-opaque-structs
#[macro_export]
macro_rules! opaque_type {
    ($(#[$doc:meta])* $ident:ident) => {
        $(#[$doc])*
        #[repr(C)]
        pub struct $ident {
            _data: [u8; 0],
            _marker: core::marker::PhantomData<(*const u8, core::marker::PhantomPinned)>,
        }
    }
}

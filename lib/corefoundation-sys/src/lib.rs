//! Direct FFI bindings to Apple's `CoreFoundation` Clang module.
//!
//! The Clang module definition can be found in the iOS and macOS SDKs included with Xcode at
//! `$SDKROOT/System/Library/Frameworks/CoreFoundation.framework/Modules/module.modulemap`.

#![allow(
    clippy::redundant_pub_crate,
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

/// Defines an opaque type to act as a proxy for a Core Foundation object type.
///
/// By default, the type is `!Send` and `!Sync`. These markers must be added by the bindings, as
/// appropriate for the object type.
macro_rules! declare_cf_type {
    ($struct:ident, $ref:ident) => {
        // LINT: This type is not intended to be user accessible.
        #[allow(missing_copy_implementations, missing_debug_implementations)]
        #[repr(C)]
        pub struct $struct {
            _data: [u8; 0],
            _marker: core::marker::PhantomData<core::marker::PhantomPinned>,
        }
        pub type $ref = *const $struct;
    };
    ($struct:ident, $ref:ident, $mutable_ref:ident) => {
        declare_cf_type!($struct, $ref);
        pub type $mutable_ref = *mut $struct;
    };
}

mod base;
mod string;
mod string_encoding_ext;

pub use base::*;
pub use string::*;
pub use string_encoding_ext::*;

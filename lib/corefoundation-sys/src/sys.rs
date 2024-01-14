#![allow(
    missing_docs,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]

use c_ffi::opaque_type;

/// Defines an opaque type to act as a proxy for a Core Foundation object type.
///
/// By default, the type is `!Send` and `!Sync`, but these traits may be implemented if supported by
/// the object type.
macro_rules! declare_cf_type {
    ($struct:ident, $ref:ident) => {
        $crate::sys::opaque_type!($struct);
        pub type $ref = *const $struct;
    };
    ($struct:ident, $ref:ident, $mutable_ref:ident) => {
        declare_cf_type!($struct, $ref);
        pub type $mutable_ref = *mut $struct;
    };
}

pub(crate) mod base;
pub(crate) mod string;
pub(crate) mod string_encoding_ext;

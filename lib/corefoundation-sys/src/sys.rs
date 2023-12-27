#![allow(
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

pub(crate) mod base;
pub(crate) mod string;
pub(crate) mod string_encoding_ext;

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
#![warn(
    clippy::arithmetic_side_effects,
    clippy::assertions_on_result_states,
    clippy::as_conversions,
    clippy::cargo,
    clippy::default_numeric_fallback,
    clippy::empty_structs_with_brackets,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::let_underscore_must_use,
    clippy::missing_assert_message,
    clippy::missing_inline_in_public_items,
    clippy::mod_module_files,
    clippy::multiple_inherent_impl,
    clippy::nursery,
    clippy::obfuscated_if_else,
    clippy::panic_in_result_fn,
    clippy::pattern_type_mismatch,
    clippy::pedantic,
    clippy::pub_without_shorthand,
    clippy::semicolon_outside_block,
    clippy::single_char_lifetime_names,
    clippy::std_instead_of_alloc,
    clippy::std_instead_of_core,
    clippy::string_slice,
    clippy::undocumented_unsafe_blocks,
    clippy::unreachable,
    clippy::unseparated_literal_suffix,
    clippy::unwrap_used,
    deprecated_in_future,
    keyword_idents,
    let_underscore_drop,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2018_idioms,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    unused_tuple_struct_fields,
    variant_size_differences
)]

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

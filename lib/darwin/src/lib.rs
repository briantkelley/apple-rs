/*!
# darwin

Idiomatic Rust bindings to Apple's Darwin Clang module (located at `$SDKROOT/usr/include/module.modulemap`).
*/

#![no_std]
#![warn(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rustdoc::invalid_html_tags,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_op_in_unsafe_fn,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![allow(clippy::missing_errors_doc, clippy::redundant_pub_crate)]

mod _sys;
pub mod errno;
pub mod sys;
pub mod unistd;

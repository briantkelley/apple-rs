/*!
# `objc4_foundation`

Idiomatic Rust bindings for Apple's Foundation framework.

## Classes

### `NSString`

The crate includes basic support for creating `NSString` instances and constants:

```compile_fail
// compile-time constant string
string_literal!(static greeting: NSString = "Hello");

// heap allocated string
let location = NSString::from_str("Bellevue");
```
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
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rustdoc::invalid_html_tags,
    rustdoc::missing_crate_level_docs,
    rustdoc::missing_doc_code_examples,
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

#[macro_use]
mod macros;
#[macro_use]
pub mod sel;

mod object;
mod string;

pub use object::NSCopying;
pub use string::{
    NSString, NSStringClass, NSStringClassInterface, NSStringEncoding, NSStringInterface,
};

/*!
# `objc4_foundation`

Idiomatic Rust bindings for Apple's Foundation framework.

## Classes

The crate includes support for creating and using instances of:

* `NSDictionary` and `NSMutableDictionary`
* `NSString` (including compile-time constants)

```compile_fail
string_literal!(static LOCATION: NSString = "location"); // compile-time constant
let location = NSString::from_str("Bellevue");           // heap allocated

let mut dict = NSMutableDictionary::<NSString, NSString>::new();
dict.set(&LOCATION, location);
assert_eq!(dict.len(), 1);
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

mod dictionary;
mod object;
mod string;
mod value;

pub use dictionary::{
    NSDictionary, NSDictionaryClass, NSDictionaryInterface, NSMutableDictionary,
    NSMutableDictionaryClass, NSMutableDictionaryInterface,
};
pub use object::NSCopying;
pub use string::{
    NSString, NSStringClass, NSStringClassInterface, NSStringEncoding, NSStringInterface,
};
pub use value::{
    NSNumber, NSNumberClass, NSNumberClassInterface, NSNumberInterface, NSValue, NSValueClass,
    NSValueInterface,
};

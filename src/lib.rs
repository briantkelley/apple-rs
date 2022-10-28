/*!
# objc4

Idiomatic Rust bindings for Apple's Objective-C runtime.

This crate aims to provide:

1. An idiomatic Rust interface for Apple's Objective-C runtime APIs.
2. Facilities to build idiomatic Rust interfaces for Objective-C interfaces.
3. Zero overhead when crossing the language boundary. Optimized Rust code should be identical to
   optimized Clang code, to the extent the Objective-C ABI is expressible in Rust.

## Usage

The `extern_class!` macro creates a new type for an Objective-C class defined in an external library
and implements traits for its class hierarchy. For example, the following creates a new type for
`NSArray`, which inherits from `NSObject`, and is defined in the `Foundation` framework:

```compile_fail
extern_class!(Foundation, pub NSArray, NSObject);
```

Because Rust does not have type inheritance, the class interface is implemented by a trait whose
name is the class name suffixed with `Interface`. Continuing the `NSArray` example, the Interface
trait and default implementation may be implemented as follows:

```compile_fail
pub trait NSArrayInterface: NSObjectInterface {
    #[must_use]
    fn with_objects(objects: &[id]) -> Box<Self> {
        let obj = msg_send!(id, *const id, usize)(
            Self::alloc().as_ptr(),
            sel![INITWITHOBJECTS_COUNT_],
            objects.as_ptr(),
            objects.len(),
        );
        // SAFETY: Objects returned by selectors beginning with ‘alloc’ must be released.
        // Panics: -initWithObjects: has a non-null return type annotation so the unwrap()
        // panic if that specification is violated.
        unsafe { Box::with_transfer(NonNull::new(obj).unwrap()) }
    }

    #[must_use]
    fn count(&self) -> usize {
        msg_send!(usize)(self.as_ptr(), sel![COUNT])
    }

    #[must_use]
    fn object_at_index(&self, index: usize) -> &objc_object {
        let obj = msg_send!(id, usize)(self.as_ptr(), sel![OBJECTATINDEX_], index);
        // SAFETY: `NSArray` cannot store `nil` pointers.
        unsafe { &*obj }
    }
}
```

The `extern_class!` macro assumes the trait provides a default implementation for all of its
methods, which enables subclasses (e.g. `NSMutableArray`) to receive the superclass bindings for
free.

The `msg_send!` macro returns a function pointer to `objc_msgSend` that is type cast given the macro
arguments. The first type argument is the method return type and any additional type arguments are
the types of the method arguments (after `self` and `_cmd`). Invoke the function pointer with the
required arguments to send the message.

The `sel!` macro is a tiny convenience to avoid having to manually add `unsafe { }` blocks around
the `extern "C"` selector symbol accesses. Selectors are defined using the `selector!` macro in a
style similar to Rust globals:

```compile_fail
selector!(COUNT = "count");
selector!(INITWITHOBJECTS_COUNT_ = "initWithObjects:count:");
selector!(OBJECTATINDEX_ = "objectAtIndex:");
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

mod arc;
mod boxed;
mod class;
mod nsobject;
mod object;
mod sys;

pub use arc::Arc;
pub use boxed::Box;
pub use class::*;
pub use macros::paste;
pub use nsobject::{NSObject, NSObjectInterface, NSObjectProtocol};
pub use object::Object;
pub use sys::{id, objc_class, objc_msgSend, objc_object, Class};

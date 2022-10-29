# objc4

Idiomatic Rust bindings for Apple's Objective-C runtime.

This crate aims to provide:

1. An idiomatic Rust interface for Apple's Objective-C runtime APIs.
2. Facilities to build idiomatic Rust interfaces for Objective-C interfaces.
3. Zero overhead when crossing the language boundary. Optimized Rust code should be identical to
   optimized Clang code, to the extent the Objective-C ABI is expressible in Rust.

Why is this crate called `objc4`? The name matches [Apple's open-source project][objc] (and `objc`
was already taken).

## Usage

The `extern_class!` macro creates a new type for an Objective-C class defined in an external library
and implements traits for its class hierarchy. For example, the following creates a new type for
`NSArray`, which inherits from `NSObject`, and is defined in the `Foundation` framework:

```rust
extern_class!(Foundation, pub NSArray, NSObject 'cls);
```

Because Rust does not have type inheritance, the class interface is implemented by a trait whose
name is the class name suffixed with `Interface`. Continuing the `NSArray` example, the Interface
trait and default implementation may be implemented as follows:

```rust
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

```rust
selector!(COUNT = "count");
selector!(INITWITHOBJECTS_COUNT_ = "initWithObjects:count:");
selector!(OBJECTATINDEX_ = "objectAtIndex:");
```

## Forward Source Compatibility

This crate is very early in the development stage and no effort will be made to preserve source
compatibility across patch versions in 0.0.x. For example, `objc4::Arc<T>` provides no mechanism for
obtaining a mutable reference and it's not immediately clear how to add that functionality with the
current design. After this crate matures a bit and reaches version 0.1.0, source breaking changes
will increment the minor version.

objc: https://github.com/apple-oss-distributions/objc4

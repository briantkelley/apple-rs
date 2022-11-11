# objc4

Idiomatic Rust bindings for Apple's Objective-C runtime.

This crate aims to provide:

1. An idiomatic Rust interface for Apple's Objective-C runtime APIs.
2. Facilities to build idiomatic Rust interfaces for Objective-C interfaces.
3. Zero overhead when crossing the language boundary. Optimized Rust code should be identical to
   optimized Clang code, to the extent the Objective-C ABI is expressible in Rust.

Why is this crate called `objc4`? The name matches [Apple's open-source project][https://github.com/apple-oss-distributions/objc4]
(and `objc` was already taken).

## Usage

The `extern_class!` macro creates a new type for an Objective-C class defined in an external library
and implements traits for its class hierarchy. The following example creates a new type for
`NSArray`, which inherits from `NSObject`, and is defined in the `Foundation` framework:

Because Rust does not have type inheritance, the class hierarchy is implemented with traits. A
class's instance methods (prefixed with `-` in Objective-C) are implemented in a trait whose name is
the class name suffixed with `Interface`. A class's class methods (prefixed with a `+` in
Objective-C) are implemented in a trait whose name is the class name suffixed with `ClassInterface`.
The `'cls` lifetime (abuse) in the `extern_class!` macro indicates a `ClassInterface` is defined for
that class.

The `extern_class!` macro assumes the trait provides a default implementation for all of its
methods, which enables subclasses (e.g. `NSMutableArray`) to receive the superclass bindings for
free.

Using these constructs, `NSArray`'s `ClassInterface` trait, `Interface` trait, and their default
implementations may be written as follows:

```rust
extern_class!(Foundation, pub NSArray 'cls, NSObject 'cls);

pub trait NSArrayClassInterface: NSObjectClassInterface {
    #[must_use]
    fn from_objects(&self, objects: &[id]) -> Box<NSArray> {
        let obj = msg_send!((id)[self.alloc().as_ptr(), initWithObjects:(*const id)objects.as_ptr()
                                                                  count:(usize)objects.len()]);
        // SAFETY: Objects returned by selectors beginning with ‘alloc’ must be released.
        // Panics: -initWithObjects: has a non-null return type annotation so the unwrap()
        // panic if that specification is violated.
        unsafe { Box::with_transfer(NonNull::new(obj).unwrap()) }
    }
}

pub trait NSArrayInterface: NSObjectInterface {
    #[must_use]
    fn count(&self) -> usize {
        msg_send!((usize)[self, count])
    }

    #[must_use]
    fn object_at_index(&self, index: usize) -> &objc_object {
        let obj = msg_send!((id)[self, objectAtIndex:(usize)index]);
        // SAFETY: `NSArray` cannot store `nil` pointers.
        unsafe { &*obj }
    }
}
```

The `msg_send!` macro, which approximates the spelling of an Objective-C method declaration, casts
the signature of `objc_msgSend` to match the return and argument types given in the macro, emits a
selector, and calls `objc_msgSend` with the given arguments and emitted selector.

## Forward Source Compatibility

This crate is very early in the development stage and no effort will be made to preserve source
compatibility across patch versions in 0.0.x. For example, `objc4::Arc<T>` provides no mechanism for
obtaining a mutable reference and it's not immediately clear how to add that functionality with the
current design. After this crate matures a bit and reaches version 0.1.0, source breaking changes
will increment the minor version.

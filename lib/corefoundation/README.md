# corefoundation

Core Foundation is a framework that provides fundamental software services useful to application
services, application environments, and to applications themselves. Core Foundation also provides
abstractions for common data types, facilitates internationalization with Unicode string storage,
and offers a suite of utilities such as plug-in support, XML property lists, URL resource access,
and preferences.

This crate aims to provide idiomatic Rust bindings to Apple's `CoreFoundation` Clang module (located
at `$SDKROOT/System/Library/Frameworks/CoreFoundation.framework/Modules/module.modulemap`) that
mirror [The Rust Standard Library](https://doc.rust-lang.org/std/) as closely as possible.

## Design

### Downstream API Bindings

Apple defines types compatible with the polymorphic Core Foundation functions outside of the Core
Foundation framework (e.g., `CoreGraphics`, `CoreText`). The facilities this crate uses to provide
idiomatic Rust API bindings for Core Foundation are available to crates implementing Rust API
bindings for frameworks with Core Foundation-compatible types.

`ForeignFunctionInterface` is the primary trait used to bridge the between the foreign function
interface and Rust. It provides a facility to retrieve the `CFTypeRef` pointer from the Rust type
for use in calling foreign functions. This trait **should not** be used by crates utilizing the Rust
API bindings; it's intended only for crates *implementing* Rust API bindings. This is intentionally
separate from `Object` so the FFI related functionality is not visible by default when using the
`Object` interface.

The `declare_and_impl_type!` macro declares a new type on which to implement Rust bindings for a
Core Foundation-compatible type. A new type is required to implement the many of the standard
traits, as the FFI type definition typically originates in a separate `-sys` crate.

### Memory Management

The `Box<T>` and `Arc<T>` smart pointers implemented by this crate fulfill the following
requirements:

* Are a true zero-cost abstraction.
* Show Core Foundation objects are heap-allocated through the type system.
* Combine Rust’s mutable references with Core Foundation’s mutable types.

The type name `Box<T>` signals to the reader that `T` is heap-allocated and that the instance `T` is
unique. Similarly, the type name `Arc<T>` indicates `T` is heap-allocated and that the instance `T`
is shared with other parts of the program.

Both types `Deref` to `T`, the Rust type implementing the Core Foundation API bindings, which is
crucial in making the abstraction zero-cost. When the smart pointer is dereferenced by the compiler,
it returns the Core Foundation pointer value as a reference to `T`, which can be passed directly
through to the C API.

The implementations of `Box<T>` and `Arc<T>` for Core Foundation are virtually identical, with the
primary difference being `Box<T>` also implements `DerefMut`, `AsMut`, and `BorrowMut`. Therefore,
`Box<T>` should only be used if the Core Foundation object instance a mutable type uniquely owned by
the raw pointer (i.e., a `Create` or `Copy` function return the pointer). Otherwise, immutable types
and objects that may be retained elsewhere should use `Arc<T>`.

### Signed/Unsigned Conversion

Core Foundation's canonical index type and size type, `CFIndex`, is signed. Foundation's canonical
index type and size type, `NSUInteger`, is unsigned, and Rust's canonical type, `usize`, is unsigned
too.

The following subsections describe Apple's implementation of signed/unsigned conversion between
Core Foundation and Foundation, Apple's implementation of signed/unsigned conversion between
Foundation and Swift, and this crate's approach to signed/unsigned conversion between Core
Foundation and Rust.

#### Foundation's Approach

Many Core Foundation and Foundation [types are interchangeable](https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/Toll-FreeBridgin/Toll-FreeBridgin.html).
`CFIndex` is used throughout Core Foundation, while Foundation's equivalent interface uses
`NSUInteger`. **Foundation effectively uses an unchecked bit cast** when converting between
`CFIndex` and `NSUInteger`—the values pass through without detection of a potential wrap or sign
loss. So, integer values with the bit at `1 << sizeof(size_t) * 8 - 1` set to `1` will be negative
in the Core Foundation interface but positive in the Foundation interface.

The only acknowledgment of the type mismatch in Apple's documentation (as far as I'm aware) is this
(slightly edited) description of the `location` and `length` fields on [`CFRange`](https://developer.apple.com/documentation/corefoundation/cfrange)
and [`NSRange`](https://developer.apple.com/documentation/foundation/nsrange/1459533-location):

> For type compatibility with the rest of the system, `LONG_MAX` is the maximum value you should use
> for location and length.

An interesting related quirk is the difference between [`kCFNotFound`](https://github.com/apple/swift-corelibs-foundation/blob/swift-5.9-RELEASE/CoreFoundation/Base.subproj/CFBase.h#L497)
and [`NSNotFound`](https://github.com/apple/swift-corelibs-foundation/blob/swift-5.9-RELEASE/Darwin/Foundation-swiftoverlay/Foundation.swift#L26).
Although many Core Foundation and Foundation types are interchangeable, the semantic "not found"
value is interface-dependent. `kCFNotFound` is defined as `-1` while `NSNotFound` is `NSIntegerMax`
(i.e., the maximum value of `CFIndex`, `LONG_MAX`). So, the addressable range of failable Foundation
methods returning an `NSRange` is effectively limited to `[0, LONG_MAX)`.

Using a signed type in the underlying Core Foundation implementation and `NSNotFound`'s definition
as the maximum signed value inhibit Foundation from utilizing the full range of the unsigned type.

#### Swift's Approach

For Apple's frameworks (e.g., AppKit, Foundation, UIKit, etc.), the Swift compiler imports
`NSUInteger` as `Int`. To make C and Objective-C interfaces visible to Swift code, the Swift
compiler "imports" a [Clang module](https://clang.llvm.org/docs/Modules.html) to build a Swift
module, where "import" is a Swift compiler process that constructs a Swift-native representation of
the compatible declarations, definitions, and types present in the Clang module.

Normally, `NSUInteger` imports as `UInt`. But, in system modules (i.e., modules found under a
directory given by an `-isystem` flag), **Swift silently retypes `NSUInteger` to `Int`** unless the
`NSUInteger` declares the type of an enum. This retyping operation occurs during the construction of
the Swift module. The compiler does not record any metadata about this change, and the operation is
not visible to other parts of the compiler. Therefore, any thunks emitted by the compiler to
facilitate crossing the language boundary do not check for a potential sign change.

#### This Crate's Approach

In considering various signed/unsigned conversion approaches for this crate, I evaluated
Foundation's and Swift's approach (transparent retyping) with Rust's [behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html).
Although Foundation's and Swift's approaches may lead to unexpected sign loss or wrapping, they are
not considered unsafe by Rust's definition. The only potentially related undefined behavior is
"Calling a function with the wrong call ABI," but signed-ness is not generally considered part of
the C ABI.

This crate introduces two traits to facilitate signed/unsigned conversion:

* `ExpectFrom` performs the signed/unsigned conversion and panics if the conversion fails.
  Implementations are a convenience wrapper for `<T as TryFrom>::try_from(value).expect("")`, which,
  while trivial, reduces the number of ad hoc `expect`s in bindings code to improve readability.
  This trait primarily facilitates conversions from idiomatic Rust types into the native Core
  Foundation types used by the foreign function interface. It provides a user-visible signal if
  `CFIndex` cannot represent an index or size.
* `FromUnchecked` performs the signed/unsigned conversion and assumes the result is correct,
  emulating the transparent retyping approach of Foundation and Swift. This trait primarily
  facilitates conversions from the native Core Foundation types used by foreign function interface
  into idiomatic Rust types where it's reasonable to assume the value is in bounds.

If a sign change goes undetected, safe Rust code will panic. Unsafe code must ensure all values are
in bounds for the given domain so an undetected sign change does not impose any additional burden,
assuming a sign change would cause the value to go out of bounds.

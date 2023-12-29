//! # Signed/Unsigned Conversion
//!
//! Core Foundation's canonical index type and size type, [`CFIndex`], is signed.  Foundation's
//! canonical index type and size type, `NSUInteger`, is unsigned, and Rust's canonical type,
//! [`usize`], is unsigned too.
//!
//! The following subsections describe Apple's implementation of signed/unsigned conversion between
//! Core Foundation and Foundation, Apple's implementation of signed/unsigned conversion between
//! Foundation and Swift, and this crate's approach to signed/unsigned conversion between Core
//! Foundation and Rust.
//!
//! ## Foundation's Approach
//!
//! Many Core Foundation and Foundation [types are interchangeable][]. `CFIndex` is used throughout
//! Core Foundation, while Foundation's equivalent interface uses `NSUInteger`. **Foundation
//! effectively uses an unchecked bit cast** when converting between `CFIndex` and `NSUInteger`—the
//! values pass through without detection of a potential wrap or sign loss. So, integer values with
//! the bit at `1 << sizeof(size_t) * 8 - 1` set to `1` will be negative in the Core Foundation
//! interface but positive in the Foundation interface.
//!
//! The only acknowledgment of the type mismatch in Apple's documentation (as far as I'm aware) is
//! this (slightly edited) description of the `location` and `length` fields on [`CFRange`][] and
//! [`NSRange`][]:
//!
//! > For type compatibility with the rest of the system, `LONG_MAX` is the maximum value you should
//! > use for location and length.
//!
//! An interesting related quirk is the difference between [`kCFNotFound`][] and [`NSNotFound`][].
//! Although many Core Foundation and Foundation types are interchangeable, the semantic "not found"
//! value is interface-dependent. `kCFNotFound` is defined as `-1` while `NSNotFound` is
//! `NSIntegerMax` (i.e., the maximum value of [`CFIndex`], `LONG_MAX`). So, the addressable range
//! of failable Foundation methods returning an [`NSRange`][] is effectively limited to
//! `[0, LONG_MAX)`.
//!
//! Using a signed type in the underlying Core Foundation implementation and `NSNotFound`'s
//! definition as the maximum signed value inhibit Foundation from utilizing the full range of the
//! unsigned type.
//!
//! ## Swift's Approach
//!
//! For Apple's frameworks (e.g., `AppKit`, `Foundation`, `UIKit`, etc.), the Swift compiler imports
//! `NSUInteger` as `Int`. To make C and Objective-C interfaces visible to Swift code, the Swift
//! compiler "imports" a [Clang module][] to build a Swift module, where "import" is a Swift
//! compiler process that constructs a Swift-native representation o the compatible declarations,
//! definitions, and types present in the Clang module.
//!
//! Normally, `NSUInteger` imports as `UInt`. But, in system modules (i.e., modules found under a
//! directory given by an `-isystem` flag), **Swift silently retypes `NSUInteger` to `Int`** unless
//! the `NSUInteger` declares the type of an enum. This retyping operation occurs during the
//! construction of the Swift module. The compiler does not record any metadata about this change,
//! and the operation is not visible to other parts of the compiler. Therefore, any thunks emitted
//! by the compiler to facilitate crossing the language boundary do not check for a potential sign
//! change.
//!
//! ## This Crate's Approach
//!
//! In considering various signed/unsigned conversion approaches for this crate, I evaluated
//! Foundation's and Swift's approach (transparent retyping) with Rust's [behavior considered
//! undefined]. Although Foundation's and Swift's approaches may lead to unexpected sign loss or
//! wrapping, they are not considered unsafe by Rust's definition. The only potentially related
//! undefined behavior is "Calling a function with the wrong call ABI," but signed-ness is not
//! generally considered part of the C ABI.
//!
//! This crate introduces two traits to facilitate signed/unsigned conversion:
//!
//! * [`ExpectFrom`] performs the signed/unsigned conversion and panics if the conversion fails.
//!   Implementations are a convenience wrapper for `<T as TryFrom>::try_from(value).expect("")`,
//!   which, while trivial, reduces the number of ad hoc `expect`s in bindings code to improve
//!   readability. This trait primarily facilitates conversions from idiomatic Rust types into the
//!   native Core Foundation types used by the foreign function interface. It provides a
//!   user-visible signal if `CFIndex` cannot represent an index or size.
//! * [`FromUnchecked`] performs the signed/unsigned conversion and assumes the result is correct,
//!   emulating the transparent retyping approach of Foundation and Swift. This trait primarily
//!   facilitates conversions from the native Core Foundation types used by foreign function interface
//!   into idiomatic Rust types where it's reasonable to assume the value is in bounds.
//!
//! If a sign change goes undetected, safe Rust code will panic. Unsafe code must ensure all values
//! are in bounds for the given domain so an undetected sign change does not impose any additional
//! burden, assuming a sign change would cause the value to go out of bounds.
//!
//! [`CFIndex`]: corefoundation_sys::CFIndex
//! [`CFRange`]: https://developer.apple.com/documentation/corefoundation/cfrange
//! [`NSNotFound`]: https://github.com/apple/swift-corelibs-foundation/blob/swift-5.9-RELEASE/Darwin/Foundation-swiftoverlay/Foundation.swift#L26
//! [`NSRange`]: https://developer.apple.com/documentation/foundation/nsrange/1459533-location
//! [`kCFNotFound`]: https://github.com/apple/swift-corelibs-foundation/blob/swift-5.9-RELEASE/CoreFoundation/Base.subproj/CFBase.h#L497
//! [Clang module]: https://clang.llvm.org/docs/Modules.html
//! [behavior considered undefined]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
//! [types are interchangeable]: https://developer.apple.com/library/archive/documentation/General/Conceptual/CocoaEncyclopedia/Toll-FreeBridgin/Toll-FreeBridgin.html

/// Performs a value-to-value conversion like [`TryFrom`] but assumes the caller has validated the
/// convert-from value so conversion will not fail.
///
/// This is usually implemented as a [`TryFrom`] convenience wrapper with a default expect message.
/// While trivial, this reduces the number of ad hoc `expect`s in bindings code to improves
/// readability.
pub trait ExpectFrom<T> {
    /// Performs the conversion.
    ///
    /// # Panics
    ///
    /// This associated function panics if conversion from `value` fails.
    fn expect_from(value: T) -> Self;
}

/// Performs a value-to-value conversion like [`TryFrom`] but assumes the caller has validated the
/// convert-from value so conversion is guaranteed to be correct.
///
/// This is usually implemented similarly to [`TryFrom`], but without validating the correctness of
/// the conversion, so is generally more performant than [`ExpectFrom`] or [`TryFrom`].
pub trait FromUnchecked<T> {
    /// Performs the conversion.
    ///
    /// # Safety
    ///
    /// At the time of this writing, implementations of this trait in this crate do not exhibit any
    /// [behavior considered undefined][], so this function is not unsafe. The only potentially
    /// related undefined behavior is "Calling a function with the wrong call ABI," but signed-ness
    /// (the class unchecked conversions for all implementations of this trait in this crate) is not
    /// generally considered to be part of the C ABI.
    ///
    /// If sign loss or wrapping occurs, safe code will panic. Unsafe code must ensure all values
    /// are in bounds for the given domain.
    ///
    /// For correct use of this trait, the caller must ensure conversion from `value` cannot fail
    /// for any reason. Consult the documentation on the implementing type for specific
    /// requirements.
    ///
    /// When in doubt, use [`ExpectFrom`] or [`TryFrom`].
    ///
    /// [behavior considered undefined]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
    fn from_unchecked(value: T) -> Self;
}

// LINT: `unreachable!()` is used where valid UTF-8 is expected from the compiler.
#![allow(clippy::unreachable)]

//! Support for emitting compile-time constant immutable [`String`]s that do not require heap
//! allocation. This uses the same ABI as Core Foundation's `CFSTR()` macro and Objective-C's
//! `NSString` literals (`@""`).
//!
//! **Note:** The functions in this module are intended to run only in a const context at compile
//! time; they **are not** intended to be used at run time. For runtime string construction, use one
//! of [`String`]'s associated functions.
//!
//! [`String`]: crate::string::String

use corefoundation_sys::{CFStringGetSurrogatePairForLongCharacter, Utf16CodePoint};

/// A compile-time constant immutable [`String`] that does not require heap allocation.
///
/// This structure has a well-defined [ABI][], as documented in the Clang source. Core Foundation
/// contains additional source documentation of [`CFRuntimeBase`][], including the use of the
/// [`flags` field][], the [`CFString` ID], and the [`CFString`-specific flags].
///
/// [`String`]: crate::string::String
/// [ABI]: https://github.com/llvm/llvm-project/blob/llvmorg-17.0.0/clang/lib/AST/ASTContext.cpp#L7319-L7326
/// [`CFRuntimeBase`]: https://github.com/apple/swift-corelibs-foundation/blob/swift-5.9-RELEASE/CoreFoundation/Base.subproj/CFRuntime.h#L216-L223
/// [`flags` field]: https://github.com/apple/swift-corelibs-foundation/blob/swift-5.9-RELEASE/CoreFoundation/Base.subproj/CFRuntime.c#L352-L369
/// [`CFString` ID]: https://github.com/apple/swift-corelibs-foundation/blob/swift-5.9-RELEASE/CoreFoundation/Base.subproj/CFRuntime_Internal.h#L23
/// [`CFString`-specific flags]: https://github.com/apple/swift-corelibs-foundation/blob/swift-5.9-RELEASE/CoreFoundation/String.subproj/CFString.c#L188-L218
#[doc(hidden)]
#[cfg(target_vendor = "apple")]
#[derive(Debug)]
#[repr(C)]
pub struct __NSConstantString {
    /// A pointer to the object's Objective-C metaclass instance.
    pub isa: *const i32,

    /// Bits 0..7 are reserved for type-specific use. Bits 0..20 contain the type ID. The remaining
    /// bits are used to track the retain count.
    pub flags: i32,

    /// A pointer to the string buffer.
    ///
    /// If the string is encoded using UTF-16, the pointer must be properly aligned.
    pub str: *const u8,

    /// If the string is encoded using ASCII, the number of bytes excluding the `NUL` terminator.
    /// Otherwise, if the string is encoding using UTF-16, the number of code units.
    pub length: isize,
}

/// The value of the [`__NSConstantString`] `flags` field when `str` points to a `NUL`-terminated
/// ASCII (8-bit) string.
///
/// * `b0`: `.isMutable = false`
/// * `b1`: (unused)
/// * `b2`: `.hasLengthByte = false`
/// * `b3`: `.hasNullByte = true`
/// * `b4`: `.isUnicode = false`
/// * `b5..=b6`: `.inlineContents = __kCFNotInlineContentsNoFree` (2)
/// * `b7`: `.usesDefaultAllocator = true`
/// * `b8..=b19`: `.typeID = _kCFRuntimeIDCFString` (7)
///
/// This value is also part of the well-defined [ABI][], as documented in the Clang source.
///
/// [ABI]: https://github.com/llvm/llvm-project/blob/llvmorg-17.0.0/clang/lib/CodeGen/CodeGenModule.cpp#L5969
#[doc(hidden)]
pub const _ASCII_FLAGS: i32 = 0b0000_0000_0111_1100_1000;

/// The value of the [`__NSConstantString`] `flags` field when `str` points to a UTF-16 string using
/// the host's native byte order.
///
/// * `b0`: `.isMutable = false`
/// * `b1`: (unused)
/// * `b2`: `.hasLengthByte = false`
/// * `b3`: `.hasNullByte = false`
/// * `b4`: `.isUnicode = true`
/// * `b5..=b6`: `.inlineContents = __kCFNotInlineContentsNoFree` (2)
/// * `b7`: `.usesDefaultAllocator = true`
/// * `b8..=b19`: `.typeID = _kCFRuntimeIDCFString` (7)
///
/// This value is also part of the well-defined [ABI][], as documented in the Clang source.
///
/// [ABI]: https://github.com/llvm/llvm-project/blob/llvmorg-17.0.0/clang/lib/CodeGen/CodeGenModule.cpp#L5969
#[doc(hidden)]
pub const _UTF16_FLAGS: i32 = 0b0000_0000_0111_1101_0000;

// SAFETY: Core Foundation guarantees it's safe to send constant strings across threads.
unsafe impl Send for __NSConstantString {}

// SAFETY: Core Foundation guarantees it's safe to share constant strings between threads.
unsafe impl Sync for __NSConstantString {}

extern "C" {
    /// The well-known symbol used for the constant string's `isa` pointer.
    #[doc(hidden)]
    pub static __CFConstantStringClassReference: i32;
}

/// Returns `true` if `s` exclusively contains non-`NUL` ASCII code points, enabling the Core
/// Foundation constant string to the 8-bit ASCII representation.
#[doc(hidden)]
#[inline]
#[must_use]
pub const fn _is_ascii_with_no_nul(s: &str) -> bool {
    let bytes = s.as_bytes();

    let mut i = 0;
    let len = bytes.len();

    while i < len {
        // PANIC: `i` is always in bounds given the loop condition.
        #[allow(clippy::indexing_slicing)]
        let byte = bytes[i];
        if byte == 0 || byte > 127 {
            return false;
        }

        i = match i.checked_add(1) {
            Some(i) => i,
            None => unreachable!(), // `i + 1` cannot overflow because it's less than `len`
        };
    }

    true
}

/// Copies the bytes of `s` into a `NUL`-terminated array.
///
/// # Panics
///
/// Panics if `N` is not equal to `s.as_bytes().len() + 1`, as this function's only purpose is to
/// transform `s` into a zero-terminated array of bytes.
#[allow(clippy::indexing_slicing)] // see comments at indexing use sites
#[doc(hidden)]
#[inline]
#[must_use]
pub const fn _ascii_code_points<const N: usize>(s: &str) -> [u8; N] {
    let mut code_units = [0_u8; N];

    let bytes = s.as_bytes();

    let mut i = 0;
    let len = bytes.len();

    while i < len {
        // PANIC: Panic if `code_units[i]` is out of bounds so that compilation in a const context
        // fails due to the caller's logic error.
        // PANIC: `bytes[i]` is always in bounds given the loop condition.
        code_units[i] = bytes[i];

        i = match i.checked_add(1) {
            Some(i) => i,
            None => unreachable!(), // `i + 1` cannot overflow because it's less than `len`
        };
    }

    // PANIC: Panic if `code_units[i]` is out of bounds so that compilation in a const context fails
    // due to the caller's logic error.
    code_units[i] = 0;

    match i.checked_add(1) {
        Some(i) if i == N => {}
        _ => panic!("N exceeds the C string's length"),
    };

    code_units
}

/// Transcodes `s` into a `0`-terminated array of UTF-16 code units.
///
/// # Panics
///
/// Panics if `N` is not equal to `s.encode_utf16().count() + 1`, as this function's only purpose is
/// to transform `s` into a zero-terminated array of UTF-16 code units.
#[allow(clippy::indexing_slicing)] // see comments at indexing use sites
#[doc(hidden)]
#[inline]
#[must_use]
pub const fn _utf16_code_units<const N: usize>(s: &str) -> [u16; N] {
    let mut code_units = [0_u16; N];

    let mut c: u32 = 0;
    let mut remaining: u8 = 0;
    let mut next_code_unit = 0;

    let bytes = s.as_bytes();

    let mut i = 0;
    let len = bytes.len();

    while i < len {
        // PANIC: `i` is always in bounds given the loop condition.
        let byte: u8 = bytes[i];

        // LINT: The following `as` conversions only expand the number of unsigned bits. No
        // alternative exists for const contexts.
        #[allow(clippy::as_conversions)]
        if byte & 0x80 == 0x00 {
            // U+0000..=U+007F
            c = byte as u32;
            assert!(remaining == 0, "UTF-8 to UTF-16 transcode logic error");
        } else if byte & 0xe0 == 0xc0 {
            // U+0080..=U+07FF
            c = (byte & 0x1f) as u32;
            remaining = 1;
        } else if byte & 0xf0 == 0xe0 {
            // U+0800..=U+FFFF
            c = (byte & 0x0f) as u32;
            remaining = 2;
        } else if byte & 0xf8 == 0xf0 {
            // U+10000..=U+10FFFF
            c = (byte & 0x07) as u32;
            remaining = 3;
        } else if byte & 0xc0 == 0x80 {
            c <<= 6_i32;
            c |= (byte & 0x3f) as u32;

            remaining = match remaining.checked_sub(1) {
                Some(remaining) => remaining,
                None => panic!("UTF-8 to UTF-16 transcode logic error"),
            }
        } else {
            panic!("invalid UTF-8 code unit")
        };

        if remaining == 0 {
            match CFStringGetSurrogatePairForLongCharacter(c) {
                // PANIC: Panic if `code_units[next_code_unit]` is out of bounds so that compilation
                // in a const context fails due to the caller's logic error.
                Utf16CodePoint::Basic(c) => code_units[next_code_unit] = c,
                Utf16CodePoint::Supplementary { high, low } => {
                    // PANIC: Panic if `code_units[next_code_unit]` is out of bounds so that
                    // compilation in a const context fails due to the caller's logic error.
                    code_units[next_code_unit] = high;
                    next_code_unit = match next_code_unit.checked_add(1) {
                        Some(next_code_unit) => next_code_unit,
                        None => unreachable!(), // next_code_unit + 1 cannot overflow because it's less than N
                    };
                    // PANIC: Panic if `code_units[next_code_unit]` is out of bounds so that
                    // compilation in a const context fails due to the caller's logic error.
                    code_units[next_code_unit] = low;
                }
            }

            next_code_unit = match next_code_unit.checked_add(1) {
                Some(next_code_unit) => next_code_unit,
                None => unreachable!(), // `next_code_unit + 1` cannot overflow because it's less than `N`
            };
        }

        i = match i.checked_add(1) {
            Some(i) => i,
            None => unreachable!(), // `i + 1` cannot overflow because it's less than `len`
        };
    }

    // PANIC: Panic if `code_units[next_code_unit]` is out of bounds so that compilation in a const
    // context fails due to the caller's logic error.
    code_units[next_code_unit] = 0;

    match next_code_unit.checked_add(1) {
        Some(next_code_unit) if next_code_unit == N => {}
        _ => panic!("`N exceeds the zero terminated UTF-16 string's length"),
    };

    code_units
}

/// Returns the number of UTF-16 code units required to encode `s` using UTF-16.
// LINT: This function doesn't need "Panics" documentation because the panic should never be hit or
// visible to the caller.
#[allow(clippy::missing_panics_doc)]
#[doc(hidden)]
#[inline]
#[must_use]
pub const fn _utf16_len(s: &str) -> usize {
    let mut utf16_len: usize = 0;

    let bytes = s.as_bytes();

    let mut i = 0;
    let len = bytes.len();

    while i < len {
        // PANIC: `i` is always in bounds given the loop condition.
        #[allow(clippy::indexing_slicing)]
        let byte: u8 = bytes[i];

        let (c_utf8_len, c_utf16_len) = if byte & 0x80 == 0x00 {
            (1, 1) // U+0000..=U+007F
        } else if byte & 0xe0 == 0xc0 {
            (2, 1) // U+0080..=U+07FF
        } else if byte & 0xf0 == 0xe0 {
            (3, 1) // U+0800..=U+FFFF
        } else if byte & 0xf8 == 0xf0 {
            (4, 2) // U+100000..=U+10FFFF
        } else {
            panic!("UTF-8 decode logic error");
        };

        i = match i.checked_add(c_utf8_len) {
            Some(i) => i,
            None => unreachable!(), // `i + 1` cannot overflow because it's less than `len`
        };

        utf16_len = match utf16_len.checked_add(c_utf16_len) {
            Some(utf16_len) => utf16_len,
            None => unreachable!(), // cannot overflow because `utf16_len` is always less than or equal to `len`
        }
    }

    utf16_len
}

/// Creates a compile-time constant immutable [`String`] from a string literal.
///
/// [`String`]: crate::string::String
#[macro_export]
macro_rules! cfstr {
    ($value:literal) => {{
        const IS_ASCII: bool = $crate::string::constant::_is_ascii_with_no_nul($value);

        const ASCII_LEN: usize = $value.len();
        const UTF16_LEN: usize = $crate::string::constant::_utf16_len($value);

        #[link_section = "__TEXT,__cstring,cstring_literals"]
        static UTF8: [u8; ASCII_LEN + 1] = $crate::string::constant::_ascii_code_points($value);

        // Although the flags `hasNullByte` is `false`, Clang always appends a `0_u16`.
        // https://github.com/llvm/llvm-project/blob/llvmorg-17.0.0/clang/lib/CodeGen/CodeGenModule.cpp#L5856-L5861
        #[link_section = "__TEXT,__ustring"]
        static UTF16: [u16; UTF16_LEN + 1] = $crate::string::constant::_utf16_code_units($value);

        // LINT: `ASCII_LEN as isize` and `UTF16_LEN as isize` will not wrap given the assert below.
        #[allow(clippy::as_conversions)]
        #[link_section = "__DATA,__cfstring"]
        static STRING: $crate::string::constant::__NSConstantString = if IS_ASCII {
            $crate::string::constant::__NSConstantString {
                // SAFETY: This pointer is used only by Core Foundation and the Objective-C runtime,
                // which correctly handle aliasing and concurrency.
                isa: unsafe { &$crate::string::constant::__CFConstantStringClassReference },
                flags: $crate::string::constant::_ASCII_FLAGS,
                str: UTF8.as_ptr(),
                length: ASCII_LEN as isize,
            }
        } else {
            $crate::string::constant::__NSConstantString {
                // SAFETY: This pointer is used only by Core Foundation and the Objective-C runtime,
                // which correctly handle aliasing and concurrency.
                isa: unsafe { &$crate::string::constant::__CFConstantStringClassReference },
                flags: $crate::string::constant::_UTF16_FLAGS,
                str: UTF16.as_ptr().cast(),
                length: UTF16_LEN as isize,
            }
        };

        // LINT: [`isize::MAX`] is a non-negative value and therefore safe to cast to [`usize`].
        #[allow(clippy::as_conversions)]
        if (ASCII_LEN > isize::MAX as usize || UTF16_LEN > isize::MAX as usize) {
            panic!("The string literal is too long to represent with CFString.");
        }

        let string: *const _ = &STRING;

        // SAFETY: `&String`'s bit representation is the [`CFStringRef`] pointer value. `&STRING` is
        // a reference to a struct that is layout-compatible with [`CFString`]. Therefore, this cast
        // to the unrelated type is safe.
        unsafe { &*string.cast::<$crate::string::String>() }
    }};
}

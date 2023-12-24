use crate::{Boolean, CFAllocatorRef, CFIndex, CFRange, CFStringRef, UInt8, UTF32Char, UniChar};
use core::ffi::c_char;

/// Identifier for character encoding; the values are the same as Text Encoding Converter
/// `TextEncoding`.
pub type CFStringEncoding = u32;

pub const kCFStringEncodingInvalidId: CFStringEncoding = 0xffff_ffff;

/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingMacRoman: CFStringEncoding = 0;
/// MS-DOS & Windows ANSI codepage 1252
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingWindowsLatin1: CFStringEncoding = 0x0500;
/// ISO 8859-1
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingISOLatin1: CFStringEncoding = 0x0201;
/// NeXTSTEP encoding
/// Platform-independent built-in encoding; always available on all platforms.
#[allow(clippy::doc_markdown)] // LINT: Casing is due to branding. It's not referring to an item.
pub const kCFStringEncodingNextStepLatin: CFStringEncoding = 0x0b01;
/// 0..127 (in creating `CFString`, values greater than 0x7F are treated as corresponding Unicode
/// value)
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingASCII: CFStringEncoding = 0x0600;
/// `kTextEncodingUnicodeDefault + kUnicodeUTF8Format`
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingUTF8: CFStringEncoding = 0x0800_0100;
/// 7bit Unicode variants used by Cocoa & Java
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingNonLossyASCII: CFStringEncoding = 0x0bff;

/// `kTextEncodingUnicodeDefault + kUnicodeUTF16Format`
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingUTF16: CFStringEncoding = 0x0100;
/// `kTextEncodingUnicodeDefault + kUnicodeUTF16BEFormat`
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingUTF16BE: CFStringEncoding = 0x1000_0100;
/// `kTextEncodingUnicodeDefault + kUnicodeUTF16LEFormat`
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingUTF16LE: CFStringEncoding = 0x1400_0100;

/// `kTextEncodingUnicodeDefault + kUnicodeUTF32Format`
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingUTF32: CFStringEncoding = 0x0c00_0100;
/// `kTextEncodingUnicodeDefault + kUnicodeUTF32BEFormat`
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingUTF32BE: CFStringEncoding = 0x1800_0100;
/// `kTextEncodingUnicodeDefault + kUnicodeUTF32LEFormat`
/// Platform-independent built-in encoding; always available on all platforms.
pub const kCFStringEncodingUTF32LE: CFStringEncoding = 0x1c00_0100;

extern "C" {
    /// Takes an explicit length, and allows you to specify whether the data is an external
    /// format—that is, whether to pay attention to the BOM character (if any) and do byte swapping
    /// if necessary.
    ///
    /// Copies the provided buffer into `CFString`'s internal storage.
    pub fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const UInt8,
        numBytes: CFIndex,
        encoding: CFStringEncoding,
        isExternalRepresentation: Boolean,
    ) -> CFStringRef;

    /// Copies the provided buffer into `CFString`'s internal storage.
    pub fn CFStringCreateWithCharacters(
        alloc: CFAllocatorRef,
        chars: *const UniChar,
        numChars: CFIndex,
    ) -> CFStringRef;

    /// Number of 16-bit Unicode characters in the string.
    pub fn CFStringGetLength(theString: CFStringRef) -> CFIndex;

    /// Extracting the contents of the string. For obtaining multiple characters, calling
    /// [`CFStringGetCharacters`] is more efficient than multiple calls to
    /// `CFStringGetCharacterAtIndex`.
    ///
    /// If the length of the string is not known (so you can't use a fixed size buffer for
    /// [`CFStringGetCharacters`]), another method is to use is
    /// `CFStringGetCharacterFromInlineBuffer`.
    pub fn CFStringGetCharacterAtIndex(theString: CFStringRef, idx: CFIndex) -> UniChar;

    pub fn CFStringGetCharacters(theString: CFStringRef, range: CFRange, buffer: *mut UniChar);

    /// May return `NULL` at any time; be prepared for `NULL`, if not now, in some other time or
    /// place.
    pub fn CFStringGetCStringPtr(
        theString: CFStringRef,
        encoding: CFStringEncoding,
    ) -> *const c_char;

    /// The primitive conversion routine; allows you to convert a string piece at a time into a
    /// fixed size buffer. Returns number of characters converted.
    ///
    /// Characters that cannot be converted to the specified encoding are represented with the byte
    /// specified by `lossByte`; if `lossByte` is `0`, then lossy conversion is not allowed and
    /// conversion stops, returning partial results.
    ///
    /// Pass `buffer==NULL` if you don't care about the converted string (but just the
    /// convertability, or number of bytes required).
    ///
    /// `maxBufLength` indicates the maximum number of bytes to generate. It is ignored when
    /// `buffer==NULL`.
    ///
    /// Does not zero-terminate. If you want to create Pascal or C string, allow one extra byte at
    /// start or end.
    ///
    /// Setting `isExternalRepresentation` causes any extra bytes that would allow the data to be
    /// made persistent to be included; for instance, the Unicode BOM. Note that CFString prepends
    /// UTF encoded data with the Unicode BOM <http://www.unicode.org/faq/utf_bom.html>  when
    /// generating external representation if the target encoding allows. It's important to note
    /// that only UTF-8, UTF-16, and UTF-32 define the handling of the byte order mark character,
    /// and the "LE" and "BE" variants of UTF-16 and UTF-32 don't.
    pub fn CFStringGetBytes(
        theString: CFStringRef,
        range: CFRange,
        encoding: CFStringEncoding,
        lossByte: u8,
        isExternalRepresentation: Boolean,
        buffer: *mut u8,
        maxBufLen: CFIndex,
        usedBufLen: *mut CFIndex,
    ) -> CFIndex;
}

#[inline]
#[must_use]
pub const fn CFStringIsSurrogateHighCharacter(character: UniChar) -> bool {
    character >= 0xd800 && character <= 0xdbff
}

#[inline]
#[must_use]
pub const fn CFStringIsSurrogateLowCharacter(character: UniChar) -> bool {
    character >= 0xdc00 && character <= 0xdfff
}

// LINT: There are no side effects when using correct values, which is the caller's responsibility.
#[allow(clippy::arithmetic_side_effects)]
#[inline]
#[must_use]
pub const fn CFStringGetLongCharacterForSurrogatePair(
    surrogateHigh: UniChar,
    surrogateLow: UniChar,
) -> UTF32Char {
    // LINT: As as conversion is the only up-cast available in a const context.
    #[allow(clippy::as_conversions)]
    let high = surrogateHigh as u32;
    // LINT: As as conversion is the only up-cast available in a const context.
    #[allow(clippy::as_conversions)]
    let low = surrogateLow as u32;

    ((high - 0xd800) << 10_u32) + (low - 0xdc00) + 0x01_0000
}

/// Maps a UTF-32 character to a pair of UTF-16 surrogate characters.
///
/// **Note:** This function differs from the original C function. It returns an ad hoc type,
/// [`Utf16CodePoint`], instead of writing the output to a user-supplied buffer so the function can
/// be used in a const context.
#[inline]
#[must_use]
pub const fn CFStringGetSurrogatePairForLongCharacter(mut character: UTF32Char) -> Utf16CodePoint {
    // LINT: This function's purpose is to perform a 32-bit to 16-bit conversion, and its
    // implementation does not truncate any valid code point.
    #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
    // LINT: The arithmetic is performed on bounded integers and does not have side effects.
    #[allow(clippy::arithmetic_side_effects)]
    if character > 0xffff && character < 0x11_0000 {
        // Non-BMP character
        character -= 0x10000;
        Utf16CodePoint::Supplementary {
            high: ((character >> 10_u32) as u16) + 0xd800,
            low: ((character & 0x3ff_u32) as u16) + 0xdc00,
        }
    } else {
        Utf16CodePoint::Basic(character as u16)
    }
}

/// The UTF-16 encoding of a single code point.
#[derive(Clone, Copy, Debug)]
pub enum Utf16CodePoint {
    /// The code point lies in the Basic Multilingual Plane, which uses a single word.
    Basic(u16),

    /// The code point lies in a supplementary plane, which uses a pair of words.
    Supplementary {
        /// The first of the two code units to form the code point.
        high: u16,
        /// The second of the two code units to form the code point.
        low: u16,
    },
}

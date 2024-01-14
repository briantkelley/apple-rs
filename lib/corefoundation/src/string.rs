//! A UTF-16–encoded string, instances of which may be read-only or mutable.

use crate::define_and_impl_type;
use crate::ffi::convert::{ExpectFrom, FromUnchecked};
use crate::ffi::ForeignFunctionInterface;
use crate::sync::Arc;
use core::ffi::CStr;
use core::fmt::{self, Display, Formatter};
use core::mem::size_of;
use core::num::NonZeroU8;
use core::ops::{Range, RangeBounds};
use core::ptr;
use core::slice;
use core::str;
use corefoundation_sys::{
    kCFAllocatorDefault, kCFStringEncodingNonLossyASCII, kCFStringEncodingUTF16,
    kCFStringEncodingUTF16BE, kCFStringEncodingUTF16LE, kCFStringEncodingUTF32,
    kCFStringEncodingUTF32BE, kCFStringEncodingUTF32LE, kCFStringEncodingUTF8, CFIndex, CFRange,
    CFStringCreateWithBytes, CFStringEncoding, CFStringGetBytes, CFStringGetCStringPtr,
    CFStringGetCharacterAtIndex, CFStringGetLength, CFStringGetLongCharacterForSurrogatePair,
    CFStringIsSurrogateHighCharacter, CFStringIsSurrogateLowCharacter, __CFString,
};

mod character_set;
#[doc(hidden)]
pub mod constant;
#[allow(clippy::module_name_repetitions)]
mod reader;
#[cfg(test)]
mod tests;

pub use character_set::CharacterSet;
pub use reader::{
    GetBytesLossyReader, GetBytesReader, GetBytesReaderResult, GetBytesReaderSummary,
    GetBytesStrReader, GetBytesStrReplacement,
};

define_and_impl_type!(
    /// An abstract interface for working with a logically contiguous sequence of UTF-16 code units.
    ///
    /// The internal encoding may not be UTF-16, and the internal storage may not be contiguous.
    String,
    raw: __CFString
);

/// Specifies the byte order used to encode UTF-16 code units or UTF-32 code points.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FromUtfByteOrder {
    /// The UTF-16 code units or UTF-32 code points use the big endian byte order.
    ///
    /// If the first value is a byte order mark (BOM), it will be treated as a code point and
    /// included as part of the string's content.
    BigEndian,

    /// The UTF-16 code units or UTF-32 code points are prefixed with a byte order mark (BOM)
    /// indicating whether the code units use big endian or little endian byte order.
    ///
    /// If a byte order mark is not present, the first code unit will be treated as a code point.
    /// Also, on little endian platforms, Core Foundation will byte swap all code units.
    ByteOrderMark,

    /// The UTF-16 code units or UTF-32 code points use the host's native byte order.
    ///
    /// For UTF-16 code units, if the first value is a byte order mark (BOM), it will be treated as
    /// a code point and included as part of the string's content.
    HostNative,

    /// The UTF-16 code units or UTF-32 code points use the little endian byte order.
    ///
    /// If the first value is a byte order mark (BOM), it will be treated as a code point and
    /// included as part of the string's content.
    LittleEndian,
}

/// Indicates an error when creating a [`String`] from an array of bytes through
/// [`String::from_bytes`].
// LINT: [`Clone`] and [`Copy`] are not implemented on similar standard library types.
#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct FromBytesError(());

/// Indicates an error when creating a [`String`] from an array of UTF-8 code units through
/// [`String::from_utf8`].
// LINT: [`Clone`] and [`Copy`] are not implemented on similar standard library types.
#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct FromUtf8Error(());

/// Indicates an error when creating a [`String`] from an array of UTF-32 code points through
/// [`String::from_utf32`].
// LINT: [`Clone`] and [`Copy`] are not implemented on similar standard library types.
#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub struct FromUtf32Error(());

// Note: The [`CFStringCreateWithBytes`] `lossByte` and `isExternalRepresentation` arguments are not
// directly exposed through these bindings.
//
// `lossByte` is used only used for UTF-32 and non-Unicode encodings. Core Foundation does not have
// a concept of lossy conversion to UTF-16 (it allows surrogate pairs to be split) or to UTF-8 (it
// just stops if conversion cannot proceed).
//
// `isExternalRepresentation` is only used by Core Foundation for UTF-16 and UTF-32 host native byte
// order. Core Foundation **does not** write the UTF-8 BOM nor does it relay the flag to ICU when
// converting to a non-Unicode encoding.
//
// So, these bindings provide define an ad hoc [`GetBytesEncoding`] type so the interface doesn't
// expose configuration options that are not implemented for these key encodings.

/// Specifies the byte order of 16-bit or 32-bit Unicode values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetBytesByteOrder {
    /// The UTF-16 or UTF-32 code units will be written to `buf` using the big endian byte order.
    BigEndian,

    /// The UTF-16 or UTF-32 code points will be written to `buf` in the host's native byte order.
    HostNative {
        /// If `true`, the conversion will be written to `buf` in an "external representation"
        /// format, which contains a byte order marker (BOM) specifying endian-ness.
        include_bom: bool,
    },

    /// The UTF-16 or UTF-32 code units will be written to `buf` using the little endian byte order.
    LittleEndian,
}

/// The character encoding to use when fetching code units from a [`String`] into a byte `buf`fer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetBytesEncoding {
    /// An encoding that is a subset of the Unicode Transformation Format.
    CharacterSet {
        /// The character set encoding scheme into which to convert the [`String`].
        character_set: CharacterSet,

        /// A character (for example, `b'?'`) that should be substituted for characters that cannot
        /// be converted to the specified `encoding`. Pass [`None`] if you do not want lossy
        /// conversion to occur.
        ///
        /// **Note:** Core Foundation will process surrogate pairs as two individual lossy code
        /// points, so the number of output code points will equal the number of input code units.
        loss_byte: Option<NonZeroU8>,
    },

    /// Unicode Transform Format 8-bit variable-width encoding.
    Utf8,

    /// Unicode Transform Format 16-bit variable-width encoding.
    Utf16 {
        /// Specifies the byte order of the 16-bit code units.
        byte_order: GetBytesByteOrder,
    },

    /// Unicode Transform Format 32-bit fixed-width encoding.
    Utf32 {
        /// Specifies the byte order of the 32-bit code points.
        byte_order: GetBytesByteOrder,

        /// A character (for example, `b'?'`) that should be substituted for an unpaired surrogate
        /// code unit. Pass [`None`] if you do not want lossy conversion to occur.
        loss_byte: Option<NonZeroU8>,
    },
}

/// Returned by [`String::get_bytes`] if a code unit the specified `range` could not be converted
/// into `encoding`.
///
/// Information about any code units successfully converted is available in `result`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetBytesError {
    /// Information about the code unit that failed to convert into `encoding`.
    kind: GetBytesErrorKind,

    /// The result of the conversion of the input range up to the start of the code unit that
    /// failed to convert into `encoding`.
    result: GetBytesResult,
}

/// Returned by [`String::get_bytes`] to indicate why a code unit failed to convert into `encoding`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GetBytesErrorKind {
    /// A code point cannot be represented in the target `encoding`.
    Character {
        /// The character that could not be converted.
        c: char,

        /// The range of the code point that could not be converted.
        ///
        /// The code point may be part of a surrogate pair, so the location of `c` is specified as a
        /// range and an index, though it may have a length of `1`.
        range: Range<usize>,
    },

    /// Conversion failed due to an invalid surrogate code unit.
    Surrogate {
        /// Information about why the surrogate code unit failed to convert into `encoding`.
        reason: GetBytesSurrogateError,

        /// The index of the surrogate code unit that failed to convert into `encoding`.
        index: usize,
    },
}

/// Returned by [`String::get_bytes_unchecked`], and [`String::get_bytes`] to indicate conversion
/// progress and output.
#[derive(Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct GetBytesResult {
    /// If an output buffer was provided, the number of bytes written into the buffer. Otherwise,
    /// the number of bytes required to convert `range` into `encoding`.
    pub buf_len: usize,

    /// The code units remaining to be converted. If all of `range` was successfully converted, the
    /// field is [`None`].
    ///
    /// Call the method again with this range to continue conversion. If a code unit could not be
    /// converted:
    ///
    /// * [`String::get_bytes`] sets the `start` of the `remaining` range to the code unit *after*
    ///   the one that could not be converted.
    /// * [`String::get_bytes_unchecked`] sets the `start` of the `remaining` range to the code unit
    ///   that could not be converted. Hence, its "unchecked" suffix.
    pub remaining: Option<Range<usize>>,
}

/// Returned by [`String::get_bytes`] to indicate why a surrogate code unit failed to convert into
/// `encoding`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GetBytesSurrogateError {
    /// The input `range` splits a surrogate pair. The start of`range` must include the high
    /// surrogate code unit and the end of `range` must include the low surrogate code unit.
    Range,

    /// The underlying string representation is invalid UTF-16. A surrogate code unit is not part of
    /// a surrogate pair.
    Unpaired,
}

/// Identifies the position of a code unit in a surrogate pair.
///
/// In UTF-16, code points with a scalar value of `U+10000` or higher are encoded using two code
/// units, which, together, form a surrogate pair. Both code units are required to encode the code
/// point.
#[derive(Clone, Copy, Debug)]
pub enum SurrogateHalf {
    /// A code unit in the "high surrogate" range (`U+D800..=U+DBFF`). The high surrogate always
    /// precedes the low surrogate.
    ///
    /// The code unit represents the code point's high 11 bits (10-20, inclusive), which are
    /// computed from the code unit's value as `0x10000 + ((value - 0xd800) << 10)`.
    High,

    /// A code unit in the "low surrogate" range (`U+DC00..=U+DFFF`). The low surrogate always
    /// follows the high surrogate.
    ///
    /// The code unit represents the code point's low 10 bits (0-9, inclusive), which are computed
    /// from the code unit's value as `value - 0xdc00`.
    Low,
}

// SAFETY: Core Foundation allows transferring ownership of strings across threads.
unsafe impl Send for String {}

// SAFETY: Core Foundation allows sharing strings across threads as long as any mutations are
// performed with exclusive access (which is guaranteed by the Rust type system).
unsafe impl Sync for String {}

impl String {
    /// Returns a [`String`] object initialized by copying the code points encoded using
    /// `character_set` from the byte slice.
    ///
    /// # Errors
    ///
    /// Returns a [`FromBytesError`] if `bytes` contains an invalid sequence for `character_set`.
    #[inline]
    pub fn from_bytes(
        bytes: impl AsRef<[u8]>,
        character_set: CharacterSet,
    ) -> Result<Arc<Self>, FromBytesError> {
        Self::from_bytes_inner(bytes.as_ref(), character_set.into(), false)
    }

    fn from_bytes_inner(
        bytes: &[u8],
        encoding: CFStringEncoding,
        is_external_representation: bool,
    ) -> Result<Arc<Self>, FromBytesError> {
        let buf = bytes.as_ptr();
        // UB: A slice's length cannot exceed [`isize::MAX`]. The upper bound of addressable memory
        // all current 64-bit machines is less than 64-bit. Apple no longer supports any 32-bit
        // platforms (where > 2 GiB allocations are possible but unlikely).
        let len = CFIndex::from_unchecked(bytes.len());
        let is_external_representation = is_external_representation.into();

        // SAFETY: `len` is the correct size of `buf`, and `buf` is a valid pointer.
        let cf = unsafe {
            CFStringCreateWithBytes(
                kCFAllocatorDefault,
                buf,
                len,
                encoding,
                is_external_representation,
            )
        };

        // SAFETY: The [`CFStringRef`] was just created so it's an exclusive pointer, it has a
        // retain that must be released, and [`String`] is a correct [`CFType`] implementation.
        unsafe { Self::try_from_create_rule(cf) }.ok_or(FromBytesError(()))
    }

    /// Returns a [`String`] object initialized by copying the UTF-8 code units from the string
    /// slice.
    #[inline]
    // LINT: Unlike [`FromStr`], this method is infallible.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: impl AsRef<str>) -> Arc<Self> {
        fn inner(s: &str) -> Arc<String> {
            let bytes = s.as_bytes();
            let string = String::from_bytes_inner(bytes, kCFStringEncodingUTF8, false);

            // [`CFStringCreateWithBytes`] returns `NULL` for UTF-8 encoded bytes when:
            //
            // 1. The bytes are not valid UTF-8. This case is not possible here because [`str`] is
            //    guaranteed to be valid UTF-8.
            // 2. The Core Foundation allocator returns `NULL`, inhibiting instantiation of the
            //    object. If creation fails, it is assumed this is what happened here.
            string.unwrap_or_else(|_| alloc_error(s.encode_utf16().count()))
        }
        inner(s.as_ref())
    }

    /// Returns a [`String`] object initialized by copying the UTF-8 code units from the byte slice.
    ///
    /// # Errors
    ///
    /// Returns a [`FromUtf8Error`] if `code_units` contains an invalid encoding of a code point.
    #[inline]
    pub fn from_utf8(code_units: impl AsRef<[u8]>) -> Result<Arc<Self>, FromUtf8Error> {
        fn inner(bytes: &[u8]) -> Result<Arc<String>, FromUtf8Error> {
            String::from_bytes_inner(bytes, kCFStringEncodingUTF8, false)
                .map_err(|_| FromUtf8Error(()))
        }
        inner(code_units.as_ref())
    }

    /// Returns a [`String`] object initialized by copying the UTF-16 code units encoded in
    /// `byte_order` from the [`u16`] slice.
    ///
    /// **Note:** Core Foundation does **not** validate the code points and will initialize the
    /// string with invalid surrogates.
    #[inline]
    pub fn from_utf16(code_units: impl AsRef<[u16]>, byte_order: FromUtfByteOrder) -> Arc<Self> {
        fn inner(code_units: &[u16], byte_order: FromUtfByteOrder) -> Arc<String> {
            // When the specified byte order matches the target architecture, generalize to the
            // host native encoding, which has an optimized path in Core Foundation.
            let (encoding, is_external_representation) = match byte_order {
                #[cfg(target_endian = "big")]
                FromUtfByteOrder::BigEndian => (kCFStringEncodingUTF16, false),
                #[cfg(target_endian = "little")]
                FromUtfByteOrder::BigEndian => (kCFStringEncodingUTF16BE, false),
                FromUtfByteOrder::ByteOrderMark => (kCFStringEncodingUTF16, true),
                FromUtfByteOrder::HostNative => (kCFStringEncodingUTF16, false),
                #[cfg(target_endian = "big")]
                FromUtfByteOrder::LittleEndian => (kCFStringEncodingUTF16LE, false),
                #[cfg(target_endian = "little")]
                FromUtfByteOrder::LittleEndian => (kCFStringEncodingUTF16, false),
            };

            // [`CFStringCreateWithBytes`] returns `NULL` for UTF-16 encoded bytes only when the
            // Core Foundation allocator returns `NULL`, inhibiting instantiation of the object.
            String::from_bytes_inner(as_bytes(code_units), encoding, is_external_representation)
                .unwrap_or_else(|_| alloc_error(code_units.len()))
        }
        inner(code_units.as_ref(), byte_order)
    }

    /// Returns a [`String`] object initialized by copying the UTF-32 code points encoded in
    /// `byte_order` from the [`u32`] slice.
    ///
    /// # Errors
    ///
    /// Returns a [`FromUtf32Error`] if `code_points` contains a value that is not a Unicode scalar
    /// (i.e., greater than `0x10FFFF`).
    #[inline]
    pub fn from_utf32(
        code_points: impl AsRef<[u32]>,
        byte_order: FromUtfByteOrder,
    ) -> Result<Arc<Self>, FromUtf32Error> {
        fn inner(
            code_points: &[u32],
            byte_order: FromUtfByteOrder,
        ) -> Result<Arc<String>, FromUtf32Error> {
            // When the byte order matches the target architecture, use the encoding with the
            // explicit byte order. `kCFStringEncodingUTF32` *always* implies use of a BOM.
            let encoding = match byte_order {
                FromUtfByteOrder::BigEndian => kCFStringEncodingUTF32BE,
                FromUtfByteOrder::ByteOrderMark => kCFStringEncodingUTF32,
                #[cfg(target_endian = "big")]
                FromUtfByteOrder::HostNative => kCFStringEncodingUTF32BE,
                #[cfg(target_endian = "little")]
                FromUtfByteOrder::HostNative => kCFStringEncodingUTF32LE,
                FromUtfByteOrder::LittleEndian => kCFStringEncodingUTF32LE,
            };

            String::from_bytes_inner(as_bytes(code_points), encoding, false)
                .map_err(|_| FromUtf32Error(()))
        }
        inner(code_points.as_ref(), byte_order)
    }

    /// Returns the entire `String` as a Rust [`String`] slice.
    ///
    /// **Important:** This may allocate a temporary [`String`]. Consider using
    /// `<alloc::string::String as From<&corefoundation::string::String>>::from` or
    /// [`alloc::borrow::Cow::into_owned`] to obtain a persistent UTF-8 encoded equivalent.
    ///
    /// [`String`]: alloc::string::String
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn as_str(&self) -> alloc::borrow::Cow<'_, str> {
        use alloc::borrow::Cow;

        self.try_as_str()
            .map_or_else(|| Cow::Owned(self.into()), Cow::Borrowed)
    }

    /// Fetches a range of the code points from the string, converts the code points to `encoding`,
    /// and writes the result into the byte `buf`fer.
    ///
    /// Arguments:
    ///
    /// * `range` specifies the indices of the UTF-16 code units to process. If `range` divides a
    ///   surrogate pair, the surrogate code unit will be processed as a lossy conversion (except
    ///   for UTF-16 encodings, which are always lossless and UTF-8 conversions, which will always
    ///   fail).
    /// * `encoding` specifies the character set to use to represent code points written to `buf`.
    /// * The converted characters are written into `buf`. If `buf` is [`None`], the method will
    ///   indicate how many bytes are required for conversion.
    ///
    /// Returns a [`GetBytesResult`] that specifies the number of bytes written to `buf` or the
    /// number of bytes required if `buf` is [`None`], and the range of the UTF-16 code units that
    /// were **not** converted.
    ///
    /// # Errors
    ///
    /// Returns a [`GetBytesError`] if the buffer cannot hold one code point, if `range` does not
    /// specify any characters to convert, if `range` divides a surrogate pair, or a surrogate is
    /// unpaired and `encoding` is [`GetBytesEncoding::Utf8`] or `encoding` is
    /// [`GetBytesEncoding::Utf32`] with a [`None`] `loss_byte`, or if a code point cannot be
    /// converted into `encoding` and `loss_byte` is [`None`].
    ///
    /// # Panics
    ///
    /// Panics if `range` cannot be represented in [`Range<usize>`] or if the `range` exceeds the
    /// bounds the string.
    #[inline]
    pub fn get_bytes(
        &self,
        range: impl RangeBounds<usize>,
        encoding: GetBytesEncoding,
        buf: Option<&mut [u8]>,
    ) -> Result<GetBytesResult, GetBytesError> {
        self.get_bytes_checked(self.range(range), encoding, buf)
    }

    fn get_bytes_checked(
        &self,
        range: CFRange,
        encoding: GetBytesEncoding,
        buf: Option<&mut [u8]>,
    ) -> Result<GetBytesResult, GetBytesError> {
        let buf_len_in = buf.as_ref().map(|buf| buf.len());

        // Converting nothing always succeeds.
        if range.is_empty() {
            return Ok(GetBytesResult {
                buf_len: 0,
                remaining: None,
            });
        } else if buf_len_in == Some(0) {
            return Ok(GetBytesResult {
                buf_len: 0,
                // UB: This was just converted from a [`Range<usize>`] so no wrapping will occur.
                remaining: Some(Range::<usize>::from_unchecked(range)),
            });
        }

        match self.get_bytes_unchecked_inner(range, encoding, buf) {
            // All of `range` was converted.
            GetBytesResult {
                buf_len,
                remaining: None,
            } => Ok(GetBytesResult {
                buf_len,
                remaining: None,
            }),

            // Not all of `range` was converted. Additional checks are needed to determine if the
            // conversion was successful or if an error occurred.
            GetBytesResult {
                buf_len,
                remaining: Some(remaining),
            } => match buf_len_in {
                // 1. The buffer was entirely filled.
                // 2. The buffer was not entirely filled, but some code points were converted.
                //    a. Conversion cannot fail (the caller provided a loss byte) so the next code
                //       unit is guaranteed to convert, but it's too large to encode into the
                //       remaining space. Conversion to a lossless encoding may fail if `range`
                //       splits a surrogate pair.
                //    b. Core Foundation always succeeds when converting to UTF-16.
                //
                // Another conversion call is necessary for the remaining code units.
                Some(buf_len_in)
                    if buf_len_in == buf_len
                        || (buf_len != 0
                            && (encoding.loss_byte().is_some()
                                || matches!(encoding, GetBytesEncoding::Utf16 { .. }))) =>
                {
                    Ok(GetBytesResult {
                        buf_len,
                        remaining: Some(remaining),
                    })
                }

                // When no buffer is provided, [`CFStringGetBytes`] only fails when conversion to
                // `encoding` is not possible. Conversion to a lossless encoding may fail if `range`
                // splits a surrogate pair.
                None => {
                    debug_assert!(
                        encoding.loss_byte().is_none(),
                        "A loss byte was provided, so the entire range should convert."
                    );
                    Err(
                        match self
                            .get_bytes_validate_surrogate_in_remaining_range(remaining.clone())
                        {
                            Ok(c) => GetBytesError::lossy_conversion_of_char(c, buf_len, remaining),
                            Err(reason) => GetBytesError::lossy_conversion_of_unpaired_surrogate(
                                reason, buf_len, remaining,
                            ),
                        },
                    )
                }

                // Not all of `range` was encoded into `buf` and not all of `buf` was utilized.
                Some(_) => {
                    match self.get_bytes_validate_surrogate_in_remaining_range(remaining.clone()) {
                        // The first code unit in the remaining range is valid.
                        Ok(c) => {
                            if !encoding.is_infallible()
                                && self
                                    .get_bytes_unchecked_inner(
                                        CFRange {
                                            // UB: Known to be representable by [`CFIndex`].
                                            location: CFIndex::from_unchecked(remaining.start),
                                            // UB: Value is always be 1 or 2.
                                            length: CFIndex::from_unchecked(c.len_utf16()),
                                        },
                                        encoding,
                                        None,
                                    )
                                    .remaining
                                    .is_some()
                            {
                                // `encoding` is not lossless, no loss byte was provided, and the
                                // next code point in `remaining` is not be convertible.
                                debug_assert!(
                                    encoding.loss_byte().is_none(),
                                    "A loss byte was provided, so the code point should convert."
                                );
                                Err(GetBytesError::lossy_conversion_of_char(
                                    c, buf_len, remaining,
                                ))
                            } else {
                                // The next code unit in the `remaining` range is valid but is too
                                // large to encode into the remaining space. Another conversion call
                                // is necessary for the remaining code units.
                                Ok(GetBytesResult {
                                    buf_len,
                                    remaining: Some(remaining),
                                })
                            }
                        }

                        // The first code unit in the remaining range is not a valid code point.
                        Err(reason) => {
                            debug_assert!(
                                encoding.loss_byte().is_none(),
                                "A loss byte was provided, so the code unit should convert."
                            );
                            Err(GetBytesError::lossy_conversion_of_unpaired_surrogate(
                                reason, buf_len, remaining,
                            ))
                        }
                    }
                }
            },
        }
    }

    /// Fetches a range of the code points from the string, converts the code points to `encoding`,
    /// and writes the result into the byte `buf`fer.
    ///
    /// Arguments:
    ///
    /// * `range` specifies the indices of the UTF-16 code units to process. If `range` divides a
    ///   surrogate pair, the surrogate code unit will be processed as a lossy conversion (except
    ///   for UTF-16 encodings, which are always lossless and UTF-8 conversions, which will always
    ///   fail).
    /// * `encoding` specifies the character set to use to represent code points written to `buf`.
    /// * The converted characters are written into `buf`. If `buf` is [`None`], the method will
    ///   indicate how many bytes are required for conversion.
    ///
    /// Returns a [`GetBytesResult`] that specifies the number of bytes written to `buf` or the
    /// number of bytes required if `buf` is [`None`], and the range of the UTF-16 code units that
    /// were **not** converted.
    ///
    /// # Panics
    ///
    /// Panics if `range` cannot be represented in [`Range<usize>`] or if the `range` exceeds the
    /// bounds the string.
    ///
    /// # Safety
    ///
    /// This method is named "unchecked" because it does not check the return value to determine if
    /// the conversion was successful. If conversion is not possible and a `loss_byte` is not
    /// provided or `encoding` is [`GetBytesEncoding::Utf8`], a conversion loop that does not check
    /// for forward progress may not terminate.
    ///
    /// This method is not marked `unsafe` because there is no risk of unsound behavior.
    #[inline]
    pub fn get_bytes_unchecked(
        &self,
        range: impl RangeBounds<usize>,
        encoding: GetBytesEncoding,
        buf: Option<&mut [u8]>,
    ) -> GetBytesResult {
        self.get_bytes_unchecked_inner(self.range(range), encoding, buf)
    }

    fn get_bytes_unchecked_inner(
        &self,
        range: CFRange,
        encoding: GetBytesEncoding,
        buf: Option<&mut [u8]>,
    ) -> GetBytesResult {
        let cf = self.as_ptr();
        let cf_encoding = encoding.into();
        let loss_byte = encoding.loss_byte().map(NonZeroU8::get).unwrap_or_default();
        let is_external_representation = u8::from(encoding.is_external_representation());
        let (buf, buf_len_in) = buf.map_or((ptr::null_mut(), 0), |buf| {
            let buf_len = buf.len();
            // UB: A slice's length cannot exceed [`isize::MAX`]. The upper bound of addressable
            // memory all current 64-bit machines is less than 64-bit. Apple no longer supports any
            // 32-bit platforms (where > 2 GiB allocations are possible but unlikely).
            let buf_len = CFIndex::from_unchecked(buf_len);
            (buf.as_mut_ptr(), buf_len)
        });

        // Core Foundation can overrun the buffer when writing the UTF-16 BOM, so we explicitly
        // validate the buffer capacity for the BOM here.
        if is_external_representation != 0
            && cf_encoding == kCFStringEncodingUTF16
            && !buf.is_null()
            && buf_len_in < 2
        {
            return GetBytesResult {
                buf_len: 0,
                remaining: Some(Range::<usize>::from_unchecked(range)),
            };
        }

        let mut buf_len_out = 0;
        // SAFETY: `cf` is a valid pointer to a `CFStringRef` object instance, and `buf` is a
        // valid pointer to a slice, which guarantees `buf_len_in` is correct.
        let code_units_converted = unsafe {
            CFStringGetBytes(
                cf,
                range,
                cf_encoding,
                loss_byte,
                is_external_representation,
                buf,
                buf_len_in,
                &mut buf_len_out,
            )
        };

        // UB: Core Foundation will never return a negative number.
        let buf_len_out = usize::from_unchecked(buf_len_out);

        GetBytesResult {
            buf_len: buf_len_out,
            remaining: (code_units_converted != range.length).then(|| {
                // UB: The result is less than or equal to the string's length, which is
                // representable by [`CFIndex`], so this will not wrap.
                let location = range.location.wrapping_add(code_units_converted);
                // UB: `code_units_converted` is less than `range.length` so this will not wrap.
                let length = range.length.wrapping_sub(code_units_converted);

                // UB: Both `location` and `length` are non-negative so they will not wrap.
                Range::<usize>::from_unchecked(CFRange { location, length })
            }),
        }
    }

    fn get_bytes_validate_surrogate_in_remaining_range(
        &self,
        remaining: Range<usize>,
    ) -> Result<char, GetBytesSurrogateError> {
        debug_assert!(
            !remaining.is_empty(),
            "The caller should have returned success if no code units remain to be converted."
        );
        let code_unit = self.index(remaining.start);

        match SurrogateHalf::try_from(code_unit) {
            Some(SurrogateHalf::High) => {
                // UB: Cannot overflow because it must be less than or equal to `remaining.end`.
                let after = remaining.start.wrapping_add(1);

                if after == self.len() {
                    // The string ends with a high surrogate.
                    Err(GetBytesSurrogateError::Unpaired)
                } else {
                    // PANIC: Cannot panic because the `if` above guarantees it's in bounds.
                    let code_unit_after = self.index(after);

                    if CFStringIsSurrogateLowCharacter(code_unit_after) {
                        if after == remaining.end {
                            // The range ended on a high surrogate. It must end on the low surrogate
                            // at the index after.
                            Err(GetBytesSurrogateError::Range)
                        } else {
                            // The `>=U+10000` code point at the start of `remaining` is valid.
                            let c = CFStringGetLongCharacterForSurrogatePair(
                                code_unit,
                                code_unit_after,
                            );
                            // SAFETY: The code units are part of a surrogate pair, which, by
                            // definition, form a valid code point when combined.
                            Ok(unsafe { char::from_u32_unchecked(c) })
                        }
                    } else {
                        // The high surrogate was not followed by a low surrogate.
                        Err(GetBytesSurrogateError::Unpaired)
                    }
                }
            }

            Some(SurrogateHalf::Low) => Err(remaining.start.checked_sub(1).map_or(
                // The string starts with a low surrogate.
                GetBytesSurrogateError::Unpaired,
                |before| {
                    // PANIC: Cannot panic because a lesser index must be in bounds.
                    if CFStringIsSurrogateHighCharacter(self.index(before)) {
                        // The range started on a low surrogate. It must start on the high surrogate
                        // at the index before.
                        GetBytesSurrogateError::Range
                    } else {
                        // The low surrogate was not preceded by a high surrogate.
                        GetBytesSurrogateError::Unpaired
                    }
                },
            )),

            None => {
                let c = code_unit.into();
                // SAFETY: The code unit is not part of a surrogate pair so it is, by definition, a
                // valid code point.
                Ok(unsafe { char::from_u32_unchecked(c) })
            }
        }
    }

    /// Gets the code unit at `index`.
    ///
    /// # Panics
    ///
    /// Panics if `index` exceeds the bounds of the string.
    #[inline]
    #[must_use]
    pub fn index(&self, index: usize) -> u16 {
        assert!(index < self.len(), "index out of bounds");

        let cf = self.as_ptr();
        let index = CFIndex::expect_from(index);

        // SAFETY: `cf` is a valid [`CFStringRef`] and `index` is in bounds.
        unsafe { CFStringGetCharacterAtIndex(cf, index) }
    }

    /// Returns `true` if `self` is the empty string, i.e. it does not have any code units.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Gets the number of UTF-16 code units in the string.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        let cf = self.as_ptr();
        // SAFETY: `cf` is a valid [`CFStringRef`]
        let length = unsafe { CFStringGetLength(cf) };
        // UB: Core Foundation will never return a negative number.
        usize::from_unchecked(length)
    }

    /// Converts a [`RangeBounds<T>`] to a [`Range<usize>`].
    ///
    /// # Panics
    ///
    /// Panics if `range` cannot be represented in [`Range<usize>`] or if the `range` exceeds the
    /// bounds the string.
    fn range(&self, range: impl RangeBounds<usize>) -> CFRange {
        CFRange::expect_from_range_bounds(range, self.len())
    }

    /// Yields a <code>&[str]</code> slice if the `String` is UTF-8 encoded and has contiguous
    /// storage. If the `String` is not UTF-8 encoded or does not have contiguous storage, returns
    /// [`None`].
    ///
    /// Do not rely on this method returning [`Some`]. Its return value may change and may not be
    /// consistent across multiple calls for the same object instance (e.g., a mutation causes the
    /// buffer to be converted to UTF-16, contiguous storage has been made non-contiguous or vice
    /// versa).
    ///
    /// [str]: prim@str
    #[inline]
    #[must_use]
    pub fn try_as_str(&self) -> Option<&str> {
        let cf = self.as_ptr();
        // SAFETY: `cf` is a valid [`CFStringRef`].
        let cstr = unsafe { CFStringGetCStringPtr(cf, kCFStringEncodingUTF8) };
        // SAFETY: If `cstr` is not `NULL`, it's an interior pointer that will live at least as long
        // as `self` and it is safe to dereference.
        unsafe { cstr.as_ref() }.map(|cstr| {
            // SAFETY: [`CFStringGetCStringPtr`] is guaranteed to return a `nul` terminated string.
            let bytes = unsafe { CStr::from_ptr(cstr) }.to_bytes();
            // SAFETY: Core Foundation only returns a non-`NULL` pointer if the string is composed
            // exclusively of code points with scalar values less than 128, and is thus
            // ASCII-compatible. `String`s bridged from Swift are guaranteed to be valid UTF-8
            // (Swift has no unsafe `String` initialization, AFAIK). The Objective-C method for
            // bridged `NSString`s is private so custom subclasses should return `NULL`.
            unsafe { str::from_utf8_unchecked(bytes) }
        })
    }
}

impl Display for String {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(s) = self.try_as_str() {
            f.write_str(s)
        } else {
            // 128 is arbitrary, but is an attempt to balance the size of the stack frame with the
            // overhead of each additional call to [`CFStringGetBytes`]. It's also the number of
            // bytes reserved for [`CFStringInlineBuffer`]'s code units.
            let mut buf = [0_u8; 128];
            let mut iter = GetBytesStrReader::new(self, GetBytesStrReplacement::default(), ..);

            while let Some(s) = iter.read(&mut buf) {
                f.write_str(s)?;
            }

            Ok(())
        }
    }
}

#[cfg(feature = "alloc")]
impl From<&String> for alloc::string::String {
    #[inline]
    fn from(value: &String) -> Self {
        GetBytesStrReader::new(value, GetBytesStrReplacement::default(), ..).collect()
    }
}

impl Display for FromBytesError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("invalid byte sequence for encoding")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromBytesError {}

impl Display for FromUtf8Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("invalid utf-8: invalid byte sequence")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromUtf8Error {}

impl Display for FromUtf32Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("invalid utf-32: non-code point scalar value found")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FromUtf32Error {}

impl GetBytesByteOrder {
    const fn is_external_representation(self) -> bool {
        match self {
            Self::BigEndian | Self::LittleEndian => false,
            Self::HostNative { include_bom } => include_bom,
        }
    }
}

impl GetBytesEncoding {
    /// Returns `true` if conversion should prepend a byte order mark (BOM).
    const fn is_external_representation(self) -> bool {
        match self {
            Self::CharacterSet { .. } | Self::Utf8 => false,
            Self::Utf16 { byte_order } | Self::Utf32 { byte_order, .. } => {
                byte_order.is_external_representation()
            }
        }
    }

    /// Returns `true` if conversion into encoding cannot fail for a range of valid code units
    /// (i.e., all surrogate pairs are in tact).
    const fn is_infallible(self) -> bool {
        match self {
            Self::CharacterSet {
                character_set,
                loss_byte,
            } => {
                // LINT: <CharacterSet as Into<CFStringEncoding>>::into cannot be used in a const
                // context.
                #[allow(clippy::as_conversions)]
                let is_lossless =
                    character_set as CFStringEncoding == kCFStringEncodingNonLossyASCII;
                loss_byte.is_some() || is_lossless
            }
            Self::Utf8 | Self::Utf16 { .. } | Self::Utf32 { .. } => true,
        }
    }

    /// Returns a byte to use in lieu of a code unit that cannot be converted into the target
    /// encoding.
    const fn loss_byte(self) -> Option<NonZeroU8> {
        match self {
            Self::CharacterSet { loss_byte, .. } | Self::Utf32 { loss_byte, .. } => loss_byte,
            Self::Utf8 | Self::Utf16 { .. } => None,
        }
    }
}

impl From<GetBytesEncoding> for CFStringEncoding {
    #[inline]
    fn from(value: GetBytesEncoding) -> Self {
        match value {
            GetBytesEncoding::CharacterSet { character_set, .. } => character_set.into(),
            GetBytesEncoding::Utf8 => kCFStringEncodingUTF8,
            GetBytesEncoding::Utf16 { byte_order } => match byte_order {
                GetBytesByteOrder::BigEndian => kCFStringEncodingUTF16BE,
                GetBytesByteOrder::HostNative { .. } => kCFStringEncodingUTF16,
                GetBytesByteOrder::LittleEndian => kCFStringEncodingUTF16LE,
            },
            GetBytesEncoding::Utf32 { byte_order, .. } => match byte_order {
                GetBytesByteOrder::BigEndian => kCFStringEncodingUTF32BE,
                GetBytesByteOrder::HostNative { .. } => kCFStringEncodingUTF32,
                GetBytesByteOrder::LittleEndian => kCFStringEncodingUTF32LE,
            },
        }
    }
}

impl GetBytesError {
    fn lossy_conversion_of_char(c: char, buf_len: usize, remaining: Range<usize>) -> Self {
        let range = Range {
            start: remaining.start,
            // UB: Cannot overflow because it's indexable by [`CFStringRef`].
            end: remaining.start.wrapping_add(c.len_utf16()),
        };
        let c_end = range.end;

        Self {
            kind: GetBytesErrorKind::Character { c, range },
            result: GetBytesResult {
                buf_len,
                remaining: (c_end < remaining.end).then_some(Range {
                    start: c_end,
                    end: remaining.end,
                }),
            },
        }
    }

    fn lossy_conversion_of_unpaired_surrogate(
        reason: GetBytesSurrogateError,
        buf_len: usize,
        remaining: Range<usize>,
    ) -> Self {
        // UB: Cannot overflow because it's indexable by [`CFStringRef`].
        let c_end = remaining.start.wrapping_add(1);

        Self {
            kind: GetBytesErrorKind::Surrogate {
                reason,
                index: remaining.start,
            },
            result: GetBytesResult {
                buf_len,
                remaining: (c_end < remaining.end).then_some(Range {
                    start: c_end,
                    end: remaining.end,
                }),
            },
        }
    }
}

impl SurrogateHalf {
    /// Returns the position of `value` in a surrogate pair, or [`None`] if the `value` lies in the
    /// Basic Multilingual Plane (BMP) and is thus a code point.
    #[inline]
    #[must_use]
    pub const fn try_from(value: u16) -> Option<Self> {
        if CFStringIsSurrogateHighCharacter(value) {
            Some(Self::High)
        } else if CFStringIsSurrogateLowCharacter(value) {
            Some(Self::Low)
        } else {
            None
        }
    }
}

#[cfg(feature = "alloc")]
fn alloc_error(utf16_len: usize) -> Arc<String> {
    use alloc::alloc::{handle_alloc_error, Layout};

    // The size of [`CFRuntimeBase`] is two [`usize`]s, and [`CFString`] adds two more: a pointer to
    // the buffer and the buffer length.
    const STRING_OBJECT_SIZE: usize = 4 * size_of::<usize>();
    // [`CFString`] object instances have pointer alignment.
    const STRING_OBJECT_ALIGN: usize = core::mem::align_of::<*const ()>();

    // UB: Cannot overflow because the number of bytes is representable by [`usize`].
    let buf_len = utf16_len.wrapping_mul(2);
    let size = STRING_OBJECT_SIZE.saturating_add(buf_len);

    // SAFETY: [`Layout`] is used only for error reporting purposes. 100% accuracy is not required.
    let layout = unsafe { Layout::from_size_align_unchecked(size, STRING_OBJECT_ALIGN) };

    handle_alloc_error(layout);
}

#[cfg(not(feature = "alloc"))]
fn alloc_error(_utf16_len: usize) -> Arc<String> {
    panic!("allocation failed")
}

const fn as_bytes<T>(v: &[T]) -> &[u8] {
    let data = v.as_ptr().cast();
    let len = v.len();

    // UB: Cannot overflow because the number of bytes is representable by [`usize`].
    let byte_len = size_of::<T>().wrapping_mul(len);

    // SAFETY: [`u8`]'s alignment requirements are less than or equal to `T`'s, the new slice covers
    // the exact same region of memory as `v`, and we are only transmuting the type of shared
    // reference to the memory region.
    unsafe { slice::from_raw_parts(data, byte_len) }
}

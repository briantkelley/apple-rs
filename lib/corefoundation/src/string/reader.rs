use crate::ffi::convert::FromUnchecked;
use crate::string::{
    GetBytesByteOrder, GetBytesEncoding, GetBytesError, GetBytesErrorKind, GetBytesResult, String,
};
use core::ops::{Range, RangeBounds};
use core::str;

/// An [`Read`]-like type to simplify calling [`String::get_bytes`]. It provides:
///
/// * Automatic adjustment of [`GetBytesEncoding`] so the byte order mark, if requested, is written
///   only once at the start of the output.
/// * Transparent tracking of the UTF-16 code unit range that has yet to be processed.
///
/// [`Read`]: std::io::Read
#[derive(Clone, Debug)]
pub struct GetBytesReader<'caller> {
    /// The string from which code points are fetched and converted into `encoding`.
    string: &'caller String,

    /// The character set in which to convert the `string`'s code points.
    encoding: GetBytesEncoding,

    /// The bounds of `string`'s UTF-16 code units remaining to be converted.
    range: Range<usize>,
}

/// Returned by [`GetBytesReader::read`] to indicate conversion progress and output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GetBytesReaderResult {
    /// The [`GetBytesReader`] made progress converting the `string`'s `range` into `encoding`.
    Ok {
        /// If an output buffer was provided, the number of bytes written into the buffer.
        /// Otherwise, the number of bytes required to convert the processed range into `encoding`.
        buf_len: usize,
    },

    /// A code unit or code point cannot be converted into `encoding`.
    ///
    /// Information about the code unit that failed to convert is available in `kind`.
    LossyConversion {
        /// If an output buffer was provided, the number of bytes written into the buffer.
        /// Otherwise, the number of bytes required to convert the processed range into `encoding`.
        buf_len: usize,

        /// Information about the code unit or code point that failed to convert into `encoding`.
        kind: GetBytesErrorKind,
    },
}

/// Returned by [`GetBytesReader::collect`] with the total number of bytes required for the
/// converted output, along with the number of code units that could not be converted into
/// `encoding`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GetBytesReaderSummary {
    /// The number of bytes required to write the converted code points.
    pub buf_len: usize,

    /// The number of UTF-16 code units that could not be converted into `encoding`.
    pub loss_char_count: usize,
}

/// An [`Read`]-like type to simplify calling [`String::get_bytes`]. It provides:
///
/// * The caller with a slice of the output buffer with only the valid bytes.
/// * More flexibility in handling lossy conversions with support for replacing non-convertible code
///   units with arbitrary bytes (e.g. `U+FFFD`).
/// * Automatic adjustment of [`GetBytesEncoding`] so the byte order mark, if requested, is written
///   only once at the start of the output.
/// * Transparent tracking of the UTF-16 code unit range that has yet to be processed.
/// * A panic if conversion does not make progress to prevent the caller from looping infinitely.
///
/// [`Read`]: std::io::Read
#[derive(Debug)]
pub struct GetBytesLossyReader<'caller> {
    inner: GetBytesReader<'caller>,
    replacement_bytes: Option<&'caller [u8]>,
    replacement_bytes_to_copy: Option<&'caller [u8]>,
}

/// An [`Read`]-like type to simplify calling [`String::get_bytes`]. It provides:
///
/// * The caller with a <code>&[str]</code> view of the output buffer with only the valid bytes.
/// * More flexibility in handling lossy conversions with support for replacing non-convertible code
///   units with arbitrary bytes (e.g. `U+FFFD`).
/// * Transparent tracking of the UTF-16 code unit range that has yet to be processed.
/// * A panic if conversion does not make progress to prevent the caller from looping infinitely.
///
/// [`Read`]: std::io::Read
/// [str]: prim@str
#[derive(Debug)]
pub struct GetBytesStrReader<'caller>(GetBytesLossyReader<'caller>);

/// Specifies how [`GetBytesStrReader`] should process code units that cannot be converted into
/// UTF-8.
#[derive(Clone, Copy, Debug, Default)]
pub enum GetBytesStrReplacement<'caller> {
    /// Silently ignore any code units that cannot be converted into UTF-8.
    None,

    /// Automatically replace any code units that cannot be converted into UTF-8 with the Unicode
    /// Replacement Character `U+FFFD`.
    #[default]
    UnicodeReplacement,

    /// Substitute the given string slice for each code unit that cannot be converted into UTF-8.
    Custom(&'caller str),
}

/// A character to substitute for code units that cannot be converted to UTF-8.
const REPLACEMENT_CHARACTER_UTF8: [u8; 3] = [0xef, 0xbf, 0xbd];

impl<'caller> GetBytesReader<'caller> {
    /// Creates [`Read`]-like type that calls [`String::get_bytes`] with `encoding` over the given
    /// `range`.
    ///
    /// # Panics
    ///
    /// Panics if `range` cannot be represented in [`Range<usize>`] or if the `range` exceeds the
    /// bounds the string.
    ///
    /// [`Read`]: std::io::Read
    #[inline]
    pub fn new(
        string: &'caller String,
        encoding: GetBytesEncoding,
        range: impl RangeBounds<usize>,
    ) -> Self {
        let range = string.range(range);
        Self {
            string,
            encoding,
            // UB: The Range will not wrap because the function argument's index type is unsigned.
            range: Range::<usize>::from_unchecked(range),
        }
    }
}

impl GetBytesReader<'_> {
    /// Collects the number of bytes required to convert the `string`'s `range` into `encoding`, and
    /// the number of code units that could not be converted into `encoding`.
    // LINT: A panic is due to an implementation error, not related to the caller.
    #[allow(clippy::missing_panics_doc)]
    #[inline]
    #[must_use]
    pub fn collect(mut self) -> GetBytesReaderSummary {
        let mut counts = GetBytesReaderSummary {
            buf_len: 0,
            loss_char_count: 0,
        };

        while let Some(result) = self.read(None) {
            match result {
                GetBytesReaderResult::Ok { buf_len } => {
                    // UB: This will not wrap because the upper bound is [`CFIndex::max`].
                    counts.buf_len = counts.buf_len.wrapping_add(buf_len);
                }
                GetBytesReaderResult::LossyConversion { buf_len, .. } => {
                    // UB: This will not wrap because the upper bound is [`CFIndex::max`].
                    counts.buf_len = counts.buf_len.wrapping_add(buf_len);
                    // UB: This will not wrap because the upper bound is [`CFIndex::max`].
                    counts.loss_char_count = counts.loss_char_count.wrapping_add(1);
                }
            }
        }

        counts
    }

    fn get_bytes(&mut self, buf: Option<&mut [u8]>) -> GetBytesReaderResult {
        match self
            .string
            .get_bytes(self.range.clone(), self.encoding, buf)
        {
            Ok(result) => GetBytesReaderResult::Ok {
                buf_len: self.handle_result(result),
            },
            Err(GetBytesError { result, kind }) => GetBytesReaderResult::LossyConversion {
                buf_len: self.handle_result(result),
                kind,
            },
        }
    }

    fn handle_result(&mut self, result: GetBytesResult) -> usize {
        let GetBytesResult { buf_len, remaining } = result;

        // If the caller requested a BOM, the flag must be cleared after conversion has made
        // progress so [`String::get_bytes`] doesn't insert a BOM on the next read.
        if buf_len != 0 {
            match self.encoding {
                GetBytesEncoding::Utf16 {
                    byte_order: GetBytesByteOrder::HostNative { include_bom: true },
                } => {
                    self.encoding = GetBytesEncoding::Utf16 {
                        byte_order: GetBytesByteOrder::HostNative { include_bom: false },
                    }
                }
                GetBytesEncoding::Utf32 {
                    byte_order: GetBytesByteOrder::HostNative { include_bom: true },
                    loss_byte,
                } => {
                    self.encoding = GetBytesEncoding::Utf32 {
                        byte_order: GetBytesByteOrder::HostNative { include_bom: false },
                        loss_byte,
                    }
                }
                _ => {}
            }
        }

        match remaining {
            Some(remaining) => self.range = remaining,
            None => self.range.start = self.range.end,
        }

        buf_len
    }

    /// Calls [`String::get_bytes`] and returns the progress of the conversion, or [`None`] if the
    /// entire range has been processed.
    #[inline]
    pub fn read(&mut self, buf: Option<&mut [u8]>) -> Option<GetBytesReaderResult> {
        if self.range.is_empty() {
            None
        } else {
            Some(self.get_bytes(buf))
        }
    }
}

impl<'caller> GetBytesLossyReader<'caller> {
    /// Creates [`Read`]-like type that calls [`String::get_bytes`] with `encoding` over the given
    /// `range`.
    ///
    /// If a `loss_byte` is set in `encoding`, `replacement_bytes` is not used. It is the caller's
    /// responsibility to ensure only one is [`Some`] or both are [`None`].
    ///
    /// The reader **does not** validate that `replacement_bytes` is valid for `encoding`. The
    /// caller has full responsibility for ensuring the byte sequence has an appropriate length and
    /// byte order for the given `encoding`. For example, if `encoding` is
    /// [`GetBytesEncoding::Utf32`], `replacement_bytes.len()` should be a multiple of 4 with each
    /// UTF-32 code point stored in the host's native byte order.
    ///
    /// # Panics
    ///
    /// Panics if `range` cannot be represented in [`Range<usize>`] or if the `range` exceeds the
    /// bounds the string.
    ///
    /// [`Read`]: std::io::Read
    #[inline]
    pub fn new(
        string: &'caller String,
        encoding: GetBytesEncoding,
        replacement_bytes: Option<&'caller [u8]>,
        range: impl RangeBounds<usize>,
    ) -> Self {
        Self {
            inner: GetBytesReader::new(string, encoding, range),
            replacement_bytes: replacement_bytes.and_then(|replacement_bytes| {
                (!replacement_bytes.is_empty()).then_some(replacement_bytes)
            }),
            replacement_bytes_to_copy: None,
        }
    }
}

impl GetBytesLossyReader<'_> {
    /// Collects all bytes from `string`'s `range` converted into `encoding` into a single buffer.
    // LINT: A panic is due to an implementation error, not related to the caller.
    #[allow(clippy::missing_panics_doc)]
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn collect(mut self) -> Vec<u8> {
        let counts = self.inner.clone().collect();

        let loss_len = self
            .replacement_bytes
            .map(<[u8]>::len)
            .unwrap_or_default()
            .checked_mul(counts.loss_char_count)
            .expect("capacity overflow");

        let buf_len = counts
            .buf_len
            .checked_add(loss_len)
            .expect("capacity overflow");

        let mut buf: Vec<u8> = vec![0; buf_len];
        assert_eq!(self.get_bytes(&mut buf), buf_len, "capacity miscalculation");
        assert!(self.inner.range.is_empty(), "did not collect all of range");
        buf
    }

    fn get_bytes(&mut self, buf: &mut [u8]) -> usize {
        let mut next_write_index: usize = 0;

        loop {
            if let Some(replacement_bytes) = self.replacement_bytes_to_copy {
                let buf_end = next_write_index
                    .checked_add(replacement_bytes.len())
                    .expect("capacity overflow");

                if let Some(dest) = buf.get_mut(next_write_index..buf_end) {
                    dest.copy_from_slice(replacement_bytes);
                    self.replacement_bytes_to_copy = None;
                    next_write_index = buf_end;

                    // Preempt the next call to [`String::get_bytes`] if the buffer is now full.
                    if next_write_index == buf.len() {
                        break;
                    }
                } else {
                    // The replacement must be appended atomically to avoid buffer have only part of
                    // a code unit.
                    assert!(
                        next_write_index != 0,
                        "buffer too small for lossy character replacement"
                    );
                    // The buffer does not have enough space remaining to write the replacement.
                    // Try again on the next read.
                    break;
                }
            }

            // Note: Use `read` in case the inner reader is empty and the loop started to append a
            // replacement for the last, non-convertible code unit.
            // LINT: A panic here indicates an internal [`GetBytesLossyReader`] logic error.
            #[allow(clippy::indexing_slicing)]
            let (buf_len, done) = match self.inner.read(Some(&mut buf[next_write_index..])) {
                None => (0, true),
                Some(GetBytesReaderResult::Ok { buf_len }) => (buf_len, true),
                Some(GetBytesReaderResult::LossyConversion { buf_len, .. }) => {
                    self.replacement_bytes_to_copy = self.replacement_bytes;
                    (buf_len, false)
                }
            };

            // UB: Cannot overflow because it will never exceed `buf.len()`.
            next_write_index = next_write_index.wrapping_add(buf_len);

            // If [`String::get_bytes`] did not stop due to a non-convertible code unit, then exit
            // the loop because the buffer has been fully utilized.
            if done {
                break;
            }
        }

        assert!(
            next_write_index != 0,
            "buffer too small to hold a code point"
        );

        next_write_index
    }

    /// Calls [`String::get_bytes`] and returns the portion of `buf` that was written into. Or, if
    /// all of the previously given `string`'s `range` has been converted, returns [`None`].
    ///
    /// For each invalid code unit is encountered, a copy of `replacement_bytes` is append into the
    /// result. If `replacement_bytes` was [`None`], invalid code units are silently skipped.
    ///
    /// # Panics
    ///
    /// Panics if the caller provided `buf`fer is too small to hold one code point or the
    /// replacement bytes.
    #[inline]
    pub fn read<'buf>(&mut self, buf: &'buf mut [u8]) -> Option<&'buf [u8]> {
        (!self.inner.range.is_empty() || self.replacement_bytes_to_copy.is_some()).then(|| {
            let buf_len = self.get_bytes(buf);
            // LINT: A panic here indicates an internal [`GetBytesLossyReader`] logic error.
            #[allow(clippy::indexing_slicing)]
            &buf[..buf_len]
        })
    }
}

impl<'caller> GetBytesStrReader<'caller> {
    /// Creates [`Read`]-like type that calls [`String::get_bytes`] to read `string`'s code units as
    /// a <code>&[str]</code>.
    ///
    /// # Panics
    ///
    /// Panics if `range` cannot be represented in [`Range<usize>`] or if the `range` exceeds the
    /// bounds the string.
    ///
    /// [`Read`]: std::io::Read
    /// [str]: prim@str
    #[inline]
    pub fn new(
        string: &'caller String,
        replacement: GetBytesStrReplacement<'caller>,
        range: impl RangeBounds<usize>,
    ) -> Self {
        GetBytesStrReader(GetBytesLossyReader::new(
            string,
            GetBytesEncoding::Utf8,
            replacement.as_bytes(),
            range,
        ))
    }
}

impl GetBytesStrReader<'_> {
    /// Converts the `string`'s `range` into a Rust [`String`].
    ///
    /// [`String`]: alloc::string::String
    #[cfg(feature = "alloc")]
    #[inline]
    #[must_use]
    pub fn collect(self) -> alloc::string::String {
        let bytes = self.0.collect();
        // SAFETY: `bytes` is guaranteed to be a UTF-8 encoded string.
        unsafe { alloc::string::String::from_utf8_unchecked(bytes) }
    }

    /// Calls [`String::get_bytes`] and returns the portion of `buf` that was written into as a
    /// <code>&[str]</code> for idiomatic access to the UTF-8 encoding of the `string`. Returns
    /// [`None`] after returning bytes for the previously given string range.
    ///
    /// For each invalid code unit is encountered, a copy of `replacement` is append into the
    /// result. If `replacement` was [`None`], invalid code units are silently skipped.
    ///
    /// # Panics
    ///
    /// Panics if the caller provided `buf`fer is too small to hold one code point or the
    /// replacement bytes.
    ///
    /// [str]: prim@str
    #[inline]
    pub fn read<'buf>(&mut self, buf: &'buf mut [u8]) -> Option<&'buf str> {
        self.0.read(buf).map(|buf| {
            // SAFETY: [`String::get_bytes`] returns valid UTF-8. Any code units that cannot be
            // converted to UTF-8 are skipped or replaced with valid UTF-8 (the default replacement
            // character or the user-provided [`str`]).
            unsafe { str::from_utf8_unchecked(buf) }
        })
    }
}

impl<'caller> GetBytesStrReplacement<'caller> {
    /// Returns the lossy character replacement as a slice of UTF-8 bytes.
    #[inline]
    #[must_use]
    pub const fn as_bytes(self) -> Option<&'caller [u8]> {
        match self {
            GetBytesStrReplacement::None => None,
            GetBytesStrReplacement::UnicodeReplacement => {
                Some(REPLACEMENT_CHARACTER_UTF8.as_slice())
            }
            GetBytesStrReplacement::Custom(s) => Some(s.as_bytes()),
        }
    }
}

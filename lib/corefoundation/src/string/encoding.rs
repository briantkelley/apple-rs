use corefoundation_sys::{
    CFStringEncoding, kCFStringEncodingDOSChineseTrad, kCFStringEncodingMacRoman,
};

/// Identifiers for non-Unicode character sets and encodings.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
#[repr(u32)]
pub enum Encoding {
    /// Mac OS Roman
    ///
    /// * Encoding Scheme: Fixed-width
    /// * Code Unit Size: 1 byte
    /// * ASCII compatibility: Code points 0 through 127, inclusive.
    MacRoman = kCFStringEncodingMacRoman,

    /// OEM/ANSI code page 950. Traditional Chinese (Taiwan; Hong Kong SAR, PRC); Chinese
    /// Traditional (Big5)
    ///
    /// * Encoding Scheme: Variable-width (1 or 2 code units)
    /// * Code Unit Size: 1 byte
    /// * ASCII compatibility: Code points 0 through 127, inclusive.
    AnsiTraditionalChinese = kCFStringEncodingDOSChineseTrad,
}

impl Encoding {
    /// Returns the underlying [`CFStringEncoding`].
    #[inline]
    #[must_use]
    pub const fn into_raw(self) -> CFStringEncoding {
        self as CFStringEncoding
    }
}

impl From<Encoding> for CFStringEncoding {
    #[inline]
    fn from(value: Encoding) -> Self {
        value.into_raw()
    }
}

use corefoundation_sys::{
    kCFStringEncodingDOSChineseTrad, kCFStringEncodingMacRoman, CFStringEncoding,
};

/// A character set encoding that is a subset of The Unicode Standard.
///
/// An encoding may be a single-byte character set (SBCS), a double-byte character set (DBCS, though
/// this may be a misnomer as many "double-byte" character sets reuse the low ASCII range and encode
/// low ASCII code points with a single byte), or a multi-byte character set (MBCS).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
#[repr(u32)]
pub enum CharacterSet {
    /// Mac OS Roman, an 8-bit character set.
    ///
    /// Code points `0..128` are identical to ASCII.
    MacRoman = kCFStringEncodingMacRoman,

    /// Windows Code Page 950, a double-byte character set (DBCS).
    ///
    /// Microsoft's implementation of the Big-5 or Big5 character encoding, which is a Chinese
    /// character encoding method used in Taiwan, Hong Kong, and Macau for traditional Chinese
    /// characters.
    TraditionalChinese = kCFStringEncodingDOSChineseTrad,
}

impl From<CharacterSet> for CFStringEncoding {
    // LINT: This is a lossless conversion into the type required by the FFI.
    #[allow(clippy::as_conversions)]
    #[inline]
    fn from(value: CharacterSet) -> Self {
        value as Self
    }
}

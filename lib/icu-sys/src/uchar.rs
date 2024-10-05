use crate::umachine::{UBool, UChar32};
use crate::uversion::UVersionInfo;
use core::ffi::{c_int, c_uint};

/// Selection constants for Unicode properties.
///
/// These constants are used in functions like [`u_hasBinaryProperty`] to select one of the Unicode
/// properties.
///
/// The properties APIs are intended to reflect Unicode properties as defined in the Unicode
/// Character Database (UCD) and Unicode Technical Reports (UTR).
///
/// For details about the properties see UAX #44: Unicode Character Database
/// (<http://www.unicode.org/reports/tr44/>).
///
/// Important: If ICU is built with UCD files from Unicode versions below, e.g., 3.2,
/// then properties marked with "new in Unicode 3.2" are not or not fully available. Check
/// `u_getUnicodeVersion` to be sure.
pub type UProperty = c_int;

/// Binary property `Alphabetic`. Same as `u_isUAlphabetic`, different from `u_isalpha`.
///
/// `Lu` + `Ll` + `Lt` + `Lm` + `Lo` + `Nl` + `Other_Alphabetic`
///
/// Stable since ICU 2.1
pub const UCHAR_ALPHABETIC: UProperty = 0;

/// Binary property `Lowercase`. Same as `u_isULowercase`, different from `u_islower`.
///
/// `Ll` + `Other_Lowercase`
///
/// Stable since ICU 2.1
pub const UCHAR_LOWERCASE: UProperty = 22;

/// Binary property `Uppercase`. Same as `u_isUUppercase`, different from `u_isupper`.
///
/// `Lu` + `Other_Uppercase`
///
/// Stable since ICU 2.1
pub const UCHAR_UPPERCASE: UProperty = 30;

/// Binary property `White_Space`. Same as `u_isUWhiteSpace`, different from `u_isspace` and
/// `u_isWhitespace`.
///
/// Space characters + `TAB` + `CR` + `LF` - `ZWSP` - `ZWNBSP`
///
/// Stable since ICU 2.1
pub const UCHAR_WHITE_SPACE: UProperty = 31;

/// Data for enumerated Unicode general category types.
///
/// See <http://www.unicode.org/Public/UNIDATA/UnicodeData.html>.
///
/// Stable since ICU 2.0
pub type UCharCategory = c_uint;

/// Non-category for unassigned and non-character code points.
///
/// Cn "Other, Not Assigned" (no characters in `UnicodeData.txt` have this
/// property.)
///
/// Stable since ICU 2.0
pub const U_UNASSIGNED: UCharCategory = 0;

/// Lu
///
/// Stable since ICU 2.0
pub const U_UPPERCASE_LETTER: UCharCategory = 1;

/// Ll
///
/// Stable since ICU 2.0
pub const U_LOWERCASE_LETTER: UCharCategory = 2;

/// Lt
///
/// Stable since ICU 2.0
pub const U_TITLECASE_LETTER: UCharCategory = 3;

/// Lm
///
/// Stable since ICU 2.0
pub const U_MODIFIER_LETTER: UCharCategory = 4;

/// Lo
///
/// Stable since ICU 2.0
pub const U_OTHER_LETTER: UCharCategory = 5;

/// Mn
///
/// Stable since ICU 2.0
pub const U_NON_SPACING_MARK: UCharCategory = 6;

/// Me
///
/// Stable since ICU 2.0
pub const U_ENCLOSING_MARK: UCharCategory = 7;

/// Mc
///
/// Stable since ICU 2.0
pub const U_COMBINING_SPACING_MARK: UCharCategory = 8;

/// Nd
///
/// Stable since ICU 2.0
pub const U_DECIMAL_DIGIT_NUMBER: UCharCategory = 9;

/// Nl
///
/// Stable since ICU 2.0
pub const U_LETTER_NUMBER: UCharCategory = 10;

/// No
///
/// Stable since ICU 2.0
pub const U_OTHER_NUMBER: UCharCategory = 11;

/// Zs
///
/// Stable since ICU 2.0
pub const U_SPACE_SEPARATOR: UCharCategory = 12;

/// Zl
///
/// Stable since ICU 2.0
pub const U_LINE_SEPARATOR: UCharCategory = 13;

/// Zp
///
/// Stable since ICU 2.0
pub const U_PARAGRAPH_SEPARATOR: UCharCategory = 14;

/// Cc
///
/// Stable since ICU 2.0
pub const U_CONTROL_CHAR: UCharCategory = 15;

/// Cf
///
/// Stable since ICU 2.0
pub const U_FORMAT_CHAR: UCharCategory = 16;

/// Co
///
/// Stable since ICU 2.0
pub const U_PRIVATE_USE_CHAR: UCharCategory = 17;

/// Cs
///
/// Stable since ICU 2.0
pub const U_SURROGATE: UCharCategory = 18;

/// Pd
///
/// Stable since ICU 2.0
pub const U_DASH_PUNCTUATION: UCharCategory = 19;

/// Ps
///
/// Stable since ICU 2.0
pub const U_START_PUNCTUATION: UCharCategory = 20;

/// Pe
///
/// Stable since ICU 2.0
pub const U_END_PUNCTUATION: UCharCategory = 21;

/// Pc
///
/// Stable since ICU 2.0
pub const U_CONNECTOR_PUNCTUATION: UCharCategory = 22;

/// Po
///
/// Stable since ICU 2.0
pub const U_OTHER_PUNCTUATION: UCharCategory = 23;

/// Sm
///
/// Stable since ICU 2.0
pub const U_MATH_SYMBOL: UCharCategory = 24;

/// Sc
///
/// Stable since ICU 2.0
pub const U_CURRENCY_SYMBOL: UCharCategory = 25;

/// Sk
///
/// Stable since ICU 2.0
pub const U_MODIFIER_SYMBOL: UCharCategory = 26;

/// So
///
/// Stable since ICU 2.0
pub const U_OTHER_SYMBOL: UCharCategory = 27;

/// Pi
///
/// Stable since ICU 2.0
pub const U_INITIAL_PUNCTUATION: UCharCategory = 28;

/// Pf
///
/// Stable since ICU 2.0
pub const U_FINAL_PUNCTUATION: UCharCategory = 29;

extern "C" {
    /// Check a binary Unicode property for a code point `c`.
    ///
    /// Returns `true` or `false` according to the binary Unicode property value for `c`. Also
    /// `false` if `which` is out of bounds or if the Unicode version does not have data for the
    /// property at all.
    ///
    /// Unicode, especially in version 3.2, defines many more properties than the original set in
    /// `UnicodeData.txt`.
    ///
    /// The properties APIs are intended to reflect Unicode properties as defined in the Unicode
    /// Character Database (UCD) and Unicode Technical Reports (UTR). For details about the
    /// properties see <http://www.unicode.org/ucd/>. For names of Unicode properties see the UCD
    /// file `PropertyAliases.txt`.
    ///
    /// Important: If ICU is built with UCD files from Unicode versions below 3.2, then properties
    /// marked with "new in Unicode 3.2" are not or not fully available.
    pub fn u_hasBinaryProperty(c: UChar32, which: UProperty) -> UBool;

    /// Returns the general category value ([`UCharCategory`]) for the code point `c`.
    ///
    /// Same as `java.lang.Character.getType()`.
    ///
    /// Stable since ICU 2.0
    pub fn u_charType(c: UChar32) -> i8;

    /// Get the "age" of the code point `c`.
    ///
    /// The "age" is the Unicode version when the code point was first designated (as a
    /// non-character or for Private Use) or assigned a character.
    ///
    /// This can be useful to avoid emitting code points to receiving processes that do not accept
    /// newer characters.
    ///
    /// The data is from the UCD file `DerivedAge.txt`.
    ///
    /// Stable since ICU 2.1
    pub fn u_charAge(c: UChar32, versionArray: *mut UVersionInfo);

    /// Gets the Unicode version implemented by the ICU library.
    ///
    /// For example, Unicode version 3.1.1 is represented as an array with the value `[3, 1, 1, 0]`.
    ///
    /// Stable since ICU 2.0
    pub fn u_getUnicodeVersion(versionArray: *mut UVersionInfo);
}

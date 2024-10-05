use icu_sys::{
    u_charAge, u_charType, u_getUnicodeVersion, u_hasBinaryProperty, UProperty, UVersionInfo,
    UCHAR_ALPHABETIC, UCHAR_LOWERCASE, UCHAR_UPPERCASE, UCHAR_WHITE_SPACE,
    U_COMBINING_SPACING_MARK, U_CONNECTOR_PUNCTUATION, U_CONTROL_CHAR, U_CURRENCY_SYMBOL,
    U_DASH_PUNCTUATION, U_DECIMAL_DIGIT_NUMBER, U_ENCLOSING_MARK, U_END_PUNCTUATION,
    U_FINAL_PUNCTUATION, U_FORMAT_CHAR, U_INITIAL_PUNCTUATION, U_LETTER_NUMBER, U_LINE_SEPARATOR,
    U_LOWERCASE_LETTER, U_MATH_SYMBOL, U_MODIFIER_LETTER, U_MODIFIER_SYMBOL, U_NON_SPACING_MARK,
    U_OTHER_LETTER, U_OTHER_NUMBER, U_OTHER_PUNCTUATION, U_OTHER_SYMBOL, U_PARAGRAPH_SEPARATOR,
    U_PRIVATE_USE_CHAR, U_SPACE_SEPARATOR, U_START_PUNCTUATION, U_SURROGATE, U_TITLECASE_LETTER,
    U_UPPERCASE_LETTER,
};

mod sealed {
    use super::{u_hasBinaryProperty, UProperty};

    /// To avoid ambiguity with built-in [`char`] methods, equivalent Unicode properties provided by
    /// this crate are exposed through a generic method ([`UnicodeProperties::is`], inspired by
    /// the design of [`CodePointSetData`][] in the `icu_properties` crate).
    ///
    /// While nominally redundant, the Unicode version implemented by the ICU library may be
    /// different than the Unicode version implemented by Rust, so downstream code should use either
    /// this crate's interface, or Rust's interface, but not both to ensure internal consistency.
    ///
    /// [`UnicodeProperties::is`]: super::UnicodeProperties::is
    /// [`CodePointSetData`]: https://docs.rs/icu/latest/icu/properties/sets/struct.CodePointSetData.html
    pub trait BinaryProperty {
        /// The ICU selector constant that identifies the binary property to check.
        const SELECTOR: UProperty;

        fn for_char(c: char) -> bool {
            let c = c as i32;
            let which = Self::SELECTOR;

            // SAFETY: [`u_hasBinaryProperty`] does not have any safety requirements.
            (unsafe { u_hasBinaryProperty(c, which) }) != 0
        }
    }
}

/// Code points with the following [`GeneralCategory`] values:
///
/// * [`GeneralCategory::UppercaseLetter`] (`Lu`)
/// * [`GeneralCategory::LowercaseLetter`] (`Ll`)
/// * [`GeneralCategory::TitlecaseLetter`] (`Lt`)
/// * [`GeneralCategory::ModifierLetter`] (`Lm`)
/// * [`GeneralCategory::OtherLetter`] (`Lo`)
/// * [`GeneralCategory::LetterNumber`] (`Nl`)
///
/// In addition, includes code points with the [`Other_Alphabetic`][] contributory property. Use
/// with [`UnicodeProperties::is`] as a replacement for [`char::is_alphabetic`].
///
/// # Examples
///
/// ```
/// # use icu::{Alphabetic, UnicodeProperties};
/// assert!('a'.is::<Alphabetic>());
/// assert!('京'.is::<Alphabetic>());
///
/// let c = '💝';
/// // love is many things, but it is not alphabetic
/// assert!(!c.is::<Alphabetic>());
/// ```
///
/// [`Other_Alphabetic`]: https://www.unicode.org/reports/tr44/#Other_Alphabetic
#[derive(Clone, Copy, Debug)]
pub struct Alphabetic(());

/// Code points that are [`Alphabetic`] or [`Numeric`]. Use with [`UnicodeProperties::is`] as a
/// replacement for [`char::is_alphanumeric`].
///
/// # Examples
///
/// ```
/// # use icu::{Alphanumeric, UnicodeProperties};
/// assert!('٣'.is::<Alphanumeric>());
/// assert!('7'.is::<Alphanumeric>());
/// assert!('৬'.is::<Alphanumeric>());
/// assert!('¾'.is::<Alphanumeric>());
/// assert!('①'.is::<Alphanumeric>());
/// assert!('K'.is::<Alphanumeric>());
/// assert!('و'.is::<Alphanumeric>());
/// assert!('藏'.is::<Alphanumeric>());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Alphanumeric(());

/// Code points in the [`GeneralCategory::Control`]. Use with [`UnicodeProperties::is`] as a
/// replacement for [`char::is_control`].
///
/// # Examples
///
/// ```
/// # use icu::{Control, UnicodeProperties};
/// // U+009C, STRING TERMINATOR
/// assert!('\u{009c}'.is::<Control>());
/// assert!(!'q'.is::<Control>());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Control(());

/// Code points in the [`GeneralCategory::LowercaseLetter`] (`Ll`), and code points with the
/// [`Other_Lowercase`][] contributory property. Use with [`UnicodeProperties::is`] as a replacement
/// for [`char::is_lowercase`].
///
/// # Examples
///
/// ```
/// # use icu::{Lowercase, UnicodeProperties};
/// assert!('a'.is::<Lowercase>());
/// assert!('δ'.is::<Lowercase>());
/// assert!(!'A'.is::<Lowercase>());
/// assert!(!'Δ'.is::<Lowercase>());
///
/// // The various Chinese scripts and punctuation do not have case, and so:
/// assert!(!'中'.is::<Lowercase>());
/// assert!(!' '.is::<Lowercase>());
/// ```
///
/// [`Other_Lowercase`]: https://www.unicode.org/reports/tr44/#Other_Lowercase
#[derive(Clone, Copy, Debug)]
pub struct Lowercase(());

/// Code points with the following [`GeneralCategory`] values:
///
/// * [`GeneralCategory::DecimalNumber`] (`Nd`)
/// * [`GeneralCategory::LetterNumber`] (`Nl`)
/// * [`GeneralCategory::OtherNumber`] (`No`)
///
/// Use with [`UnicodeProperties::is`] as a replacement for [`char::is_numeric`].
///
/// # Examples
///
/// ```
/// # use icu::{Numeric, UnicodeProperties};
/// assert!('٣'.is::<Numeric>());
/// assert!('7'.is::<Numeric>());
/// assert!('৬'.is::<Numeric>());
/// assert!('¾'.is::<Numeric>());
/// assert!('①'.is::<Numeric>());
/// assert!(!'K'.is::<Numeric>());
/// assert!(!'و'.is::<Numeric>());
/// assert!(!'藏'.is::<Numeric>());
/// assert!(!'三'.is::<Numeric>());
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Numeric(());

/// Code points in the [`GeneralCategory::UppercaseLetter`] (`Lu`), and code points with the
/// [`Other_Uppercase`][] contributory property. Use with [`UnicodeProperties::is`] as a replacement
/// for [`char::is_uppercase`].
///
/// # Examples
///
/// ```
/// # use icu::{Uppercase, UnicodeProperties};
/// assert!(!'a'.is::<Uppercase>());
/// assert!(!'δ'.is::<Uppercase>());
/// assert!('A'.is::<Uppercase>());
/// assert!('Δ'.is::<Uppercase>());
///
/// // The various Chinese scripts and punctuation do not have case, and so:
/// assert!(!'中'.is::<Uppercase>());
/// assert!(!' '.is::<Uppercase>());
/// ```
///
/// [`Other_Uppercase`]: https://www.unicode.org/reports/tr44/#Other_Uppercase
#[derive(Clone, Copy, Debug)]
pub struct Uppercase(());

/// Code points with the [`White_Space`][] property. Use with [`UnicodeProperties::is`] as a
/// replacement for [`char::is_whitespace`].
///
/// # Examples
///
/// ```
/// # use icu::{Whitespace, UnicodeProperties};
/// assert!(' '.is::<Whitespace>());
///
/// // line break
/// assert!('\n'.is::<Whitespace>());
///
/// // a non-breaking space
/// assert!('\u{A0}'.is::<Whitespace>());
///
/// assert!(!'越'.is::<Whitespace>());
/// ```
///
/// [`White_Space`]: https://www.unicode.org/reports/tr44/#White_Space
#[derive(Clone, Copy, Debug)]
pub struct Whitespace(());

/// The most general classification of a code point, which is usually determined based on the
/// primary characteristic of the assigned character for that code point.
///
/// For more information, see [General Category Values][] at `unicode.org`.
///
/// # Compatibility Note
///
/// [`unicode-properties`][] implements [`Ord`] and [`PartialOrd`] for this enum, but there is no
/// intrinsic order. This implementation matches the order of the [`icu_properties`][] crate, which
/// also matches ICU4C. If an order is desired, it should be implemented using application-specific
/// logic.
///
/// [General Category Values]: https://www.unicode.org/reports/tr44/#General_Category_Values
/// [`icu_properties`]: https://github.com/unicode-org/icu4x/blob/icu%401.5.0/components/properties/src/props.rs#L823-L906
/// [`unicode-properties`]: https://docs.rs/unicode-properties/latest/unicode_properties/general_category/enum.GeneralCategory.html#trait-implementations
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GeneralCategory {
    /// `Cn`, a reserved unassigned code point or a non-character
    Unassigned,

    /// `Lu`, an uppercase letter
    UppercaseLetter,
    /// `Ll`, a lowercase letter
    LowercaseLetter,
    /// `Lt`, a digraph encoded as a single character, with first part uppercase
    TitlecaseLetter,
    /// `Lm`, a modifier letter
    ModifierLetter,
    /// `Lo`, other letters, including syllables and ideographs
    OtherLetter,

    /// `Mn`, a non-spacing combining mark (zero advance width)
    NonspacingMark,
    /// `Me`, an enclosing combining mark
    EnclosingMark,
    /// `Mc`, a spacing combining mark (positive advance width)
    SpacingMark,

    /// `Nd`, a decimal digit
    DecimalNumber,
    /// `Nl`, a letter-like numeric character
    LetterNumber,
    /// `No`, a numeric character of other type
    OtherNumber,

    /// `Zs`, a space character (of various non-zero widths)
    SpaceSeparator,
    /// `Zl`, `\u{2028}` LINE SEPARATOR only
    LineSeparator,
    /// `Zp`, `\u{2029}` PARAGRAPH SEPARATOR only
    ParagraphSeparator,

    /// `Cc`, a `C0` or `C1` control code
    Control,
    /// `Cf`, a format control character
    Format,
    /// `Co`, a private-use character
    PrivateUse,
    /// `Cs`, a surrogate code point
    Surrogate,

    /// `Pd`, a dash or hyphen punctuation mark
    DashPunctuation,
    /// `Ps`, an opening punctuation mark (of a pair)
    OpenPunctuation,
    /// `Pe`, a closing punctuation mark (of a pair)
    ClosePunctuation,
    /// `Pc`, a connecting punctuation mark, like a tie
    ConnectorPunctuation,
    /// `Po`, a punctuation mark of other type
    OtherPunctuation,

    /// `Sm`, a symbol of mathematical use
    MathSymbol,
    /// `Sc`, a currency sign
    CurrencySymbol,
    /// `Sk`, a non-letter-like modifier symbol
    ModifierSymbol,
    /// `So`, a symbol of other type
    OtherSymbol,

    /// `Pi`, an initial quotation mark
    InitialPunctuation,
    /// `Pf`, a final quotation mark
    FinalPunctuation,
}

/// Families of related [`GeneralCategory`] variants.
///
/// # Compatibility Note
///
/// [`unicode-properties`][] implements [`Ord`] and [`PartialOrd`] for this enum, but there is no
/// intrinsic order. This implementation matches the order of ICU4C (so to speak). If an order is
/// desired, it should be implemented using application-specific logic.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum GeneralCategoryGroup {
    /// `L` categories (`Lu | Ll | Lt | Lm | Lo`).
    Letter,
    /// `M` categories (`Mn | Me | Mc`).
    Mark,
    /// `N` categories (`Nd | Nl | No`).
    Number,
    /// `Z` categories (`Zs | Zl | Zp`).
    Separator,
    /// `C` categories (`Cn | Cc | Cf | Co | Cs`).
    Other,
    /// `P` categories (`Pd | Ps | Pe | Pc | Po | Pi | Pf`).
    Punctuation,
    /// `S` categories (`Sm | Sc | Sk | So`).
    Symbol,
}

/// Interface to get a Unicode code point's general category, as defined by [UAX #44][].
///
/// [UAX #44]: https://www.unicode.org/reports/tr44/
pub trait UnicodeGeneralCategory: Sized {
    /// Gets the classification of the code point.
    fn general_category(self) -> GeneralCategory;

    /// Returns the family of the code point's [`GeneralCategory`].
    #[inline]
    fn general_category_group(self) -> GeneralCategoryGroup {
        GeneralCategoryGroup::from(self.general_category())
    }

    /// `LC` categories (`Lu | Ll | Lt`).
    ///
    /// Returns whether the family of the code point is "Cased Letter", which is a subset of
    /// [`Letter`].
    ///
    /// [`Letter`]: GeneralCategoryGroup::Letter
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    fn is_letter_cased(self) -> bool {
        matches!(
            self.general_category(),
            GeneralCategory::UppercaseLetter
                | GeneralCategory::LowercaseLetter
                | GeneralCategory::TitlecaseLetter
        )
    }
}

/// Interface to get arbitrary Unicode code point properties, as defined by [UAX #44][].
///
/// [UAX #44]: https://www.unicode.org/reports/tr44/
pub trait UnicodeProperties {
    /// Returns the first version of the Unicode standard in which this code point was assigned a
    /// character, or designed as a non-character, or designed for Private Use. Returns [`None`] if
    /// the code point does not yet have a designation.
    fn age(self) -> Option<[u8; 2]>;

    /// Returns `true` if the code point has the binary Unicode property.
    ///
    /// Returns `false` if the code point does not have the binary Unicode property, or if the
    /// implementation of ICU does not have data for the property.
    fn is<T>(self) -> bool
    where
        T: sealed::BinaryProperty;
}

impl sealed::BinaryProperty for Alphabetic {
    const SELECTOR: UProperty = UCHAR_ALPHABETIC;
}

impl sealed::BinaryProperty for Alphanumeric {
    const SELECTOR: UProperty = 0;

    fn for_char(c: char) -> bool {
        c.is::<Alphabetic>() || c.is::<Numeric>()
    }
}

impl sealed::BinaryProperty for Control {
    const SELECTOR: UProperty = 0;

    fn for_char(c: char) -> bool {
        matches!(c.general_category(), GeneralCategory::Control)
    }
}

impl sealed::BinaryProperty for Lowercase {
    const SELECTOR: UProperty = UCHAR_LOWERCASE;
}

impl sealed::BinaryProperty for Numeric {
    const SELECTOR: UProperty = 0;

    fn for_char(c: char) -> bool {
        matches!(
            c.general_category(),
            GeneralCategory::DecimalNumber
                | GeneralCategory::LetterNumber
                | GeneralCategory::OtherNumber
        )
    }
}

impl sealed::BinaryProperty for Uppercase {
    const SELECTOR: UProperty = UCHAR_UPPERCASE;
}

impl sealed::BinaryProperty for Whitespace {
    const SELECTOR: UProperty = UCHAR_WHITE_SPACE;
}

impl From<GeneralCategory> for GeneralCategoryGroup {
    #[inline]
    fn from(value: GeneralCategory) -> Self {
        match value {
            GeneralCategory::UppercaseLetter
            | GeneralCategory::LowercaseLetter
            | GeneralCategory::TitlecaseLetter
            | GeneralCategory::ModifierLetter
            | GeneralCategory::OtherLetter => Self::Letter,
            GeneralCategory::NonspacingMark
            | GeneralCategory::EnclosingMark
            | GeneralCategory::SpacingMark => Self::Mark,
            GeneralCategory::DecimalNumber
            | GeneralCategory::LetterNumber
            | GeneralCategory::OtherNumber => Self::Number,
            GeneralCategory::SpaceSeparator
            | GeneralCategory::LineSeparator
            | GeneralCategory::ParagraphSeparator => Self::Separator,
            GeneralCategory::Unassigned
            | GeneralCategory::Control
            | GeneralCategory::Format
            | GeneralCategory::PrivateUse
            | GeneralCategory::Surrogate => Self::Other,
            GeneralCategory::DashPunctuation
            | GeneralCategory::OpenPunctuation
            | GeneralCategory::ClosePunctuation
            | GeneralCategory::ConnectorPunctuation
            | GeneralCategory::OtherPunctuation
            | GeneralCategory::InitialPunctuation
            | GeneralCategory::FinalPunctuation => Self::Punctuation,
            GeneralCategory::MathSymbol
            | GeneralCategory::CurrencySymbol
            | GeneralCategory::ModifierSymbol
            | GeneralCategory::OtherSymbol => Self::Symbol,
        }
    }
}

impl UnicodeGeneralCategory for char {
    #[inline]
    fn general_category(self) -> GeneralCategory {
        let c = self as i32;
        // SAFETY: [`u_charType`] does not have any safety requirements.
        let category = unsafe { u_charType(c) };

        match u32::try_from(category).unwrap_or_default() {
            U_UPPERCASE_LETTER => GeneralCategory::UppercaseLetter,
            U_LOWERCASE_LETTER => GeneralCategory::LowercaseLetter,
            U_TITLECASE_LETTER => GeneralCategory::TitlecaseLetter,
            U_MODIFIER_LETTER => GeneralCategory::ModifierLetter,
            U_OTHER_LETTER => GeneralCategory::OtherLetter,
            U_NON_SPACING_MARK => GeneralCategory::NonspacingMark,
            U_ENCLOSING_MARK => GeneralCategory::EnclosingMark,
            U_COMBINING_SPACING_MARK => GeneralCategory::SpacingMark,
            U_DECIMAL_DIGIT_NUMBER => GeneralCategory::DecimalNumber,
            U_LETTER_NUMBER => GeneralCategory::LetterNumber,
            U_OTHER_NUMBER => GeneralCategory::OtherNumber,
            U_SPACE_SEPARATOR => GeneralCategory::SpaceSeparator,
            U_LINE_SEPARATOR => GeneralCategory::LineSeparator,
            U_PARAGRAPH_SEPARATOR => GeneralCategory::ParagraphSeparator,
            U_CONTROL_CHAR => GeneralCategory::Control,
            U_FORMAT_CHAR => GeneralCategory::Format,
            U_PRIVATE_USE_CHAR => GeneralCategory::PrivateUse,
            U_SURROGATE => GeneralCategory::Surrogate,
            U_DASH_PUNCTUATION => GeneralCategory::DashPunctuation,
            U_START_PUNCTUATION => GeneralCategory::OpenPunctuation,
            U_END_PUNCTUATION => GeneralCategory::ClosePunctuation,
            U_CONNECTOR_PUNCTUATION => GeneralCategory::ConnectorPunctuation,
            U_OTHER_PUNCTUATION => GeneralCategory::OtherPunctuation,
            U_MATH_SYMBOL => GeneralCategory::MathSymbol,
            U_CURRENCY_SYMBOL => GeneralCategory::CurrencySymbol,
            U_MODIFIER_SYMBOL => GeneralCategory::ModifierSymbol,
            U_OTHER_SYMBOL => GeneralCategory::OtherSymbol,
            U_INITIAL_PUNCTUATION => GeneralCategory::InitialPunctuation,
            U_FINAL_PUNCTUATION => GeneralCategory::FinalPunctuation,
            _ => GeneralCategory::Unassigned,
        }
    }
}

impl UnicodeProperties for char {
    #[inline]
    fn age(self) -> Option<[u8; 2]> {
        let c = self as i32;
        let mut version = UVersionInfo::default();

        // SAFETY: `versionArray` is a valid pointer to an array of 4 [`u8`] elements.
        unsafe { u_charAge(c, &mut version) };

        match version {
            [0, 0, 0, 0] => None,
            [major, minor, ..] => Some([major, minor]),
        }
    }

    #[inline]
    fn is<T>(self) -> bool
    where
        T: sealed::BinaryProperty,
    {
        T::for_char(self)
    }
}

/// Gets the Unicode version implemented by the ICU library.
///
/// For example, Unicode version 3.1.1 is represented as an array with the value `[3, 1, 1]`.
#[inline]
#[must_use]
pub fn unicode_version() -> [u8; 3] {
    let mut version = UVersionInfo::default();

    // SAFETY: `versionArray` is a valid pointer to an array of 4 [`u8`] elements.
    unsafe { u_getUnicodeVersion(&mut version) };

    let [major, minor, revision, _] = version;
    [major, minor, revision]
}

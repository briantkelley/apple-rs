//! # ICU
//!
//! This crate aims to provide idiomatic Rust bindings to Apple's ICU Clang module (located at
//! `$SDKROOT/usr/include/unicode/module.modulemap`, or in the Apple OSS Distributions [ICU][]
//! GitHub project), and aims to be a source-compatible, drop in replacement for various
//! [`unicode-rs`][] crates for the purpose of reducing binary size by relying on system-provided
//! Unicode databases.
//!
//! ## Examples
//!
//! ```
//! use icu::UnicodeGeneralCategory;
//!
//! let ch = '🦀'; // U+1F980 CRAB
//! let group = ch.general_category_group();
//! println!("{}({:?})", ch, group);
//! ```
//!
//! [ICU]: https://github.com/apple-oss-distributions/ICU/blob/ICU-74000.403/modules/ICU.modulemap
//! [`unicode-rs`]: https://github.com/unicode-rs

mod uchar;

pub use uchar::{
    unicode_version, Alphabetic, Alphanumeric, Control, GeneralCategory, GeneralCategoryGroup,
    Lowercase, Numeric, UnicodeGeneralCategory, UnicodeProperties, Uppercase, Whitespace,
};

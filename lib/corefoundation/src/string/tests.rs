#![allow(clippy::indexing_slicing, clippy::unwrap_used)]

use crate::cfstr;
use crate::string::String;

mod create;
mod get_bytes;
mod reader;

#[derive(Clone, Copy)]
#[repr(align(2))]
struct U16Align<const N: usize>([u8; N]);

#[derive(Clone, Copy)]
#[repr(align(4))]
struct U32Align<const N: usize>([u8; N]);

static EMPTY_STRING: &String = cfstr!("");

// The "POLAR BEAR" emoji (🐻‍❄️) is composed of the following four code points:
//
// 1. U+1F43B: BEAR FACE
// 2. U+0200D: ZERO WIDTH JOINER
// 3. U+02744: SNOWFLAKE
// 4. U+0FE0F: VARIATION SELECTOR-16
//
// These four code points require five (5) UTF-16 code units and thirteen (13) UTF-8 code units. The
// first code point, BEAR FACE, is a UTF-16 surrogate pair.

static POLAR_BEAR: &String = cfstr!("🐻‍❄️");

const POLAR_BEAR_UTF8: [u8; 13] = [
    0xf0, 0x9f, 0x90, 0xbb, 0xe2, 0x80, 0x8d, 0xe2, 0x9d, 0x84, 0xef, 0xb8, 0x8f,
];

const POLAR_BEAR_UTF16_BE: U16Align<10> =
    U16Align([0xd8, 0x3d, 0xdc, 0x3b, 0x20, 0x0d, 0x27, 0x44, 0xfe, 0x0f]);

const POLAR_BEAR_UTF16_LE: U16Align<10> =
    U16Align([0x3d, 0xd8, 0x3b, 0xdc, 0x0d, 0x20, 0x44, 0x27, 0x0f, 0xfe]);

const POLAR_BEAR_UTF16_NE: U16Align<10> = native_endian(POLAR_BEAR_UTF16_BE, POLAR_BEAR_UTF16_LE);

const POLAR_BEAR_UTF16_BE_BOM: U16Align<12> = U16Align([
    0xfe, 0xff, 0xd8, 0x3d, 0xdc, 0x3b, 0x20, 0x0d, 0x27, 0x44, 0xfe, 0x0f,
]);

const POLAR_BEAR_UTF16_LE_BOM: U16Align<12> = U16Align([
    0xff, 0xfe, 0x3d, 0xd8, 0x3b, 0xdc, 0x0d, 0x20, 0x44, 0x27, 0x0f, 0xfe,
]);

const POLAR_BEAR_UTF16_NE_BOM: U16Align<12> =
    native_endian(POLAR_BEAR_UTF16_BE_BOM, POLAR_BEAR_UTF16_LE_BOM);

const POLAR_BEAR_UTF32_BE: U32Align<16> = U32Align([
    0x00, 0x01, 0xf4, 0x3b, 0x00, 0x00, 0x20, 0x0d, 0x00, 0x00, 0x27, 0x44, 0x00, 0x00, 0xfe, 0x0f,
]);

const POLAR_BEAR_UTF32_LE: U32Align<16> = U32Align([
    0x3b, 0xf4, 0x01, 0x00, 0x0d, 0x20, 0x00, 0x00, 0x44, 0x27, 0x00, 0x00, 0x0f, 0xfe, 0x00, 0x00,
]);

const POLAR_BEAR_UTF32_NE: U32Align<16> = native_endian(POLAR_BEAR_UTF32_BE, POLAR_BEAR_UTF32_LE);

const POLAR_BEAR_UTF32_BE_BOM: U32Align<20> = U32Align([
    0x00, 0x00, 0xfe, 0xff, 0x00, 0x01, 0xf4, 0x3b, 0x00, 0x00, 0x20, 0x0d, 0x00, 0x00, 0x27, 0x44,
    0x00, 0x00, 0xfe, 0x0f,
]);

const POLAR_BEAR_UTF32_LE_BOM: U32Align<20> = U32Align([
    0xff, 0xfe, 0x00, 0x00, 0x3b, 0xf4, 0x01, 0x00, 0x0d, 0x20, 0x00, 0x00, 0x44, 0x27, 0x00, 0x00,
    0x0f, 0xfe, 0x00, 0x00,
]);

const POLAR_BEAR_UTF32_NE_BOM: U32Align<20> =
    native_endian(POLAR_BEAR_UTF32_BE_BOM, POLAR_BEAR_UTF32_LE_BOM);

macro_rules! impl_align {
    ($struct:ident) => {
        impl<const N: usize> AsRef<[u8]> for $struct<N> {
            fn as_ref(&self) -> &[u8] {
                &self.0
            }
        }

        impl<const N: usize> core::fmt::Debug for $struct<N> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <[u8; N] as core::fmt::Debug>::fmt(&self.0, f)
            }
        }

        impl<const N: usize> core::ops::Deref for $struct<N> {
            type Target = [u8];

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<const N: usize> PartialEq<[u8]> for $struct<N> {
            fn eq(&self, other: &[u8]) -> bool {
                <[u8] as PartialEq>::eq(self, other)
            }
        }

        impl<const N: usize> PartialEq<$struct<N>> for [u8] {
            fn eq(&self, other: &$struct<N>) -> bool {
                <[u8] as PartialEq>::eq(self, other)
            }
        }
    };
}

impl_align!(U16Align);
impl_align!(U32Align);

fn is_aligned<T>(p: *const T) -> bool {
    // LINT: [`align_of`] never returns 0, so the remainder operator won't panic.
    #[allow(clippy::arithmetic_side_effects)]
    // LINT: `_ as usize` is currently the only stable way to get the address.
    #[allow(clippy::as_conversions)]
    fn inner(p: *const (), align: usize) -> bool {
        (p as usize) % align == 0
    }
    inner(p.cast(), align_of::<T>())
}

#[cfg(target_endian = "big")]
const fn native_endian<T: Copy>(big: T, _little: T) -> T {
    big
}

#[cfg(target_endian = "little")]
const fn native_endian<T: Copy>(_big: T, little: T) -> T {
    little
}

const fn non_native_endian<T: Copy>(big: T, little: T) -> T {
    native_endian(little, big)
}

#[cfg(feature = "alloc")]
#[test]
fn as_str() {
    use alloc::borrow::Cow;

    assert_eq!(EMPTY_STRING.as_str(), "");
    assert_eq!(EMPTY_STRING.as_str(), Cow::Borrowed(""));

    assert_eq!(POLAR_BEAR.as_str(), "🐻‍❄️");
    assert_eq!(
        POLAR_BEAR.as_str(),
        Cow::Owned::<'_, str>("🐻‍❄️".to_owned())
    );

    assert_eq!(cfstr!("Hello, World!").as_str(), "Hello, World!");
    assert_eq!(
        cfstr!("Hello, World!").as_str(),
        Cow::Borrowed("Hello, World!")
    );
}

#[test]
fn eq() {
    assert_eq!(POLAR_BEAR, POLAR_BEAR);
    assert_eq!(POLAR_BEAR, &*String::from_utf8(POLAR_BEAR_UTF8).unwrap());
}

#[test]
fn to_string() {
    assert_eq!(EMPTY_STRING.to_string(), "");
    assert_eq!(POLAR_BEAR.to_string(), "🐻‍❄️");
    assert_eq!(cfstr!("Hello, World!").to_string(), "Hello, World!");
}

#[test]
fn try_as_str() {
    assert_eq!(EMPTY_STRING.try_as_str(), Some(""));

    assert!(POLAR_BEAR.try_as_str().is_none());

    assert_eq!(cfstr!("Hello, World!").try_as_str(), Some("Hello, World!"));
}

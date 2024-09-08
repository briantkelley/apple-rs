use super::{
    is_aligned, non_native_endian, POLAR_BEAR, POLAR_BEAR_UTF16_BE, POLAR_BEAR_UTF16_BE_BOM,
    POLAR_BEAR_UTF16_LE, POLAR_BEAR_UTF16_LE_BOM, POLAR_BEAR_UTF16_NE, POLAR_BEAR_UTF16_NE_BOM,
    POLAR_BEAR_UTF32_BE, POLAR_BEAR_UTF32_BE_BOM, POLAR_BEAR_UTF32_LE, POLAR_BEAR_UTF32_LE_BOM,
    POLAR_BEAR_UTF32_NE, POLAR_BEAR_UTF32_NE_BOM, POLAR_BEAR_UTF8,
};
use crate::cfstr;
use crate::string::{CharacterSet, FromUtfByteOrder, String};
use core::slice;

#[test]
fn from_bytes() {
    const BYTES: [u8; 7] = [0xc0, 0xd2, 0xa6, 0xd3, 0xb7, 0xee, 0xf4];

    assert_eq!(
        String::from_bytes(BYTES, CharacterSet::MacRoman).unwrap(),
        cfstr!("¿“¶”∑ÓÙ")
    );
}

#[test]
fn from_invalid_bytes() {
    let _ = String::from_bytes([0x81, 0x81], CharacterSet::TraditionalChinese).unwrap_err();
}

#[test]
fn from_str() {
    assert_eq!(String::from_str("🐻‍❄️"), POLAR_BEAR);
    assert_eq!(String::from_utf8(POLAR_BEAR_UTF8).unwrap(), POLAR_BEAR);
}

#[test]
fn from_invalid_utf8() {
    let _ = String::from_utf8(POLAR_BEAR_UTF16_BE).unwrap_err();
    let _ = String::from_utf8(POLAR_BEAR_UTF16_LE).unwrap_err();
}

#[test]
fn from_utf16() {
    assert_eq!(
        String::from_utf16(as_slice(&POLAR_BEAR_UTF16_BE), FromUtfByteOrder::BigEndian),
        POLAR_BEAR
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&POLAR_BEAR_UTF16_LE),
            FromUtfByteOrder::LittleEndian
        ),
        POLAR_BEAR
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&POLAR_BEAR_UTF16_BE_BOM),
            FromUtfByteOrder::ByteOrderMark
        ),
        POLAR_BEAR
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&POLAR_BEAR_UTF16_LE_BOM),
            FromUtfByteOrder::ByteOrderMark
        ),
        POLAR_BEAR
    );

    assert_eq!(
        String::from_utf16(as_slice(&POLAR_BEAR_UTF16_NE), FromUtfByteOrder::HostNative),
        POLAR_BEAR
    );
}

#[test]
fn from_invalid_utf16() {
    const SURROGATE_HIGH: u16 = 0xd83d;
    const SURROGATE_LOW: u16 = 0xdc3b;

    assert_eq!(
        String::from_utf16(
            [SURROGATE_LOW.to_be(), SURROGATE_HIGH.to_be()],
            FromUtfByteOrder::BigEndian
        )
        .len(),
        2
    );

    assert_eq!(
        String::from_utf16(
            [SURROGATE_LOW.to_le(), SURROGATE_HIGH.to_le()],
            FromUtfByteOrder::LittleEndian
        )
        .len(),
        2
    );

    assert_eq!(
        String::from_utf16(
            [0x1234, SURROGATE_LOW, SURROGATE_HIGH],
            FromUtfByteOrder::ByteOrderMark
        )
        .len(),
        3
    );

    assert_eq!(
        String::from_utf16(
            [SURROGATE_LOW, SURROGATE_HIGH],
            FromUtfByteOrder::HostNative
        )
        .len(),
        2
    );

    assert_eq!(
        String::from_utf16(as_slice(&POLAR_BEAR_UTF16_LE), FromUtfByteOrder::BigEndian).len(),
        5
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&POLAR_BEAR_UTF16_BE),
            FromUtfByteOrder::LittleEndian
        )
        .len(),
        5
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&POLAR_BEAR_UTF16_LE_BOM),
            FromUtfByteOrder::BigEndian
        )
        .len(),
        6
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&POLAR_BEAR_UTF16_BE_BOM),
            FromUtfByteOrder::LittleEndian
        )
        .len(),
        6
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&non_native_endian(POLAR_BEAR_UTF16_BE, POLAR_BEAR_UTF16_LE)),
            FromUtfByteOrder::HostNative
        )
        .len(),
        5
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&non_native_endian(
                POLAR_BEAR_UTF16_BE_BOM,
                POLAR_BEAR_UTF16_LE_BOM
            )),
            FromUtfByteOrder::HostNative
        )
        .len(),
        6
    );

    assert_eq!(
        String::from_utf16(
            as_slice(&POLAR_BEAR_UTF16_NE_BOM),
            FromUtfByteOrder::HostNative
        )
        .len(),
        6
    );
}

#[test]
fn from_utf32() {
    assert_eq!(
        String::from_utf32(as_slice(&POLAR_BEAR_UTF32_BE), FromUtfByteOrder::BigEndian).unwrap(),
        POLAR_BEAR
    );

    assert_eq!(
        String::from_utf32(
            as_slice(&POLAR_BEAR_UTF32_LE),
            FromUtfByteOrder::LittleEndian
        )
        .unwrap(),
        POLAR_BEAR
    );

    assert_eq!(
        String::from_utf32(
            as_slice(&POLAR_BEAR_UTF32_BE_BOM),
            FromUtfByteOrder::ByteOrderMark
        )
        .unwrap(),
        POLAR_BEAR
    );

    assert_eq!(
        String::from_utf32(
            as_slice(&POLAR_BEAR_UTF32_LE_BOM),
            FromUtfByteOrder::ByteOrderMark
        )
        .unwrap(),
        POLAR_BEAR
    );

    assert_eq!(
        String::from_utf32(as_slice(&POLAR_BEAR_UTF32_NE), FromUtfByteOrder::HostNative).unwrap(),
        POLAR_BEAR
    );
}

#[test]
fn from_invalid_utf32() {
    let _ = String::from_utf32(as_slice(&POLAR_BEAR_UTF32_LE), FromUtfByteOrder::BigEndian)
        .unwrap_err();

    let _ = String::from_utf32(
        as_slice(&POLAR_BEAR_UTF32_BE),
        FromUtfByteOrder::LittleEndian,
    )
    .unwrap_err();

    assert_eq!(
        String::from_utf32(
            as_slice(&POLAR_BEAR_UTF32_BE_BOM),
            FromUtfByteOrder::BigEndian
        )
        .unwrap()
        .len(),
        6
    );

    let _ = String::from_utf32(
        as_slice(&POLAR_BEAR_UTF32_LE_BOM),
        FromUtfByteOrder::BigEndian,
    )
    .unwrap_err();

    let _ = String::from_utf32(
        as_slice(&POLAR_BEAR_UTF32_BE_BOM),
        FromUtfByteOrder::LittleEndian,
    )
    .unwrap_err();

    assert_eq!(
        String::from_utf32(
            as_slice(&POLAR_BEAR_UTF32_LE_BOM),
            FromUtfByteOrder::LittleEndian,
        )
        .unwrap()
        .len(),
        6
    );

    assert_eq!(
        String::from_utf32(
            [0x1234_u32.to_be(), 0x5678_u32.to_be()],
            FromUtfByteOrder::ByteOrderMark
        )
        .unwrap()
        .len(),
        2
    );

    assert_eq!(
        String::from_utf32(
            as_slice(&POLAR_BEAR_UTF32_NE_BOM),
            FromUtfByteOrder::HostNative,
        )
        .unwrap()
        .len(),
        6
    );

    let _ = String::from_utf32([0x11_0000], FromUtfByteOrder::HostNative).unwrap_err();
}

// LINT: Panicking on a zero-sized type is fine, as the condition is unexpected.
#[allow(clippy::arithmetic_side_effects)]
fn as_slice<T>(v: &[u8]) -> &[T] {
    let data = v.as_ptr().cast();
    let byte_len = v.len();

    assert!(is_aligned(data), "v is not properly aligned for T");
    assert_eq!(
        byte_len % size_of::<T>(),
        0,
        "v.len() is not a multiple of size_of::<T>()"
    );

    let len = byte_len / size_of::<T>();

    // SAFETY: [`u8`]'s alignment requirements are less than or equal to `T`'s, the new slice covers
    // the exact same region of memory as `v`, and we are only transmuting the type of shared
    // reference to the memory region.
    unsafe { slice::from_raw_parts(data, len) }
}

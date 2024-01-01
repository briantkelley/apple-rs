use super::{POLAR_BEAR, POLAR_BEAR_UTF16_NE_BOM, POLAR_BEAR_UTF32_NE_BOM};
use crate::string::{
    FromUtfByteOrder, GetBytesByteOrder, GetBytesEncoding, GetBytesLossyReader, GetBytesReader,
    GetBytesReaderSummary, GetBytesStrReader, GetBytesStrReplacement, String,
};
use core::mem::size_of;

const POLAR_BEAR_UTF16_CODE_UNITS: [u16; 5] = [0xd83d, 0xdc3b, 0x200d, 0x2744, 0xfe0f];

#[test]
fn str_reader() {
    let mut buf = [0_u8; 16];

    let mut reader = GetBytesStrReader::new(POLAR_BEAR, GetBytesStrReplacement::None, ..);
    assert_eq!(reader.read(&mut buf), Some("🐻‍❄️"));
    assert!(reader.read(&mut buf).is_none());

    let mut reader = GetBytesStrReader::new(POLAR_BEAR, GetBytesStrReplacement::None, ..);
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{1f43b}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
    assert!(reader.read(&mut buf).is_none());

    assert_eq!(
        GetBytesReader::new(POLAR_BEAR, GetBytesEncoding::Utf8, ..).collect(),
        GetBytesReaderSummary {
            buf_len: 13,
            loss_char_count: 0
        }
    );
}

#[test]
fn str_reader_replacement_none() {
    let mut buf = [0_u8; 16];
    let replacement = GetBytesStrReplacement::None;

    for index in 0..2 {
        let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
        let _ = v.remove(index);
        let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

        let mut reader = GetBytesStrReader::new(&s, replacement, ..);
        assert_eq!(reader.read(&mut buf), Some("\u{0200d}\u{02744}\u{0fe0f}"));
        assert!(reader.read(&mut buf).is_none());

        let mut reader = GetBytesStrReader::new(&s, replacement, ..);
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
        assert!(reader.read(&mut buf).is_none());

        assert_eq!(
            GetBytesReader::new(&s, GetBytesEncoding::Utf8, ..).collect(),
            GetBytesReaderSummary {
                buf_len: 9,
                loss_char_count: 1
            }
        );
    }

    let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
    v.insert(3, 0xd83d);
    let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(
        reader.read(&mut buf),
        Some("\u{1f43b}\u{0200d}\u{02744}\u{0fe0f}")
    );
    assert!(reader.read(&mut buf).is_none());

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{1f43b}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
    assert!(reader.read(&mut buf).is_none());

    assert_eq!(
        GetBytesReader::new(&s, GetBytesEncoding::Utf8, ..).collect(),
        GetBytesReaderSummary {
            buf_len: 13,
            loss_char_count: 1
        }
    );

    let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
    v.push(0xd83d);
    let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(
        reader.read(&mut buf),
        Some("\u{1f43b}\u{0200d}\u{02744}\u{0fe0f}")
    );
    assert!(reader.read(&mut buf).is_none());

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{1f43b}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
    assert!(reader.read(&mut buf).is_none());

    assert_eq!(
        GetBytesReader::new(&s, GetBytesEncoding::Utf8, ..).collect(),
        GetBytesReaderSummary {
            buf_len: 13,
            loss_char_count: 1
        }
    );
}

#[test]
fn str_reader_replacement_default() {
    let mut buf = [0_u8; 16];
    let replacement = GetBytesStrReplacement::UnicodeReplacement;

    for index in 0..2 {
        let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
        let _ = v.remove(index);
        let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

        let mut reader = GetBytesStrReader::new(&s, replacement, ..);
        assert_eq!(
            reader.read(&mut buf),
            Some("\u{0fffd}\u{0200d}\u{02744}\u{0fe0f}")
        );
        assert!(reader.read(&mut buf).is_none());

        let mut reader = GetBytesStrReader::new(&s, replacement, ..);
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fffd}"));
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
        assert!(reader.read(&mut buf).is_none());
    }

    let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
    v.insert(3, 0xd83d);
    let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(
        reader.read(&mut buf),
        Some("\u{1f43b}\u{0200d}\u{0fffd}\u{02744}\u{0fe0f}")
    );
    assert!(reader.read(&mut buf).is_none());

    let mut foo = [0_u8; 16];
    let _ = char::from_u32(0x200d).unwrap().encode_utf8(&mut foo);
    foo[4] = b'?';
    let x = core::str::from_utf8(&foo[0..4]).unwrap();
    dbg!(x);

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{1f43b}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fffd}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
    assert!(reader.read(&mut buf).is_none());

    let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
    v.push(0xd83d);
    let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(
        reader.read(&mut buf),
        Some("\u{1f43b}\u{0200d}\u{02744}\u{0fe0f}\u{0fffd}")
    );
    assert!(reader.read(&mut buf).is_none());

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{1f43b}"));
    assert_eq!(reader.read(&mut buf[..3]), Some("\u{0200d}"));
    assert_eq!(reader.read(&mut buf[..3]), Some("\u{02744}"));
    assert_eq!(reader.read(&mut buf[..3]), Some("\u{0fe0f}"));
    assert_eq!(reader.read(&mut buf[..3]), Some("\u{0fffd}"));
    assert!(reader.read(&mut buf).is_none());
}

#[test]
fn str_reader_replacement_custom() {
    let mut buf = [0_u8; 16];
    let replacement = GetBytesStrReplacement::Custom("?-?");

    for index in 0..2 {
        let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
        let _ = v.remove(index);
        let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

        let mut reader = GetBytesStrReader::new(&s, replacement, ..);
        assert_eq!(
            reader.read(&mut buf),
            Some("?-?\u{0200d}\u{02744}\u{0fe0f}")
        );
        assert!(reader.read(&mut buf).is_none());

        let mut reader = GetBytesStrReader::new(&s, replacement, ..);
        assert_eq!(reader.read(&mut buf[..4]), Some("?-?"));
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
        assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
        assert!(reader.read(&mut buf).is_none());
    }

    let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
    v.insert(3, 0xd83d);
    let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(
        reader.read(&mut buf),
        Some("\u{1f43b}\u{0200d}?-?\u{02744}\u{0fe0f}")
    );
    assert!(reader.read(&mut buf).is_none());

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{1f43b}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("?-?"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
    assert!(reader.read(&mut buf).is_none());

    let mut v = POLAR_BEAR_UTF16_CODE_UNITS.to_vec();
    v.push(0xd83d);
    let s = String::from_utf16(v, FromUtfByteOrder::HostNative);

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(
        reader.read(&mut buf),
        Some("\u{1f43b}\u{0200d}\u{02744}\u{0fe0f}?-?")
    );
    assert!(reader.read(&mut buf).is_none());

    let mut reader = GetBytesStrReader::new(&s, replacement, ..);
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{1f43b}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0200d}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{02744}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("\u{0fe0f}"));
    assert_eq!(reader.read(&mut buf[..4]), Some("?-?"));
    assert!(reader.read(&mut buf).is_none());
}

#[test]
fn utf16_bom() {
    let mut buf = [0_u8; 16];
    let encoding = GetBytesEncoding::Utf16 {
        byte_order: GetBytesByteOrder::HostNative { include_bom: true },
    };

    let mut reader = GetBytesLossyReader::new(POLAR_BEAR, encoding, None, ..);
    assert_eq!(
        reader.read(&mut buf),
        Some(POLAR_BEAR_UTF16_NE_BOM.as_ref())
    );
    assert!(reader.read(&mut buf).is_none());

    let mut reader = GetBytesLossyReader::new(POLAR_BEAR, encoding, None, ..);
    let mut index = 0;

    while let Some(next) = reader.read(&mut buf[..2]) {
        for byte_index in 0..size_of::<u16>() {
            assert_eq!(
                next[byte_index],
                POLAR_BEAR_UTF16_NE_BOM[index + byte_index]
            );
        }
        index += size_of::<u16>();
    }
    assert_eq!(index, POLAR_BEAR_UTF16_NE_BOM.len());

    assert_eq!(
        GetBytesReader::new(POLAR_BEAR, encoding, ..).collect(),
        GetBytesReaderSummary {
            buf_len: 12,
            loss_char_count: 0
        }
    );
}

#[test]
fn utf32_bom() {
    let mut buf = [0_u8; 20];
    let encoding = GetBytesEncoding::Utf32 {
        byte_order: GetBytesByteOrder::HostNative { include_bom: true },
        loss_byte: None,
    };

    let mut reader = GetBytesLossyReader::new(POLAR_BEAR, encoding, None, ..);
    assert_eq!(
        reader.read(&mut buf),
        Some(POLAR_BEAR_UTF32_NE_BOM.as_ref())
    );
    assert!(reader.read(&mut buf).is_none());

    let mut reader = GetBytesLossyReader::new(POLAR_BEAR, encoding, None, ..);
    let mut index = 0;

    while let Some(next) = reader.read(&mut buf[..4]) {
        for byte_index in 0..size_of::<u32>() {
            assert_eq!(
                next[byte_index],
                POLAR_BEAR_UTF32_NE_BOM[index + byte_index]
            );
        }
        index += size_of::<u32>();
    }
    assert_eq!(index, POLAR_BEAR_UTF32_NE_BOM.len());

    assert_eq!(
        GetBytesReader::new(POLAR_BEAR, encoding, ..).collect(),
        GetBytesReaderSummary {
            buf_len: 20,
            loss_char_count: 0
        }
    );
}

#[should_panic(expected = "buffer too small to hold a code point")]
#[test]
fn buf_none() {
    let mut buf = [0_u8; 1];

    let mut reader = GetBytesStrReader::new(POLAR_BEAR, GetBytesStrReplacement::None, ..);
    let _ = reader.read(&mut buf);
}

#[should_panic(expected = "buffer too small for lossy character replacement")]
#[test]
fn buf_too_small() {
    let mut buf = [0_u8; 1];

    let s = String::from_utf16([0xd83d], FromUtfByteOrder::HostNative);
    let mut reader = GetBytesStrReader::new(&s, GetBytesStrReplacement::Custom("Too Long"), ..);

    let _ = reader.read(&mut buf);
}

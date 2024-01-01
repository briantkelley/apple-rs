use super::{
    is_aligned, EMPTY_STRING, POLAR_BEAR, POLAR_BEAR_UTF16_BE, POLAR_BEAR_UTF16_LE,
    POLAR_BEAR_UTF16_NE, POLAR_BEAR_UTF16_NE_BOM, POLAR_BEAR_UTF32_BE, POLAR_BEAR_UTF32_LE,
    POLAR_BEAR_UTF32_NE, POLAR_BEAR_UTF32_NE_BOM, POLAR_BEAR_UTF8,
};
use crate::cfstr;
use crate::string::{
    CharacterSet, FromUtfByteOrder, GetBytesByteOrder, GetBytesEncoding, GetBytesError,
    GetBytesErrorKind, GetBytesResult, GetBytesSurrogateError, String,
};
use core::num::NonZeroU8;

static EMPTY_RESULT: GetBytesResult = GetBytesResult {
    buf_len: 0,
    remaining: None,
};

static POLAR_BEAR_WITH_ASCII: &String = cfstr!("[🐻‍❄️]");

#[test]
fn get_bytes_bom_buf() {
    let mut buf = [0_u8; 32];

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::HostNative { include_bom: true },
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf_len, 12);
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF16_NE_BOM);
    assert_eq!(buf[buf_len..], [0; 20]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::HostNative { include_bom: true },
                loss_byte: None,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf_len, 20);
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF32_NE_BOM);
    assert_eq!(buf[buf_len..], [0; 12]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);
}

#[test]
fn get_bytes_bom_utf16_buf_no_code_points() {
    let mut buf = [0xcc_u8; 4];

    let encoding = GetBytesEncoding::Utf16 {
        byte_order: GetBytesByteOrder::HostNative { include_bom: true },
    };

    for len in 0..2 {
        assert_eq!(
            POLAR_BEAR.get_bytes(.., encoding, Some(&mut buf[..len])),
            Ok(GetBytesResult {
                buf_len: 0,
                remaining: Some(0..5)
            })
        );
        assert_eq!(buf, [0xcc; 4]); // verify buffer was not written to

        assert_eq!(
            POLAR_BEAR.get_bytes_unchecked(.., encoding, Some(&mut buf[..len])),
            GetBytesResult {
                buf_len: 0,
                remaining: Some(0..5)
            }
        );
        assert_eq!(buf, [0xcc; 4]); // verify buffer was not written to
    }

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(.., encoding, Some(&mut buf[..2]))
        .unwrap();
    assert_eq!(buf_len, 2);
    assert_eq!(buf[..2], 0xfeff_u16.to_ne_bytes());
    assert_eq!(buf[2..], [0xcc; 2]); // verify buffer was not written to
    assert_eq!(remaining, Some(0..5));
    buf.fill(0xcc);

    let GetBytesResult { buf_len, remaining } =
        POLAR_BEAR.get_bytes_unchecked(.., encoding, Some(&mut buf[..2]));
    assert_eq!(buf_len, 2);
    assert_eq!(buf[..2], 0xfeff_u16.to_ne_bytes());
    assert_eq!(buf[2..], [0xcc; 2]); // verify buffer was not written to
    assert_eq!(remaining, Some(0..5));
}

#[test]
fn get_bytes_bom_utf32_buf_no_code_points() {
    let mut buf = [0xcc_u8; 8];

    let encoding = GetBytesEncoding::Utf32 {
        byte_order: GetBytesByteOrder::HostNative { include_bom: true },
        loss_byte: None,
    };

    for len in 0..4 {
        assert_eq!(
            POLAR_BEAR.get_bytes(.., encoding, Some(&mut buf[..len])),
            Ok(GetBytesResult {
                buf_len: 0,
                remaining: Some(0..5)
            })
        );
        assert_eq!(buf, [0xcc; 8]);

        assert_eq!(
            POLAR_BEAR.get_bytes_unchecked(.., encoding, Some(&mut buf[..len])),
            GetBytesResult {
                buf_len: 0,
                remaining: Some(0..5)
            }
        );
        assert_eq!(buf, [0xcc; 8]);
    }

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(.., encoding, Some(&mut buf[..4]))
        .unwrap();
    assert_eq!(buf_len, 4);
    assert_eq!(buf[..4], 0xfeff_u32.to_ne_bytes());
    assert_eq!(buf[4..], [0xcc; 4]); // verify buffer was not written to
    assert_eq!(remaining, Some(0..5));
    buf.fill(0xcc);

    let GetBytesResult { buf_len, remaining } =
        POLAR_BEAR.get_bytes_unchecked(.., encoding, Some(&mut buf[..4]));
    assert_eq!(buf_len, 4);
    assert_eq!(buf[..4], 0xfeff_u32.to_ne_bytes());
    assert_eq!(buf[4..], [0xcc; 4]); // verify buffer was not written to
    assert_eq!(remaining, Some(0..5));
}

#[test]
fn get_bytes_buf_empty() {
    let mut buf = [0_u8; 0];

    assert_eq!(
        EMPTY_STRING.get_bytes(.., GetBytesEncoding::Utf8, Some(&mut buf)),
        Ok(EMPTY_RESULT.clone())
    );

    assert_eq!(
        POLAR_BEAR.get_bytes(.., GetBytesEncoding::Utf8, Some(&mut buf)),
        Ok(GetBytesResult {
            buf_len: 0,
            remaining: Some(0..5)
        })
    );
}

#[test]
fn get_bytes_buf_too_small() {
    let mut buf = [0_u8; 16];

    assert_eq!(
        POLAR_BEAR.get_bytes(.., GetBytesEncoding::Utf8, Some(&mut buf[0..3])),
        Ok(GetBytesResult {
            buf_len: 0,
            remaining: Some(0..5)
        })
    );
    assert_eq!(buf, [0; 16]); // verify buffer was not written to

    assert_eq!(
        POLAR_BEAR.get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::HostNative { include_bom: false }
            },
            Some(&mut buf[0..1])
        ),
        Ok(GetBytesResult {
            buf_len: 0,
            remaining: Some(0..5)
        })
    );
    assert_eq!(buf, [0; 16]); // verify buffer was not written to

    // In my opinion this should fail because it splits a surrogate pair, but Core Foundation
    // allows it, even though it fails for UTF-8 and UTF-32.
    assert_eq!(
        POLAR_BEAR.get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::LittleEndian
            },
            Some(&mut buf[0..3])
        ),
        Ok(GetBytesResult {
            buf_len: 2,
            remaining: Some(1..5),
        })
    );
    assert_eq!(buf[..2], POLAR_BEAR_UTF16_LE[..2]);
    assert_eq!(buf[2..], [0; 14]);
    buf.fill(0);

    assert_eq!(
        POLAR_BEAR.get_bytes(
            ..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::HostNative { include_bom: false },
                loss_byte: None,
            },
            Some(&mut buf[0..3])
        ),
        Ok(GetBytesResult {
            buf_len: 0,
            remaining: Some(0..5)
        })
    );
    assert_eq!(buf, [0; 16]); // verify buffer was not written to
}

#[test]
fn get_bytes_cannot_convert_lossless_buf_none() {
    let mac_roman = GetBytesEncoding::CharacterSet {
        character_set: CharacterSet::MacRoman,
        loss_byte: None,
    };

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(.., mac_roman, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Character {
                c: char::from_u32(0x1f43b).unwrap(),
                range: 1..3
            },
            result: GetBytesResult {
                buf_len: 1,
                remaining: Some(3..7)
            }
        })
    );

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(2.., mac_roman, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Surrogate {
                reason: GetBytesSurrogateError::Range,
                index: 2
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(3..7)
            }
        })
    );

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(3.., mac_roman, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Character {
                c: char::from_u32(0x0200d).unwrap(),
                range: 3..4
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(4..7)
            }
        })
    );

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(4.., mac_roman, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Character {
                c: char::from_u32(0x02744).unwrap(),
                range: 4..5
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(5..7)
            }
        })
    );

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(5.., mac_roman, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Character {
                c: char::from_u32(0x0fe0f).unwrap(),
                range: 5..6
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(6..7)
            }
        })
    );

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(6.., mac_roman, None),
        Ok(GetBytesResult {
            buf_len: 1,
            remaining: None
        })
    );
}

#[test]
fn get_bytes_cannot_convert_lossless_buf() {
    let mut buf = [0_u8; 16];
    let mac_roman = GetBytesEncoding::CharacterSet {
        character_set: CharacterSet::MacRoman,
        loss_byte: None,
    };

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(.., mac_roman, Some(&mut buf)),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Character {
                c: char::from_u32(0x1f43b).unwrap(),
                range: 1..3
            },
            result: GetBytesResult {
                buf_len: 1,
                remaining: Some(3..7)
            }
        })
    );
    assert_eq!(buf[..1], [b'[']);
    assert_eq!(buf[1..], [0; 15]); // verify buffer was not written to
    buf.fill(0);

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(2.., mac_roman, Some(&mut buf)),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Surrogate {
                reason: GetBytesSurrogateError::Range,
                index: 2
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(3..7)
            }
        })
    );
    assert_eq!(buf, [0; 16]); // verify buffer was not written to

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(3.., mac_roman, Some(&mut buf)),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Character {
                c: char::from_u32(0x0200d).unwrap(),
                range: 3..4
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(4..7)
            }
        })
    );
    assert_eq!(buf, [0; 16]); // verify buffer was not written to

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(4.., mac_roman, Some(&mut buf)),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Character {
                c: char::from_u32(0x02744).unwrap(),
                range: 4..5
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(5..7)
            }
        })
    );
    assert_eq!(buf, [0; 16]); // verify buffer was not written to

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(5.., mac_roman, Some(&mut buf)),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Character {
                c: char::from_u32(0x0fe0f).unwrap(),
                range: 5..6
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(6..7)
            }
        })
    );
    assert_eq!(buf, [0; 16]); // verify buffer was not written to

    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(6.., mac_roman, Some(&mut buf)),
        Ok(GetBytesResult {
            buf_len: 1,
            remaining: None
        })
    );
    assert_eq!(buf[..1], [b']']);
    assert_eq!(buf[1..], [0; 15]); // verify buffer was not written to
}

#[test]
fn get_bytes_cannot_convert_lossy_buf_none() {
    assert_eq!(
        POLAR_BEAR_WITH_ASCII.get_bytes(
            ..,
            GetBytesEncoding::CharacterSet {
                character_set: CharacterSet::MacRoman,
                loss_byte: NonZeroU8::new(0xf0),
            },
            None
        ),
        Ok(GetBytesResult {
            buf_len: 7,
            remaining: None
        })
    );
}

#[test]
fn get_bytes_cannot_convert_lossy_buf() {
    let mut buf = [0_u8; 16];

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR_WITH_ASCII
        .get_bytes(
            ..,
            GetBytesEncoding::CharacterSet {
                character_set: CharacterSet::MacRoman,
                loss_byte: NonZeroU8::new(0xf0),
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], [b'[', 0xf0, 0xf0, 0xf0, 0xf0, 0xf0, b']']);
    assert_eq!(buf[buf_len..], [0; 9]); // verify buffer was not written to
    assert!(remaining.is_none());
}

#[test]
fn get_bytes_utf8_orphan_surrogate_buf() {
    let mut buf = [0_u8; 16];

    let GetBytesError { kind, result } = POLAR_BEAR_WITH_ASCII
        .get_bytes(..2, GetBytesEncoding::Utf8, Some(&mut buf))
        .unwrap_err();
    assert_eq!(
        kind,
        GetBytesErrorKind::Surrogate {
            reason: GetBytesSurrogateError::Range,
            index: 1
        }
    );
    assert_eq!(buf[..result.buf_len], [b'[']);
    assert_eq!(buf[result.buf_len..], [0; 15]); // verify buffer was not written to
    assert!(result.remaining.is_none());
    buf.fill(0);

    let GetBytesError { kind, result } = POLAR_BEAR_WITH_ASCII
        .get_bytes(1..2, GetBytesEncoding::Utf8, Some(&mut buf))
        .unwrap_err();
    assert_eq!(
        kind,
        GetBytesErrorKind::Surrogate {
            reason: GetBytesSurrogateError::Range,
            index: 1
        }
    );
    assert_eq!(
        result,
        GetBytesResult {
            buf_len: 0,
            remaining: None
        }
    );
    assert_eq!(buf, [0_u8; 16]); // verify buffer was not written to

    let GetBytesError { kind, result } = POLAR_BEAR_WITH_ASCII
        .get_bytes(2.., GetBytesEncoding::Utf8, Some(&mut buf))
        .unwrap_err();
    assert_eq!(
        kind,
        GetBytesErrorKind::Surrogate {
            reason: GetBytesSurrogateError::Range,
            index: 2
        }
    );
    assert_eq!(
        result,
        GetBytesResult {
            buf_len: 0,
            remaining: Some(3..7)
        }
    );
    assert_eq!(buf, [0_u8; 16]); // verify buffer was not written to
}

#[test]
fn get_bytes_utf16_orphan_surrogate_buf() {
    let mut buf = [0_u8; 16];

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR_WITH_ASCII
        .get_bytes(
            ..2,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::LittleEndian,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], [b'[', 0x00, 0x3d, 0xd8]);
    assert_eq!(buf[buf_len..], [0; 12]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR_WITH_ASCII
        .get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::LittleEndian,
            },
            Some(&mut buf[0..4]),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], [b'[', 0x00, 0x3d, 0xd8]);
    assert_eq!(buf[buf_len..], [0; 12]); // verify buffer was not written to
    assert_eq!(remaining, Some(2..7));
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR_WITH_ASCII
        .get_bytes(
            2..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::LittleEndian,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(
        &buf[..buf_len],
        &[0x3b, 0xdc, 0x0d, 0x20, 0x44, 0x27, 0x0f, 0xfe, b']', 0x00]
    );
    assert_eq!(buf[buf_len..], [0; 6]); // verify buffer was not written to
    assert!(remaining.is_none());
}

#[test]
fn get_bytes_utf32_orphan_surrogate_buf() {
    let mut buf = [0_u8; 32];

    let GetBytesError { kind, result } = POLAR_BEAR_WITH_ASCII
        .get_bytes(
            ..2,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::LittleEndian,
                loss_byte: None,
            },
            Some(&mut buf),
        )
        .unwrap_err();
    assert_eq!(
        kind,
        GetBytesErrorKind::Surrogate {
            reason: GetBytesSurrogateError::Range,
            index: 1
        }
    );

    assert_eq!(buf[..result.buf_len], [b'[', 0x00, 0x00, 0x00]);
    assert_eq!(buf[result.buf_len..], [0; 28]); // verify buffer was not written to
    assert!(result.remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR_WITH_ASCII
        .get_bytes(
            ..2,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::LittleEndian,
                loss_byte: NonZeroU8::new(b'?'),
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(
        &buf[..buf_len],
        &[b'[', 0x00, 0x00, 0x00, b'?', 0x00, 0x00, 0x00],
    );
    assert_eq!(buf[buf_len..], [0; 24]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesError { kind, result } = POLAR_BEAR_WITH_ASCII
        .get_bytes(
            2..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::LittleEndian,
                loss_byte: None,
            },
            Some(&mut buf),
        )
        .unwrap_err();
    assert_eq!(
        kind,
        GetBytesErrorKind::Surrogate {
            reason: GetBytesSurrogateError::Range,
            index: 2
        }
    );
    assert_eq!(
        result,
        GetBytesResult {
            buf_len: 0,
            remaining: Some(3..7)
        }
    );
    assert_eq!(buf, [0; 32]); // verify buffer was not written to

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR_WITH_ASCII
        .get_bytes(
            2..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::LittleEndian,
                loss_byte: NonZeroU8::new(b'?'),
            },
            Some(&mut buf[..16]),
        )
        .unwrap();
    assert_eq!(
        &buf[..buf_len],
        &[
            b'?', 0x00, 0x00, 0x00, 0x0d, 0x20, 0x00, 0x00, 0x44, 0x27, 0x00, 0x00, 0x0f, 0xfe,
            0x00, 0x00,
        ],
    );
    assert_eq!(buf[buf_len..], [0_u8; 16]); // verify buffer was not written to
    assert_eq!(remaining, Some(6..7));
}

#[test]
fn get_bytes_range_full() {
    let mut buf = [0_u8; 32];

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(.., GetBytesEncoding::Utf8, Some(&mut buf))
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF8);
    assert_eq!(buf[buf_len..], [0; 19]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::BigEndian,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF16_BE);
    assert_eq!(buf[buf_len..], [0; 22]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::LittleEndian,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF16_LE);
    assert_eq!(buf[buf_len..], [0; 22]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::HostNative { include_bom: false },
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF16_NE);
    assert_eq!(buf[buf_len..], [0; 22]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::BigEndian,
                loss_byte: None,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF32_BE);
    assert_eq!(buf[buf_len..], [0_u8; 16]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::LittleEndian,
                loss_byte: None,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF32_LE);
    assert_eq!(buf[buf_len..], [0_u8; 16]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::HostNative { include_bom: false },
                loss_byte: None,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF32_NE);
    assert_eq!(buf[buf_len..], [0_u8; 16]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);
}

#[test]
fn get_bytes_range_full_buf_none() {
    assert_eq!(
        POLAR_BEAR.get_bytes(.., GetBytesEncoding::Utf8, None),
        Ok(GetBytesResult {
            buf_len: POLAR_BEAR_UTF8.len(),
            remaining: None
        })
    );

    assert_eq!(
        POLAR_BEAR.get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::LittleEndian
            },
            None
        ),
        Ok(GetBytesResult {
            buf_len: POLAR_BEAR_UTF16_LE.len(),
            remaining: None
        })
    );

    assert_eq!(
        POLAR_BEAR.get_bytes(
            ..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::LittleEndian,
                loss_byte: None
            },
            None
        ),
        Ok(GetBytesResult {
            buf_len: POLAR_BEAR_UTF32_LE.len(),
            remaining: None
        })
    );
}

#[test]
fn get_bytes_range_full_buf_small() {
    let mut buf = [0_u8; 5];

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(.., GetBytesEncoding::Utf8, Some(&mut buf))
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF8[..4]);
    assert_eq!(buf[buf_len..], [0; 1]); // verify buffer was not written to
    assert_eq!(remaining, Some(2..5));

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::BigEndian,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF16_BE[..4]);
    assert_eq!(buf[buf_len..], [0; 1]); // verify buffer was not written to
    assert_eq!(remaining, Some(2..5));

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf16 {
                byte_order: GetBytesByteOrder::LittleEndian,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF16_LE[..4]);
    assert_eq!(buf[buf_len..], [0; 1]); // verify buffer was not written to
    assert_eq!(remaining, Some(2..5));

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::BigEndian,
                loss_byte: None,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF32_BE[..4]);
    assert_eq!(buf[buf_len..], [0; 1]); // verify buffer was not written to
    assert_eq!(remaining, Some(2..5));

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR
        .get_bytes(
            ..,
            GetBytesEncoding::Utf32 {
                byte_order: GetBytesByteOrder::LittleEndian,
                loss_byte: None,
            },
            Some(&mut buf),
        )
        .unwrap();
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF32_LE[..4]);
    assert_eq!(buf[buf_len..], [0; 1]); // verify buffer was not written to
    assert_eq!(remaining, Some(2..5));
}

#[test]
fn get_bytes_range_empty() {
    let mut buf = [0_u8; 16];

    assert_eq!(
        EMPTY_STRING.get_bytes(..0, GetBytesEncoding::Utf8, None),
        Ok(EMPTY_RESULT.clone())
    );

    assert_eq!(
        POLAR_BEAR.get_bytes(..0, GetBytesEncoding::Utf8, None),
        Ok(GetBytesResult {
            buf_len: 0,
            remaining: None
        })
    );

    assert_eq!(
        EMPTY_STRING.get_bytes(..0, GetBytesEncoding::Utf8, Some(&mut buf)),
        Ok(EMPTY_RESULT.clone())
    );
    assert_eq!(buf, [0; 16]);

    assert_eq!(
        POLAR_BEAR.get_bytes(..0, GetBytesEncoding::Utf8, Some(&mut buf)),
        Ok(GetBytesResult {
            buf_len: 0,
            remaining: None
        })
    );
    assert_eq!(buf, [0; 16]);
}

#[test]
fn invalid_utf16() {
    let polar_bear_with_ascii_code_units: [u16; 7] = [
        b'['.into(),
        0xd83d,
        0xdc3b,
        0x200d,
        0x2744,
        0xfe0f,
        b']'.into(),
    ];

    // valid code points, then low surrogate (no high), then valid code points
    let mut v = polar_bear_with_ascii_code_units.to_vec();
    let _ = v.remove(2); // U+DC3B
    let s = String::from_utf16(v, FromUtfByteOrder::HostNative);
    assert_eq!(
        s.get_bytes(.., GetBytesEncoding::Utf8, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Surrogate {
                reason: GetBytesSurrogateError::Unpaired,
                index: 1
            },
            result: GetBytesResult {
                buf_len: 1,
                remaining: Some(2..6)
            }
        })
    );

    // valid code points, then low surrogate (no high) at end
    let v = polar_bear_with_ascii_code_units.to_vec(); // U+DC3B..
    let s = String::from_utf16(v.split_at(2).0, FromUtfByteOrder::HostNative);
    assert_eq!(
        s.get_bytes(.., GetBytesEncoding::Utf8, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Surrogate {
                reason: GetBytesSurrogateError::Unpaired,
                index: 1
            },
            result: GetBytesResult {
                buf_len: 1,
                remaining: None
            }
        })
    );

    // valid code points, then high surrogate (no low), then valid code points
    let mut v = polar_bear_with_ascii_code_units.to_vec();
    let _ = v.remove(1); // U+0xD83D
    let s = String::from_utf16(v, FromUtfByteOrder::HostNative);
    assert_eq!(
        s.get_bytes(.., GetBytesEncoding::Utf8, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Surrogate {
                reason: GetBytesSurrogateError::Unpaired,
                index: 1
            },
            result: GetBytesResult {
                buf_len: 1,
                remaining: Some(2..6)
            }
        })
    );

    // then high surrogate (no low), then valid code points
    let v = polar_bear_with_ascii_code_units.to_vec(); // ..U+0xD83D
    let s = String::from_utf16(v.split_at(2).1, FromUtfByteOrder::HostNative);
    assert_eq!(
        s.get_bytes(.., GetBytesEncoding::Utf8, None),
        Err(GetBytesError {
            kind: GetBytesErrorKind::Surrogate {
                reason: GetBytesSurrogateError::Unpaired,
                index: 0
            },
            result: GetBytesResult {
                buf_len: 0,
                remaining: Some(1..5)
            }
        })
    );
}

#[test]
fn get_bytes_unchecked_range_full_buf() {
    let mut buf = [0_u8; 32];

    let GetBytesResult { buf_len, remaining } =
        POLAR_BEAR.get_bytes_unchecked(.., GetBytesEncoding::Utf8, Some(&mut buf));
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF8);
    assert_eq!(buf[buf_len..], [0; 19]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR.get_bytes_unchecked(
        ..,
        GetBytesEncoding::Utf16 {
            byte_order: GetBytesByteOrder::LittleEndian,
        },
        Some(&mut buf),
    );
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF16_LE);
    assert_eq!(buf[buf_len..], [0; 22]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);

    let GetBytesResult { buf_len, remaining } = POLAR_BEAR.get_bytes_unchecked(
        ..,
        GetBytesEncoding::Utf32 {
            byte_order: GetBytesByteOrder::LittleEndian,
            loss_byte: None,
        },
        Some(&mut buf),
    );
    assert_eq!(buf[..buf_len], POLAR_BEAR_UTF32_LE);
    assert_eq!(buf[buf_len..], [0; 16]); // verify buffer was not written to
    assert!(remaining.is_none());
    buf.fill(0);
}

#[should_panic(expected = "invalid range: end index 16 exceeds the container length of 5")]
#[test]
fn get_bytes_out_of_bounds() {
    let mut buf = [0_u8; 16];

    drop(POLAR_BEAR.get_bytes_unchecked(0..16, GetBytesEncoding::Utf8, Some(&mut buf)));
}

#[test]
fn get_bytes_unaligned() {
    const BYTE_ORDERS: [GetBytesByteOrder; 4] = [
        GetBytesByteOrder::BigEndian,
        GetBytesByteOrder::HostNative { include_bom: false },
        GetBytesByteOrder::HostNative { include_bom: true },
        GetBytesByteOrder::LittleEndian,
    ];

    const UTF16_BYTES: [&[u8]; 4] = [
        &POLAR_BEAR_UTF16_BE.0,
        &POLAR_BEAR_UTF16_NE.0,
        &POLAR_BEAR_UTF16_NE_BOM.0,
        &POLAR_BEAR_UTF16_LE.0,
    ];

    const UTF32_BYTES: [&[u8]; 4] = [
        &POLAR_BEAR_UTF32_BE.0,
        &POLAR_BEAR_UTF32_NE.0,
        &POLAR_BEAR_UTF32_NE_BOM.0,
        &POLAR_BEAR_UTF32_LE.0,
    ];

    let mut buf = [0xcc_u8; 32];

    let i = first_unaligned_offset::<u16>(&buf);
    assert_ne!(i, 0);
    for (byte_order, bytes) in BYTE_ORDERS.iter().zip(UTF16_BYTES) {
        let GetBytesResult { buf_len, remaining } = POLAR_BEAR
            .get_bytes(
                ..,
                GetBytesEncoding::Utf16 {
                    byte_order: *byte_order,
                },
                Some(&mut buf[i..]),
            )
            .unwrap();
        assert_eq!(buf[..i], [0xcc_u8; 1]);
        assert_eq!(&buf[i..i + buf_len], bytes);
        assert!(&buf[i + buf_len..].iter().all(|b| *b == 0xcc));
        assert!(remaining.is_none());
        buf.fill(0xcc);
    }

    let i = first_unaligned_offset::<u32>(&buf);
    assert_ne!(i, 0);
    for (byte_order, bytes) in BYTE_ORDERS.iter().zip(UTF32_BYTES) {
        let GetBytesResult { buf_len, remaining } = POLAR_BEAR
            .get_bytes(
                ..,
                GetBytesEncoding::Utf32 {
                    byte_order: *byte_order,
                    loss_byte: None,
                },
                Some(&mut buf[i..]),
            )
            .unwrap();
        assert_eq!(buf[..i], [0xcc_u8; 1]);
        assert_eq!(&buf[i..i + buf_len], bytes);
        assert!(&buf[i + buf_len..].iter().all(|b| *b == 0xcc));
        assert!(remaining.is_none());
        buf.fill(0xcc);
    }
}

fn first_unaligned_offset<T>(v: &[u8]) -> usize {
    let mut i = 0;

    while i < v.len() {
        if !is_aligned(v[i..].as_ptr().cast::<T>()) {
            break;
        }
        // UB: Cannot overflow because it will never exceed `v.len()`.
        i = i.wrapping_add(1);
    }

    assert_ne!(i, v.len(), "unable to find an unaligned offset in v for T");
    i
}

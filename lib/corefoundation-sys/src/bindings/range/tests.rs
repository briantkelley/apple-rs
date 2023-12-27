#![allow(clippy::reversed_empty_ranges, clippy::undocumented_unsafe_blocks)]

use super::*;

const FIRST_INVALID_INDEX: usize = 1_usize << (usize::BITS - 1);

#[test]
fn expect_from() {
    assert_eq!(
        CFRange::expect_from_range_bounds(5.., 10),
        CFRange {
            location: 5,
            length: 5
        }
    );
}

#[should_panic(expected = "invalid range: cannot end after usize::MAX")]
#[test]
fn expect_from_range_bounds_exclusive_overflow_panic() {
    let _ = CFRange::expect_from_range_bounds(0x8000_0000..=usize::MAX, usize::MAX);
}

#[should_panic(expected = "invalid range: start index 5 exceeds the container length of 0")]
#[test]
fn expect_from_range_bounds_out_of_bounds_panic() {
    let _ = CFRange::expect_from_range_bounds(5..10, 0);
}

#[should_panic(expected = "invalid range: end index 9223372036854775808 exceeds CFIndex::MAX")]
#[test]
fn expect_from_range_bounds_signed_overflow_panic() {
    let _ = CFRange::expect_from_range_bounds(100..FIRST_INVALID_INDEX, usize::MAX);
}

#[test]
fn is_empty() {
    assert!(CFRange {
        location: 10,
        length: 0
    }
    .is_empty());

    assert!(!CFRange {
        location: 0,
        length: 1
    }
    .is_empty());
}

#[test]
fn try_from_cf_range_not_found() {
    assert_eq!(
        Option::<Range<usize>>::try_from(CFRange {
            location: kCFNotFound,
            length: 0,
        }),
        Ok(None)
    );

    assert_eq!(
        Option::<Range<usize>>::try_from(CFRange {
            location: 5,
            length: 0,
        }),
        Ok(Some(5..5))
    );

    assert_eq!(
        Option::<Range<usize>>::try_from(CFRange {
            location: -5,
            length: 0,
        }),
        Err(TryFromCFRangeError(
            TryFromCFRangeErrorKind::NegativeLocation
        ))
    );
}

#[test]
fn try_from_cf_range_ok() {
    assert_eq!(
        Range::<usize>::try_from(CFRange {
            location: 0,
            length: 0
        }),
        Ok(0..0)
    );

    assert_eq!(
        Range::<usize>::try_from(CFRange {
            location: 10,
            length: 0
        }),
        Ok(10..10)
    );

    assert_eq!(
        Range::<usize>::try_from(CFRange {
            location: 10,
            length: 5
        }),
        Ok(10..15)
    );
}

#[should_panic(expected = "invalid range: negative length")]
#[test]
fn try_from_cf_range_negative_length_panic() {
    let _ = Range::<usize>::try_from(CFRange {
        location: 10,
        length: -5,
    })
    .expect("invalid range");
}

#[should_panic(expected = "invalid range: negative location")]
#[test]
fn try_from_cf_range_negative_location_panic() {
    let _ = Range::<usize>::try_from(CFRange {
        location: -5,
        length: 10,
    })
    .expect("invalid range");
}

#[should_panic(expected = "invalid range: location + length exceeds CFIndex::MAX")]
#[test]
fn try_from_cf_range_too_long_panic() {
    let _ = Range::<usize>::try_from(CFRange {
        location: 1,
        length: CFIndex::MAX,
    })
    .expect("invalid range");
}

#[test]
fn try_from_range_bounds_ok() {
    assert_eq!(
        CFRange::try_from_range_bounds(0..0, 0),
        Ok(CFRange {
            location: 0,
            length: 0
        })
    );

    assert_eq!(
        CFRange::try_from_range_bounds(..10, 10),
        Ok(CFRange {
            location: 0,
            length: 10
        })
    );

    assert_eq!(
        CFRange::try_from_range_bounds(10.., 20),
        Ok(CFRange {
            location: 10,
            length: 10
        })
    );

    assert_eq!(
        CFRange::try_from_range_bounds(.., 20),
        Ok(CFRange {
            location: 0,
            length: 20
        })
    );

    assert_eq!(
        CFRange::try_from_range_bounds(10..5, 11),
        Ok(CFRange {
            location: 10,
            length: 0
        })
    );
}

#[test]
fn try_from_range_bounds_exclusive_overflow() {
    struct R(usize);

    impl RangeBounds<usize> for R {
        fn start_bound(&self) -> Bound<&usize> {
            Bound::Excluded(&self.0)
        }

        fn end_bound(&self) -> Bound<&usize> {
            Bound::Included(&self.0)
        }
    }

    assert_eq!(
        CFRange::try_from_range_bounds(R(usize::MAX), usize::MAX),
        Err(TryFromRangeError(TryFromRangeErrorKind::ExclusiveOverflow(
            TryFromRangeBound::Start
        )))
    );

    assert_eq!(
        CFRange::try_from_range_bounds(..=usize::MAX, usize::MAX),
        Err(TryFromRangeError(TryFromRangeErrorKind::ExclusiveOverflow(
            TryFromRangeBound::End
        )))
    );
}

#[test]
fn try_from_range_bounds_out_of_bounds() {
    assert_eq!(
        CFRange::try_from_range_bounds(10..5, 10),
        Err(TryFromRangeError(TryFromRangeErrorKind::OutOfBounds {
            bound: TryFromRangeBound::Start,
            value: 10,
            len: 10
        }))
    );

    assert_eq!(
        CFRange::try_from_range_bounds(10..15, 5),
        Err(TryFromRangeError(TryFromRangeErrorKind::OutOfBounds {
            bound: TryFromRangeBound::Start,
            value: 10,
            len: 5
        }))
    );

    assert_eq!(
        CFRange::try_from_range_bounds(..=10, 10),
        Err(TryFromRangeError(TryFromRangeErrorKind::OutOfBounds {
            bound: TryFromRangeBound::End,
            value: 11,
            len: 10
        }))
    );

    assert_eq!(
        CFRange::try_from_range_bounds(10..15, 14),
        Err(TryFromRangeError(TryFromRangeErrorKind::OutOfBounds {
            bound: TryFromRangeBound::End,
            value: 15,
            len: 14
        }))
    );
}

// LINT: Tests a different code path than the canonical spelling.
#[allow(clippy::range_minus_one)]
#[test]
fn try_from_range_bounds_signed_overflow() {
    assert_eq!(
        CFRange::try_from_range_bounds(FIRST_INVALID_INDEX.., usize::MAX),
        Err(TryFromRangeError(TryFromRangeErrorKind::SignedOverflow {
            bound: TryFromRangeBound::Start,
            value: FIRST_INVALID_INDEX
        }))
    );

    assert_eq!(
        CFRange::try_from_range_bounds(..=(FIRST_INVALID_INDEX - 1), usize::MAX),
        Err(TryFromRangeError(TryFromRangeErrorKind::SignedOverflow {
            bound: TryFromRangeBound::End,
            value: FIRST_INVALID_INDEX
        }))
    );

    assert_eq!(
        CFRange::try_from_range_bounds(..FIRST_INVALID_INDEX, usize::MAX),
        Err(TryFromRangeError(TryFromRangeErrorKind::SignedOverflow {
            bound: TryFromRangeBound::End,
            value: FIRST_INVALID_INDEX
        }))
    );
}

#[test]
fn try_from_range_ok() {
    assert_eq!(
        CFRange::try_from(0..0),
        Ok(CFRange {
            location: 0,
            length: 0
        })
    );

    assert_eq!(
        CFRange::try_from(0..10),
        Ok(CFRange {
            location: 0,
            length: 10
        })
    );

    assert_eq!(
        CFRange::try_from(10..20),
        Ok(CFRange {
            location: 10,
            length: 10
        })
    );

    assert_eq!(
        CFRange::try_from(10..5),
        Ok(CFRange {
            location: 10,
            length: 0
        })
    );
}

#[test]
fn try_from_range_signed_overflow() {
    const INFLECTION_POINT: usize = 1_usize << (usize::BITS - 1);

    assert_eq!(
        CFRange::try_from(INFLECTION_POINT..usize::MAX),
        Err(TryFromRangeError(TryFromRangeErrorKind::SignedOverflow {
            bound: TryFromRangeBound::Start,
            value: INFLECTION_POINT
        }))
    );

    assert_eq!(
        CFRange::try_from(0..INFLECTION_POINT),
        Err(TryFromRangeError(TryFromRangeErrorKind::SignedOverflow {
            bound: TryFromRangeBound::End,
            value: INFLECTION_POINT
        }))
    );
}

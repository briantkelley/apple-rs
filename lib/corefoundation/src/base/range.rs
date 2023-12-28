use crate::{ExpectFrom, FromUnchecked};
use core::ops::Range;
use corefoundation_sys::{CFIndex, CFRange};

impl ExpectFrom<Range<usize>> for CFRange {
    #[inline]
    fn expect_from(value: Range<usize>) -> Self {
        Self::try_from(value).expect("invalid range")
    }
}

impl FromUnchecked<Range<usize>> for CFRange {
    /// Converts a [`Range<usize>`] into a [`CFRange`].
    ///
    /// # Safety
    ///
    /// Both the `start` and `end` fields of `value` must be less than or equal to [`isize::MAX`].
    #[inline]
    fn from_unchecked(value: Range<usize>) -> Self {
        let start = CFIndex::from_unchecked(value.start);
        let end = CFIndex::from_unchecked(value.end);

        // UB: Caller assumes responsibility for the correctness of this operation.
        let length = end.wrapping_sub(start);

        Self {
            location: start,
            length,
        }
    }
}

impl ExpectFrom<CFRange> for Range<usize> {
    #[inline]
    fn expect_from(value: CFRange) -> Self {
        Self::try_from(value).expect("invalid range")
    }
}

impl FromUnchecked<CFRange> for Range<usize> {
    /// Converts a [`CFRange`] into a [`Range<usize>`].
    ///
    /// # Safety
    ///
    /// Both the `location` and `length` fields of `value` must be non-negative, and their sum must
    /// be less than or equal to [`isize::MAX`].
    #[inline]
    fn from_unchecked(value: CFRange) -> Self {
        let location = usize::from_unchecked(value.location);
        let length = usize::from_unchecked(value.length);

        // UB: Caller assumes responsibility for the correctness of this operation.
        let end = location.wrapping_add(length);

        Self {
            start: location,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_from() {
        assert_eq!(
            CFRange::expect_from(5..10),
            CFRange {
                location: 5,
                length: 5
            }
        );

        assert_eq!(
            Range::<usize>::expect_from(CFRange {
                location: 5,
                length: 10
            }),
            5..15
        );
    }

    #[should_panic(expected = "invalid range: negative length")]
    #[test]
    fn expect_from_cf_range_negative_length_panic() {
        let _ = Range::<usize>::expect_from(CFRange {
            location: 10,
            length: -5,
        });
    }

    #[should_panic(expected = "invalid range: negative location")]
    #[test]
    fn expect_from_cf_range_negative_location_panic() {
        let _ = Range::<usize>::expect_from(CFRange {
            location: -5,
            length: 10,
        });
    }

    #[should_panic(expected = "invalid range: location + length exceeds CFIndex::MAX")]
    #[test]
    fn expect_from_cf_range_too_long_panic() {
        let _ = Range::<usize>::expect_from(CFRange {
            location: 1,
            length: CFIndex::MAX,
        });
    }

    #[test]
    fn from_unchecked() {
        const FIRST_INVALID_INDEX: usize = 1_usize << (usize::BITS - 1);

        assert_eq!(
            CFRange::from_unchecked(250..1000),
            CFRange {
                location: 250,
                length: 750
            }
        );

        assert_eq!(
            CFRange::from_unchecked(FIRST_INVALID_INDEX..usize::MAX),
            CFRange {
                location: CFIndex::MIN,
                length: CFIndex::MAX
            }
        );

        assert_eq!(
            Range::<usize>::from_unchecked(CFRange {
                location: 500,
                length: 3000,
            }),
            500..3500
        );

        assert_eq!(
            Range::<usize>::from_unchecked(CFRange {
                location: -1,
                length: CFIndex::MIN,
            }),
            Range {
                start: usize::MAX,
                end: FIRST_INVALID_INDEX - 1
            }
        );
    }
}

use crate::{kCFNotFound, CFIndex, CFRange};
use core::fmt::{self, Debug, Display, Formatter};
use core::ops::{Bound, Range, RangeBounds};

#[cfg(test)]
mod tests;

/// The error type returned when creating a [`Range<usize>`] from a [`CFRange`] fails.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TryFromCFRangeError(TryFromCFRangeErrorKind);

#[derive(Clone, Copy, Eq, PartialEq)]
enum TryFromCFRangeErrorKind {
    NegativeLength,
    NegativeLocation,
    TooLong,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum TryFromRangeBound {
    Start,
    End,
}

/// The error type returned when creating a [`CFRange`] from a [`RangeBounds<usize>`] fails.
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TryFromRangeError(TryFromRangeErrorKind);

#[derive(Clone, Copy, Eq, PartialEq)]
enum TryFromRangeErrorKind {
    ExclusiveOverflow(TryFromRangeBound),
    OutOfBounds {
        bound: TryFromRangeBound,
        value: usize,
        len: usize,
    },
    SignedOverflow {
        bound: TryFromRangeBound,
        value: usize,
    },
}

impl CFRange {
    /// Performs conversion and bounds-checking of a `range`.
    ///
    /// # Panics
    ///
    /// Panics if `range`'s end is greater than `to`, if a range bound cannot be represented by
    /// [`CFIndex`], or if an exclusive bound overflows [`usize`].
    #[inline]
    pub fn expect_from_range_bounds(range: impl RangeBounds<usize>, to: usize) -> Self {
        Self::try_from_range_bounds(range, to).expect("invalid range")
    }

    /// Performs conversion and bounds-checking of a `range`.
    ///
    /// # Errors
    ///
    /// Returns a [`TryFromRangeError`] if `range`'s end is greater than `to`, if a range bound
    /// cannot be represented by [`CFIndex`], or if an exclusive bound overflows [`usize`].
    #[inline]
    pub fn try_from_range_bounds(
        range: impl RangeBounds<usize>,
        to: usize,
    ) -> Result<Self, TryFromRangeError> {
        fn inner(start: usize, end: usize, to: usize) -> Result<CFRange, TryFromRangeError> {
            let range = CFRange::try_from(Range { start, end })?;
            // [`CFRange::try_from`] may have changed `end` if it was greater than `start`.
            // UB: This cannot overflow because it was validated by [`CFRange::try_from`].
            let end = range.location.wrapping_add(range.length);
            // LINT: No risk of sign change as [`CFRange::try_from`] verified this is non-negative.
            #[allow(clippy::as_conversions, clippy::cast_sign_loss)]
            let end = end as usize;

            // Handle `0..0` as a special case where `start == to` is valid.
            if start > to || (start == to && to != 0) {
                Err(TryFromRangeErrorKind::OutOfBounds {
                    bound: TryFromRangeBound::Start,
                    value: start,
                    len: to,
                }
                .into())
            } else if end > to {
                Err(TryFromRangeErrorKind::OutOfBounds {
                    bound: TryFromRangeBound::End,
                    value: end,
                    len: to,
                }
                .into())
            } else {
                Ok(range)
            }
        }

        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => {
                start
                    .checked_add(1)
                    .ok_or(TryFromRangeErrorKind::ExclusiveOverflow(
                        TryFromRangeBound::Start,
                    ))?
            }
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&end) => {
                end.checked_add(1)
                    .ok_or(TryFromRangeErrorKind::ExclusiveOverflow(
                        TryFromRangeBound::End,
                    ))?
            }
            Bound::Excluded(&end) => end,
            Bound::Unbounded => to,
        };

        inner(start, end, to)
    }

    /// Returns `true` if the range contains no items.
    #[inline]
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl TryFrom<Range<usize>> for CFRange {
    type Error = TryFromRangeError;

    #[inline]
    fn try_from(value: Range<usize>) -> Result<Self, Self::Error> {
        // `end - start` will always return [`Some`] unless `start > end`, in which case the range
        // is defined to be empty.
        let length = value.end.checked_sub(value.start).unwrap_or_default();

        let start =
            CFIndex::try_from(value.start).map_err(|_| TryFromRangeErrorKind::SignedOverflow {
                bound: TryFromRangeBound::Start,
                value: value.start,
            })?;

        // Verify `end` does not exceed [`CFIndex::MAX`].
        let _ =
            CFIndex::try_from(value.end).map_err(|_| TryFromRangeErrorKind::SignedOverflow {
                bound: TryFromRangeBound::End,
                value: value.end,
            })?;

        // LINT: If both `start` and `end` can be represented by [`CFIndex`], then by definition,
        // `length` can be too.
        #[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
        let length = length as CFIndex;

        Ok(Self {
            location: start,
            length,
        })
    }
}

impl TryFrom<CFRange> for Option<Range<usize>> {
    type Error = TryFromCFRangeError;

    #[inline]
    fn try_from(value: CFRange) -> Result<Self, Self::Error> {
        if value.location == kCFNotFound {
            Ok(None)
        } else {
            Range::<usize>::try_from(value).map(Some)
        }
    }
}

impl TryFrom<CFRange> for Range<usize> {
    type Error = TryFromCFRangeError;

    #[inline]
    fn try_from(value: CFRange) -> Result<Self, Self::Error> {
        let start = usize::try_from(value.location)
            .map_err(|_| TryFromCFRangeErrorKind::NegativeLocation)?;
        let _ =
            usize::try_from(value.length).map_err(|_| TryFromCFRangeErrorKind::NegativeLength)?;

        let end = value
            .location
            .checked_add(value.length)
            .ok_or(TryFromCFRangeErrorKind::TooLong)?;

        // SAFETY: If `location` and `length` are non-negative, and `end` can be represented by
        // [`CFIndex`], then by definition, it can be represented by [`usize`] too.
        #[allow(clippy::as_conversions, clippy::cast_sign_loss)]
        let end = end as usize;

        Ok(Self { start, end })
    }
}

impl Debug for TryFromCFRangeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for TryFromCFRangeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            TryFromCFRangeErrorKind::NegativeLength => write!(f, "negative length"),
            TryFromCFRangeErrorKind::NegativeLocation => write!(f, "negative location"),
            TryFromCFRangeErrorKind::TooLong => {
                write!(f, "location + length exceeds CFIndex::MAX")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromCFRangeError {}

impl From<TryFromCFRangeErrorKind> for TryFromCFRangeError {
    #[inline]
    fn from(value: TryFromCFRangeErrorKind) -> Self {
        Self(value)
    }
}

impl TryFromRangeBound {
    const fn location_name(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}

impl Debug for TryFromRangeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl Display for TryFromRangeError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            TryFromRangeErrorKind::ExclusiveOverflow(bound) => {
                let verb = bound.location_name();
                write!(f, "cannot {verb} after usize::MAX")
            }
            TryFromRangeErrorKind::OutOfBounds { bound, value, len } => {
                let location = bound.location_name();
                write!(
                    f,
                    "{location} index {value} exceeds the container length of {len}"
                )
            }
            TryFromRangeErrorKind::SignedOverflow { bound, value } => {
                let location = bound.location_name();
                write!(f, "{location} index {value} exceeds CFIndex::MAX")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TryFromRangeError {}

impl From<TryFromRangeErrorKind> for TryFromRangeError {
    #[inline]
    fn from(value: TryFromRangeErrorKind) -> Self {
        Self(value)
    }
}

use crate::{ExpectFrom, FromUnchecked};
use corefoundation_sys::CFIndex;

impl ExpectFrom<usize> for CFIndex {
    #[inline]
    fn expect_from(value: usize) -> Self {
        Self::try_from(value).expect("value is greater than CFIndex::MAX")
    }
}

impl FromUnchecked<usize> for CFIndex {
    /// Converts `value` into a [`CFIndex`].
    ///
    /// # Safety
    ///
    /// `value` must be less than or equal to [`isize::MAX`].
    // LINT: Caller assumes responsibility for the correctness of this operation.
    #[allow(clippy::as_conversions, clippy::cast_possible_wrap)]
    #[inline]
    fn from_unchecked(value: usize) -> Self {
        value as _
    }
}

impl ExpectFrom<CFIndex> for usize {
    #[inline]
    fn expect_from(value: CFIndex) -> Self {
        Self::try_from(value).unwrap_or_else(|_| panic!("value is negative"))
    }
}

impl FromUnchecked<CFIndex> for usize {
    /// Converts `value` into a [`usize`].
    ///
    /// # Safety
    ///
    /// `value` must be non-negative.
    // LINT: Caller assumes responsibility for the correctness of this operation.
    #[allow(clippy::as_conversions, clippy::cast_sign_loss)]
    #[inline]
    fn from_unchecked(value: isize) -> Self {
        value as _
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_from() {
        assert_eq!(CFIndex::expect_from(100), 100);
        assert_eq!(usize::expect_from(100), 100);
    }

    #[should_panic(expected = "value is negative")]
    #[test]
    fn expect_from_cf_index_panic() {
        let _ = usize::expect_from(-1);
    }

    #[should_panic(expected = "value is greater than CFIndex::MAX")]
    #[test]
    fn expect_from_usize_panic() {
        let _ = CFIndex::expect_from(usize::MAX);
    }

    #[test]
    fn from_unchecked() {
        assert_eq!(CFIndex::from_unchecked(100), 100);
        assert_eq!(CFIndex::from_unchecked(usize::MAX), -1);

        assert_eq!(usize::from_unchecked(100), 100);
        assert_eq!(usize::from_unchecked(-1), usize::MAX);
    }
}

/// An ICU version consists of up to 4 numbers from `0..=255`.
///
/// Stable since ICU 2.4
pub const U_MAX_VERSION_LENGTH: usize = 4;

/// The binary form of a version on ICU APIs is an array of 4 [`u8`].
///
/// To compare two versions, use `memcmp(v1, v2, sizeof(UVersionInfo))`.
///
/// Stable since ICU 2.4
pub type UVersionInfo = [u8; U_MAX_VERSION_LENGTH];

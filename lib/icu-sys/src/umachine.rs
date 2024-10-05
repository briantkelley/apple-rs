/// The ICU boolean type, a signed-byte integer.
///
/// ICU-specific for historical reasons: The C and C++ standards used to not define type `bool`.
/// Also provides a fixed type definition, as opposed to type bool whose details (e.g., `sizeof`)
/// may vary by compiler and between C and C++.
///
/// Stable since ICU 2.0
pub type UBool = i8;

/// Define `UChar32` as a type for single Unicode code points. `UChar32` is a signed 32-bit integer
/// (same as [`i32`]).
///
/// The Unicode code point range is `0..=0x10ffff`. All other values (negative or `>=0x110000`) are
/// illegal as Unicode code points. They may be used as sentinel values to indicate "done", "error"
/// or similar non-code point conditions.
///
/// Before ICU 2.4 (Jitterbug 2146), `UChar32` was defined to be `wchar_t` if that is 32 bits wide
/// (`wchar_t` may be signed or unsigned) or else to be [`u32`]. That is, the definition of
/// `UChar32` was platform-dependent.
///
/// Stable since ICU 2.4
pub type UChar32 = i32;

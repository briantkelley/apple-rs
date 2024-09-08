//! Functions to get configurable system variables.
//!
//! Provides a well-defined interface for the application to determine the current value of a
//! configurable system limit or option (*variable*).

use core::num::NonZeroUsize;
use darwin_sys::{sysconf, _SC_PAGESIZE};

/// Returns the size, in bytes, of a virtual memory page.
///
/// The minimum acceptable value is `1`[^limits]. The `PAGESIZE` variable is required to be
/// returned by [`sysconf`][^sysconf].
///
/// # Panics
///
/// Panics if the underlying [`sysconf`] implementation does not return a positive integer.
///
/// [^limits]: <https://pubs.opengroup.org/onlinepubs/9799919799/basedefs/limits.h.html>
/// [^sysconf]: <https://pubs.opengroup.org/onlinepubs/9799919799/functions/sysconf.html>
#[inline]
pub fn page_size() -> usize {
    // SAFETY: [`sysconf`] does not have any safety requirements to consider.
    let value = unsafe { sysconf(_SC_PAGESIZE) };

    usize::try_from(value)
        .ok()
        .and_then(NonZeroUsize::new)
        .expect("non-conformant PAGESIZE value")
        .get()
}

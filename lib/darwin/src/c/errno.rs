use crate::_sys::c::errno::{self, __error};
use core::num::NonZeroI32;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Error {
    IO = errno::EIO,
    OutOfMemory = errno::ENOMEM,
    InvalidArgument = errno::EINVAL,
}

#[must_use]
pub fn get() -> Option<NonZeroI32> {
    // SAFETY: __error() is guaranteed to return a thread-local, non-null pointer.
    NonZeroI32::new(unsafe { *__error() })
}

/// Set the last error number visible to the current thread.
pub fn set(errno: Option<NonZeroI32>) {
    let errno = errno.map_or(0, NonZeroI32::get);
    // SAFETY: __error() is guaranteed to return a thread-specific non-null pointer.
    unsafe { *__error() = errno };
}

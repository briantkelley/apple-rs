use core::ffi::CStr;
use core::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct LogString {
    pub ptr: *const i8,
}

#[allow(missing_debug_implementations)]
#[doc(hidden)]
#[repr(C)]
pub struct _LogStringImpl<S> {
    pub _str: S,
    pub _nul: u8,
}

// SAFETY: `LogString` instances are immutable and they only contain pointers to read-only segments
// of the executable and are therefore safe to send and share across threads.
unsafe impl Send for LogString {}
unsafe impl Sync for LogString {}

impl Debug for LogString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // SAFETY: LogString is only constructed by the `log_string!` macro, which guarantees `ptr`
        // is a valid nul-terminated C-style string.
        unsafe { CStr::from_ptr(self.ptr) }.fmt(f)
    }
}

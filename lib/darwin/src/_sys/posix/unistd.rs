use core::ffi::{c_char, c_int};

pub(crate) const _CS_DARWIN_USER_TEMP_DIR: c_int = 65537;

extern "C" {
    pub(crate) fn close(fildes: c_int) -> c_int;
    pub(crate) fn unlink(path: *const c_char) -> c_int;
    pub(crate) fn confstr(name: c_int, buf: *mut c_char, len: usize) -> usize;
    pub(crate) fn mkstemp(template: *mut c_char) -> c_int;
}

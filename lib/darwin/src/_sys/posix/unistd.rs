use core::ffi::{c_char, c_int};

extern "C" {
    pub(crate) fn close(fildes: c_int) -> c_int;
    pub(crate) fn confstr(name: c_int, buf: *mut c_char, len: usize) -> usize;
}

pub(crate) const _CS_DARWIN_USER_TEMP_DIR: c_int = 65537;

use core::ffi::{c_char, c_int};

pub(crate) const O_RDONLY: c_int = 0x0000;
pub(crate) const O_WRONLY: c_int = 0x0001;
pub(crate) const O_RDWR: c_int = 0x0002;
pub(crate) const O_ACCMODE: c_int = 0x0003;

pub(crate) const O_CLOEXEC: c_int = 0x0100_0000;

extern "C" {
    pub(crate) fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
}

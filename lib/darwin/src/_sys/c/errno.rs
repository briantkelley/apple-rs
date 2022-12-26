use core::ffi::c_int;

pub(crate) const EIO: c_int = 5;
pub(crate) const ENOMEM: c_int = 12;
pub(crate) const EINVAL: c_int = 22;

extern "C" {
    pub(crate) fn __error() -> &'static mut c_int;
}

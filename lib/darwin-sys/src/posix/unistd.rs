use core::ffi::{c_int, c_long};

pub const _SC_PAGESIZE: c_int = 29;

extern "C" {
    pub fn sysconf(name: c_int) -> c_long;
}

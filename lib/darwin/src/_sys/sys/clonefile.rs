use crate::io::BorrowedFd;
use core::ffi::{c_char, c_int};

pub(crate) const CLONE_NOFOLLOW: u32 = 0x0001;
pub(crate) const CLONE_NOOWNERCOPY: u32 = 0x0002;
pub(crate) const CLONE_ACL: u32 = 0x0004;

extern "C" {
    pub(crate) fn fclonefileat(
        srcfd: BorrowedFd<'_>,
        dst_dirfd: BorrowedFd<'_>,
        dst: *const c_char,
        flags: u32,
    ) -> c_int;
}

use crate::_sys::sys::types::{
    blkcnt_t, blksize_t, dev_t, gid_t, mode_t, nlink_t, off_t, timespec, uid_t, S_IRGRP, S_IROTH,
    S_IRUSR, S_IRWXG, S_IRWXO, S_IRWXU, S_ISGID, S_ISUID, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR,
};
use crate::io::BorrowedFd;
use core::ffi::c_int;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct stat {
    pub(crate) dev: dev_t,
    pub(crate) mode: mode_t,
    pub(crate) nlink: nlink_t,
    pub(crate) ino: u64,
    pub(crate) st_uid: uid_t,
    pub(crate) st_gid: gid_t,
    pub(crate) st_rdev: dev_t,
    pub(crate) atimespec: timespec,
    pub(crate) mtimespec: timespec,
    pub(crate) ctimespec: timespec,
    pub(crate) birthtimespec: timespec,
    pub(crate) size: off_t,
    pub(crate) blocks: blkcnt_t,
    pub(crate) blksize: blksize_t,
    pub(crate) flags: u32,
    pub(crate) gen: u32,
    pub(crate) lspare: i32,
    pub(crate) qspare: [i64; 2],
}

pub(crate) const ALLPERMS: mode_t = S_ISUID | S_ISGID | S_ISVTX | S_IRWXU | S_IRWXG | S_IRWXO;
pub(crate) const DEFFILEMODE: mode_t = S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;

extern "C" {
    pub(crate) fn fstat(fildes: BorrowedFd<'_>, buf: &mut stat) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::stat;
    use core::mem::size_of;

    #[test]
    fn size() {
        assert_eq!(size_of::<stat>(), 144);
    }
}

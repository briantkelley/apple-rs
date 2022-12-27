#![allow(non_camel_case_types)]

pub(crate) type blkcnt_t = i64;
pub(crate) type blksize_t = i32;
pub(crate) type dev_t = i32;
pub(crate) type gid_t = u32;
pub(crate) type mode_t = u16;
pub(crate) type nlink_t = u16;
pub(crate) type off_t = i64;

pub(crate) const S_IFMT: mode_t = 0o170_000;
pub(crate) const S_IFIFO: mode_t = 0o010_000;
pub(crate) const S_IFCHR: mode_t = 0o020_000;
pub(crate) const S_IFDIR: mode_t = 0o040_000;
pub(crate) const S_IFBLK: mode_t = 0o060_000;
pub(crate) const S_IFREG: mode_t = 0o100_000;
pub(crate) const S_IFLNK: mode_t = 0o120_000;
pub(crate) const S_IFSOCK: mode_t = 0o140_000;
pub(crate) const S_IRWXU: mode_t = 0o000_700;
pub(crate) const S_IRUSR: mode_t = 0o000_400;
pub(crate) const S_IWUSR: mode_t = 0o000_200;
pub(crate) const S_IXUSR: mode_t = 0o000_100;
pub(crate) const S_IRWXG: mode_t = 0o000_070;
pub(crate) const S_IRGRP: mode_t = 0o000_040;
pub(crate) const S_IWGRP: mode_t = 0o000_020;
pub(crate) const S_IXGRP: mode_t = 0o000_010;
pub(crate) const S_IRWXO: mode_t = 0o000_007;
pub(crate) const S_IROTH: mode_t = 0o000_004;
pub(crate) const S_IWOTH: mode_t = 0o000_002;
pub(crate) const S_IXOTH: mode_t = 0o000_001;
pub(crate) const S_ISUID: mode_t = 0o004_000;
pub(crate) const S_ISGID: mode_t = 0o002_000;
pub(crate) const S_ISVTX: mode_t = 0o001_000;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct timespec {
    pub(crate) sec: isize,
    pub(crate) nsec: isize,
}

pub(crate) type uid_t = u32;

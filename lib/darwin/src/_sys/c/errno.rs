use core::ffi::c_int;

pub(crate) const ENOENT: c_int = 2;
pub(crate) const EINTR: c_int = 4;
pub(crate) const EIO: c_int = 5;
pub(crate) const ENXIO: c_int = 6;
pub(crate) const EBADF: c_int = 9;
pub(crate) const EDEADLK: c_int = 11;
pub(crate) const ENOMEM: c_int = 12;
pub(crate) const EACCES: c_int = 13;
pub(crate) const EFAULT: c_int = 14;
pub(crate) const EEXIST: c_int = 17;
pub(crate) const ENOTDIR: c_int = 20;
pub(crate) const EISDIR: c_int = 21;
pub(crate) const EINVAL: c_int = 22;
pub(crate) const ENFILE: c_int = 23;
pub(crate) const EMFILE: c_int = 24;
pub(crate) const ETXTBSY: c_int = 26;
pub(crate) const ENOSPC: c_int = 28;
pub(crate) const EROFS: c_int = 30;
pub(crate) const EAGAIN: c_int = 35;
pub(crate) const ELOOP: c_int = 62;
pub(crate) const ENAMETOOLONG: c_int = 63;
pub(crate) const EDQUOT: c_int = 69;
pub(crate) const EOVERFLOW: c_int = 84;
pub(crate) const EILSEQ: c_int = 92;
pub(crate) const EOPNOTSUPP: c_int = 102;

extern "C" {
    pub(crate) fn __error() -> &'static mut c_int;
}

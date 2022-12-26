use crate::_sys::posix::unistd::close;
use core::ffi::c_int;

/// An interface to construct an owner type for a raw file descriptor.
pub trait FromRawFd {
    /// Accepts ownership of the file descriptor and will close it when dropped.
    ///
    /// # Safety
    ///
    /// The file descriptor `fd` must be open, must be the unique owner of the resource, and must
    /// not require any clean up other than `close(2)`.
    unsafe fn from_raw_fd(fd: c_int) -> Self;
}

/// An owned file descriptor.
///
/// This closes the file descriptor on drop.
#[repr(transparent)]
#[derive(Debug)]
pub struct OwnedFd {
    fd: c_int,
}

impl Drop for OwnedFd {
    fn drop(&mut self) {
        // It is not possible to recover from `close(2)` errors as the close may have actually
        // succeeded. Retrying may close an unowned file descriptor acquired by another thread in
        // the process.

        // SAFETY: OwnedFd is the unique owner of the resource identified by `fd` and therefore it
        // is safe to release the resource.
        let _ = unsafe { close(self.fd) };
    }
}

impl FromRawFd for OwnedFd {
    unsafe fn from_raw_fd(fd: c_int) -> Self {
        Self { fd }
    }
}

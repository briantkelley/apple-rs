use crate::_sys::posix::fcntl::{open, O_ACCMODE, O_CLOEXEC, O_RDONLY, O_RDWR, O_WRONLY};
use crate::c::errno::check_retry;
use crate::io::{FromRawFd, OwnedFd};
use core::ffi::CStr;
use core::num::NonZeroI32;

/// Specifies the type of I/O access granted to the file.
#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum AccessMode {
    ReadOnly = O_RDONLY,
    WriteOnly = O_WRONLY,
    ReadWrite = O_RDWR,
}

/// Settings for opening, creating, and accessing a file.
#[allow(missing_copy_implementations)]
#[derive(Debug, Default)]
pub struct Open {
    oflag: i32,
}

impl Open {
    #[must_use]
    pub const fn new(access_mode: AccessMode) -> Self {
        Self {
            oflag: access_mode as _,
        }
    }

    #[must_use]
    pub const fn access_mode(mut self, access_mode: AccessMode) -> Self {
        self.oflag &= !O_ACCMODE;
        self.oflag |= access_mode as i32;
        self
    }

    #[must_use]
    pub const fn close_on_exec(self, close_on_exec: bool) -> Self {
        self.set_flag_enabled(O_CLOEXEC, close_on_exec)
    }

    pub fn path(self, path: impl AsRef<CStr>) -> Result<OwnedFd, NonZeroI32> {
        let path = path.as_ref().as_ptr();
        let oflag = self.oflag;

        // SAFETY: path is guaranteed to be a valid, nul-terminated C-style string and open() will
        // not write to path.
        check_retry(|| unsafe { open(path, oflag) })
            // SAFETY: fd is opened, the unique owner of the resource, and must be `close(2)`ed.
            .map(|fd| unsafe { OwnedFd::from_raw_fd(fd) })
    }

    const fn set_flag_enabled(mut self, flag: i32, enable: bool) -> Self {
        if enable {
            self.oflag |= flag;
        } else {
            self.oflag &= !flag;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{AccessMode, Open};
    use crate::_sys::posix::fcntl::{O_CLOEXEC, O_RDONLY, O_RDWR, O_WRONLY};
    use crate::c::errno::Error;

    #[test]
    fn access_mode() {
        let o = Open::default;
        assert_eq!(o().oflag, O_RDONLY);
        assert_eq!(o().access_mode(AccessMode::WriteOnly).oflag, O_WRONLY);
        assert_eq!(o().access_mode(AccessMode::ReadWrite).oflag, O_RDWR);
        assert_eq!(o().access_mode(AccessMode::ReadOnly).oflag, O_RDONLY);
    }

    #[test]
    fn flags() {
        let o = Open::default;

        assert_eq!(o().close_on_exec(true).oflag, O_CLOEXEC);
        assert_eq!(o().close_on_exec(true).close_on_exec(false).oflag, 0_i32);
    }

    #[test]
    fn not_found() {
        let path = c"/this/path/does/not/exist";
        let result = Open::new(AccessMode::ReadOnly).path(path);

        assert_eq!(result.unwrap_err().get(), Error::NotFound as _);
    }

    #[test]
    fn read() {
        let path = c"/dev/random";
        let result = Open::new(AccessMode::ReadOnly).path(path);

        assert!(result.is_ok());
        drop(result);
    }

    #[test]
    fn write() {
        let path = c"/dev/null";
        let result = Open::new(AccessMode::WriteOnly).path(path);

        assert!(result.is_ok());
        drop(result);
    }
}

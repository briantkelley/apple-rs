use crate::_sys::posix::fcntl::{open, O_ACCMODE, O_CLOEXEC, O_RDONLY, O_RDWR, O_WRONLY};
use crate::c::errno::check_retry;
use crate::io::{FromRawFd, OwnedFd};
use core::ffi::CStr;
use core::num::NonZeroI32;

// Note: Additional variants cannot be added. The values must be masked by [`O_ACCMODE`].
#[repr(i32)]
enum AccessModeFlag {
    Read = 0b01,
    Write = 0b10,
}

/// Settings for opening, creating, and accessing a file.
#[derive(Clone, Copy, Debug, Default)]
pub struct OpenOptions {
    options: i32,
}

impl OpenOptions {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn close_on_exec(&mut self, close_on_exec: bool) -> &mut Self {
        self.set_flag_enabled(O_CLOEXEC, close_on_exec)
    }

    pub fn read(&mut self, read: bool) -> &mut Self {
        self.set_flag_enabled(AccessModeFlag::Read as _, read)
    }

    pub fn write(&mut self, write: bool) -> &mut Self {
        self.set_flag_enabled(AccessModeFlag::Write as _, write)
    }

    pub fn open(self, path: impl AsRef<CStr>) -> Result<OwnedFd, NonZeroI32> {
        let path = path.as_ref().as_ptr();
        let oflag = self.oflag();

        // SAFETY: path is guaranteed to be a valid, nul-terminated C-style string and open() will
        // not write to path.
        check_retry(|| unsafe { open(path, oflag) })
            // SAFETY: fd is opened, the unique owner of the resource, and must be `close(2)`ed.
            .map(|fd| unsafe { OwnedFd::from_raw_fd(fd) })
    }

    fn oflag(self) -> i32 {
        let oflag = self.options & !O_ACCMODE;
        let access_mode = match self.options & O_ACCMODE {
            0 => 0,
            am if am == AccessModeFlag::Read as _ => O_RDONLY,
            am if am == AccessModeFlag::Write as _ => O_WRONLY,
            am if am == AccessModeFlag::Read as i32 | AccessModeFlag::Write as i32 => O_RDWR,
            _ => unreachable!(),
        };
        oflag | access_mode
    }

    fn set_flag_enabled(&mut self, flag: i32, enable: bool) -> &mut Self {
        if enable {
            self.options |= flag;
        } else {
            self.options &= !flag;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::OpenOptions;
    use crate::_sys::posix::fcntl::{O_CLOEXEC, O_RDONLY, O_RDWR, O_WRONLY};
    use crate::c::errno::Error;
    use core::ffi::CStr;

    #[test]
    fn access_mode() {
        let mut o = OpenOptions::new();

        assert_eq!(o.oflag(), O_RDONLY);
        assert_eq!(o.read(true).oflag(), O_RDONLY);
        assert_eq!(o.write(true).oflag(), O_RDWR);
        assert_eq!(o.read(false).oflag(), O_WRONLY);
        assert_eq!(o.write(false).oflag(), O_RDONLY);
    }

    #[test]
    fn flags() {
        let mut o = OpenOptions::new();

        assert_eq!(o.close_on_exec(true).oflag(), O_CLOEXEC);
        assert_eq!(o.close_on_exec(false).oflag(), 0);
    }

    #[test]
    fn not_found() {
        let path = CStr::from_bytes_with_nul(b"/this/path/does/not/exist\0").unwrap();
        let result = OpenOptions::new().read(true).open(path);

        assert_eq!(result.unwrap_err().get(), Error::NotFound as _);
    }

    #[test]
    fn read() {
        let path = CStr::from_bytes_with_nul(b"/dev/random\0").unwrap();
        let result = OpenOptions::new().read(true).open(path);

        assert!(matches!(result, Ok(_)));
        drop(result);
    }

    #[test]
    fn write() {
        let path = CStr::from_bytes_with_nul(b"/dev/null\0").unwrap();
        let result = OpenOptions::new().write(true).open(path);

        assert!(matches!(result, Ok(_)));
        drop(result);
    }
}

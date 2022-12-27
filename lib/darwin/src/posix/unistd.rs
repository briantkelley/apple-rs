use crate::_sys::posix::unistd::{self, confstr, mkstemp, _CS_DARWIN_USER_TEMP_DIR};
use crate::c::errno::{self, check_retry};
use crate::io::{FromRawFd, OwnedFd};
use core::ffi::CStr;
use core::num::{NonZeroI32, NonZeroUsize};
use core::ptr;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum ConfigurationString {
    TemporaryDirectory = _CS_DARWIN_USER_TEMP_DIR,
}

impl ConfigurationString {
    /// Get the string value of the configurable variable.
    ///
    /// The return type represents all possible outcomes of a call to `confstr(3)`:
    /// * `Ok(_)`:
    ///    * `Some(NonZeroUsize)`: The call was successful. The value is the buffer size required to
    ///      hold the entire string value, including the nul terminator.
    ///    * `None`: The variable name is valid but does not have a defined value.
    /// * `Err(_)`: The call was not successful and failed due to the provided reason.
    pub fn get(self, buf: Option<&mut [u8]>) -> Result<Option<NonZeroUsize>, NonZeroI32> {
        let (ptr, len) = buf.map_or((ptr::null_mut(), 0), |buf| (buf.as_mut_ptr(), buf.len()));

        // Clear the current error code. This must occur prior to calling the C function to
        // disambiguate an error from "not defined".
        errno::set(None);

        // SAFETY: buf is a mutable slice, thus its range is guaranteed to be a valid write
        // destination. The system function handles null pointers, never overruns the buffer, and
        // always nul terminates the output.
        match NonZeroUsize::new(unsafe { confstr(self as _, ptr.cast(), len) }) {
            // confstr(3) returned 0. There was either an error or there is no entry.
            None => errno::get().map(Err).transpose(),
            // A non-zero result is always the capacity required for the full nul terminated string.
            cap => Ok(cap),
        }
    }
}

/// Takes the given file name `template` and overwrites a portion of it to create a file name. This
/// file name is guaranteed not to exist at the time of function invocation. The template may be any
/// file name with some number of `X`s appended to it (e.g. `/tmp/temp.XXXXXX`). The trailing `X`s
/// are replaced with a unique alphanumeric combination. The number of unique file names depends on
/// the number of `X`s provided (e.g. six `X`s will result in selecting one of 56,800,235,584
/// (62 ** 6) possible temporary file names.
///
/// This function creates the template file, mode 0600, returning the file descriptor opened for
/// reading and writing. This avoids the race between testing for a file's existence and opening it
/// for use.
///
/// # Panics
///
/// Panics if `template` is not nul-terminated or does not end with one or more `X`s.
pub fn create_and_open_unique_file(template: &mut [u8]) -> Result<OwnedFd, NonZeroI32> {
    let mut iter = template.iter().rev();
    assert!(*iter.next().unwrap() == 0);

    let template_len = template.len() - 1 /* nul */;
    let placeholder_range = (template_len - iter.position(|c| *c != b'X').unwrap())..template_len;

    check_retry(|| {
        // SAFETY: template is guaranteed to be a valid mutable buffer. The assert above verifies
        // the buffer is nul-terminated. The system function will only overwrite bytes preceding the
        // nul terminator.
        let result = unsafe { mkstemp(template.as_mut_ptr().cast()) };
        if result == -1
            && matches!(
                errno::get().map(NonZeroI32::get),
                Some(crate::_sys::c::errno::EINTR)
            )
        {
            // template is in an undefined state. Restore the placeholders for the retry.
            template[placeholder_range.clone()].fill(b'X');
        }
        result
    })
    // SAFETY: fd is opened, the unique owner of the resource, and must be `close(2)`ed.
    .map(|fd| unsafe { OwnedFd::from_raw_fd(fd) })
}

pub fn unlink(path: &CStr) -> Result<(), NonZeroI32> {
    let path = path.as_ptr();
    // SAFETY: path is guaranteed to be a valid C-style string. The system function only reads its
    // contents.
    let _ = check_retry(|| unsafe { unistd::unlink(path) })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{create_and_open_unique_file, unlink, ConfigurationString};
    use crate::c::errno::Error;
    use core::ffi::CStr;
    use core::mem;

    const NAMES: [ConfigurationString; 1] = [ConfigurationString::TemporaryDirectory];

    // ConfigurationString

    #[test]
    fn all_strings() {
        // Some character that's unlikely to be written by confstr.
        const NEWLINE: u8 = 10;

        let mut buf: [u8; 100] = unsafe { mem::zeroed() };
        for name in NAMES {
            buf.fill(NEWLINE);

            let cap = name.get(Some(&mut buf)).unwrap().unwrap().get();
            assert!(cap <= buf.len()); // ensure buffer actually contains the entire value
            let len = cap - 1;

            assert!(buf[0..len].iter().all(|c| *c != 0 && *c != NEWLINE));
            assert_eq!(buf[len], 0);
        }
    }

    #[test]
    fn bad_name() {
        let name: ConfigurationString = unsafe { mem::transmute(0) };
        assert_eq!(
            name.get(None).unwrap_err().get(),
            Error::InvalidArgument as _
        );
    }

    #[test]
    fn buffer_too_small() {
        let mut buf: [u8; 2] = [0, 0];

        for name in NAMES {
            let cap = name.get(Some(&mut buf)).unwrap().unwrap().get();

            assert_ne!(buf[0], 0);
            assert_eq!(buf[1], 0);
            assert_ne!(cap, buf.len());
        }
    }

    #[test]
    fn capacity() {
        for name in NAMES {
            assert!(name.get(None).unwrap().is_some());
        }
    }

    // create_and_open_unique_file()

    #[test]
    fn temporary_file() {
        let mut buf: [u8; 512] = unsafe { mem::zeroed() };

        let len = ConfigurationString::TemporaryDirectory
            .get(Some(&mut buf))
            .unwrap()
            .unwrap()
            .get()
            - 1 /* nul */;

        let template = b"temp.XXXXXX";
        let range = len..len + template.len();

        let buf = &mut buf[..=range.end];
        buf[range].copy_from_slice(template);

        let fd = create_and_open_unique_file(buf).unwrap();

        let path = CStr::from_bytes_with_nul(buf).unwrap();
        unlink(path).unwrap();
        drop(fd);

        assert_eq!(template.len(), 5 + 6 /* see next two ranges */);
        let temp_dot_range = len..len + 5;
        let template_range = temp_dot_range.end..temp_dot_range.end + 6;

        assert_eq!(buf[template_range.end], 0);
        assert_ne!(&buf[template_range], b"XXXXXX");
        assert_eq!(&buf[temp_dot_range], b"temp.");
    }
}

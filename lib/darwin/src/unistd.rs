use crate::_sys::unistd::{confstr, _CS_DARWIN_USER_TEMP_DIR};
use crate::errno;
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

#[cfg(test)]
mod tests {
    use super::ConfigurationString;
    use crate::errno::Error;
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
}

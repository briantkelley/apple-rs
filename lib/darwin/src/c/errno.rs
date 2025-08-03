use crate::_sys::c::errno::{self, __error};
use core::num::NonZeroI32;

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Error {
    NotPermitted = errno::EPERM,
    NotFound = errno::ENOENT,
    Interrupted = errno::EINTR,
    IO = errno::EIO,
    NoDevice = errno::ENXIO,
    BadFileDescriptor = errno::EBADF,
    Deadlock = errno::EDEADLK,
    OutOfMemory = errno::ENOMEM,
    NoAccess = errno::EACCES,
    BadAddress = errno::EFAULT,
    ResourceBusy = errno::EBUSY,
    AlreadyExists = errno::EEXIST,
    CrossesDevices = errno::EXDEV,
    NotADirectory = errno::ENOTDIR,
    IsADirectory = errno::EISDIR,
    InvalidArgument = errno::EINVAL,
    SystemFileLimit = errno::ENFILE,
    ProcessFileLimit = errno::EMFILE,
    ExecutableFileBusy = errno::ETXTBSY,
    StorageFull = errno::ENOSPC,
    ReadOnlyFilesystem = errno::EROFS,
    WouldBlock = errno::EAGAIN,
    FilesystemLoop = errno::ELOOP,
    InvalidFilename = errno::ENAMETOOLONG,
    DirectoryNotEmpty = errno::ENOTEMPTY,
    FilesystemQuotaExceeded = errno::EDQUOT,
    Overflow = errno::EOVERFLOW,
    IllegalByteSequence = errno::EILSEQ,
    NotSupported = errno::EOPNOTSUPP,
}

impl From<Error> for NonZeroI32 {
    fn from(error: Error) -> Self {
        // SAFETY: `Error` does not have a zero discriminant.
        unsafe { Self::new_unchecked(error as _) }
    }
}

impl TryFrom<NonZeroI32> for Error {
    type Error = NonZeroI32;

    fn try_from(err: NonZeroI32) -> Result<Self, Self::Error> {
        let variant = match err.get() {
            errno::EPERM => Self::NotPermitted,
            errno::ENOENT => Self::NotFound,
            errno::EINTR => Self::Interrupted,
            errno::EIO => Self::IO,
            errno::ENXIO => Self::NoDevice,
            errno::EBADF => Self::BadFileDescriptor,
            errno::EDEADLK => Self::Deadlock,
            errno::ENOMEM => Self::OutOfMemory,
            errno::EACCES => Self::NoAccess,
            errno::EFAULT => Self::BadAddress,
            errno::EBUSY => Self::ResourceBusy,
            errno::EEXIST => Self::AlreadyExists,
            errno::EXDEV => Self::CrossesDevices,
            errno::ENOTDIR => Self::NotADirectory,
            errno::EISDIR => Self::IsADirectory,
            errno::EINVAL => Self::InvalidArgument,
            errno::ENFILE => Self::SystemFileLimit,
            errno::EMFILE => Self::ProcessFileLimit,
            errno::ETXTBSY => Self::ExecutableFileBusy,
            errno::ENOSPC => Self::StorageFull,
            errno::EROFS => Self::ReadOnlyFilesystem,
            errno::EAGAIN => Self::WouldBlock,
            errno::ELOOP => Self::FilesystemLoop,
            errno::ENAMETOOLONG => Self::InvalidFilename,
            errno::ENOTEMPTY => Self::DirectoryNotEmpty,
            errno::EDQUOT => Self::FilesystemQuotaExceeded,
            errno::EOVERFLOW => Self::Overflow,
            errno::EILSEQ => Self::IllegalByteSequence,
            errno::EOPNOTSUPP => Self::NotSupported,
            _ => Err(err)?,
        };
        Ok(variant)
    }
}

#[must_use]
pub fn get() -> Option<NonZeroI32> {
    // SAFETY: __error() is guaranteed to return a thread-local, non-null pointer.
    NonZeroI32::new(unsafe { *__error() })
}

/// Set the last error number visible to the current thread.
pub fn set(errno: Option<NonZeroI32>) {
    let errno = errno.map_or(0_i32, NonZeroI32::get);
    // SAFETY: __error() is guaranteed to return a thread-specific non-null pointer.
    unsafe { *__error() = errno };
}

/// Returns the value of [`get()`] as an [`Err`] if `result == -1`, otherwise returns the value of
/// `result` as [`Ok`].
pub(crate) fn check(result: i32) -> Result<i32, NonZeroI32> {
    if result == -1 {
        Err(get().unwrap())
    } else {
        Ok(result)
    }
}

/// Calls `f` and validates the result with [`check()`]. Continues to call `f` while the result is
/// the [`Err`] variant with a value of [`Error::Interrupted`]. Otherwise returns the result.
pub(crate) fn check_retry(mut f: impl FnMut() -> i32) -> Result<i32, NonZeroI32> {
    loop {
        match check(f()) {
            Err(e) if e.get() == Error::Interrupted as _ => {}
            result => return result,
        }
    }
}

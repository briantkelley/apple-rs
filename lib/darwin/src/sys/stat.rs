use crate::_sys::sys::stat::{fstat, stat, ALLPERMS, DEFFILEMODE};
use crate::_sys::sys::types::{
    S_IFBLK, S_IFCHR, S_IFDIR, S_IFIFO, S_IFLNK, S_IFMT, S_IFREG, S_IFSOCK, S_IRGRP, S_IROTH,
    S_IRUSR, S_ISGID, S_ISUID, S_ISVTX, S_IWGRP, S_IWOTH, S_IWUSR, S_IXGRP, S_IXOTH, S_IXUSR,
};
use crate::c::errno::check_retry;
use crate::io::AsFd;
use core::mem::MaybeUninit;
use core::num::NonZeroI32;
use core::ops::BitOr;

/// Information about a file.
#[derive(Clone, Copy, Debug)]
pub struct Metadata {
    stat: stat,
}

/// Specifies the type of a file and its [`Permissions`].
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Mode(u16);

/// An specific access right that can be granted to a file.
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Permission {
    UserRead = S_IRUSR,
    UserWrite = S_IWUSR,
    UserExecute = S_IXUSR,
    GroupRead = S_IRGRP,
    GroupWrite = S_IWGRP,
    GroupExecute = S_IXGRP,
    OtherRead = S_IROTH,
    OtherWrite = S_IWOTH,
    OtherExecute = S_IXOTH,

    SetUserID = S_ISUID,
    SetGroupID = S_ISGID,
    Sticky = S_ISVTX,
}

/// A set of granted rights, and the set user ID, set group ID and sticky bits.
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Permissions(u16);

#[allow(clippy::len_without_is_empty)] // not a container type
impl Metadata {
    pub fn from_fd(fd: &impl AsFd) -> Result<Self, NonZeroI32> {
        let mut metadata = Self {
            // SAFETY: stat is a scalar structure that is safe to zero-initialize.
            stat: unsafe { MaybeUninit::<stat>::zeroed().assume_init() },
        };

        // Note: EINTR is not explicitly listed as an error condition for fstat(2), but POSIX
        // section 2.3 does not prohibit implementations from returning additional error codes.

        // SAFETY: The file descriptor and the buffer are guaranteed to be valid. The operating
        // system will not write outside the bounds of the buffer.
        let _ = check_retry(|| unsafe { fstat(fd.as_fd(), &mut metadata.stat) })?;

        Ok(metadata)
    }

    #[must_use]
    pub fn len(&self) -> u64 {
        self.stat.size.try_into().unwrap_or_default()
    }

    #[must_use]
    pub const fn mode(&self) -> Mode {
        Mode(self.stat.mode)
    }
}

impl Mode {
    #[inline]
    #[must_use]
    pub const fn is_block_device(self) -> bool {
        self.file_type() == S_IFBLK
    }

    #[inline]
    #[must_use]
    pub const fn is_char_device(self) -> bool {
        self.file_type() == S_IFCHR
    }

    #[inline]
    #[must_use]
    pub const fn is_dir(self) -> bool {
        self.file_type() == S_IFDIR
    }

    #[inline]
    #[must_use]
    pub const fn is_fifo(self) -> bool {
        self.file_type() == S_IFIFO
    }

    #[inline]
    #[must_use]
    pub const fn is_file(self) -> bool {
        self.file_type() == S_IFREG
    }

    #[inline]
    #[must_use]
    pub const fn is_symbolic_link(self) -> bool {
        self.file_type() == S_IFLNK
    }

    #[inline]
    #[must_use]
    pub const fn is_socket(self) -> bool {
        self.file_type() == S_IFSOCK
    }

    #[inline]
    #[must_use]
    const fn file_type(self) -> u16 {
        self.0 & S_IFMT
    }

    #[inline]
    #[must_use]
    pub const fn permissions(self) -> Permissions {
        Permissions(self.0 & ALLPERMS)
    }

    #[inline]
    #[must_use]
    pub const fn into_raw(self) -> u16 {
        self.0
    }
}

impl BitOr for Permission {
    type Output = Permissions;

    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs: Self::Output = self.into();
        let rhs: Self::Output = rhs.into();
        lhs | rhs
    }
}

impl BitOr<Permissions> for Permission {
    type Output = Permissions;

    fn bitor(self, rhs: Permissions) -> Self::Output {
        let lhs: Self::Output = self.into();
        lhs | rhs
    }
}

impl Permissions {
    /// Tests whether the given `permission` is granted in this permission set.
    #[must_use]
    pub const fn has(self, permission: Permission) -> bool {
        let bit: u16 = permission as _;
        self.0 & bit == bit
    }

    /// Tests whether the given `permissions` are granted in this permission set.
    #[must_use]
    pub const fn has_all(self, permissions: Self) -> bool {
        self.0 & permissions.0 == permissions.0
    }

    /// Tests whether the given `permissions` are not granted in this permission set.
    #[must_use]
    pub const fn has_none(self, permissions: Self) -> bool {
        self.0 & permissions.0 == 0
    }
}

impl BitOr for Permissions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOr<Permission> for Permissions {
    type Output = Self;

    fn bitor(self, rhs: Permission) -> Self::Output {
        let rhs: Self = rhs.into();
        self | rhs
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Self(DEFFILEMODE)
    }
}

impl From<Permission> for Permissions {
    fn from(permission: Permission) -> Self {
        Self(permission as _)
    }
}

#[cfg(test)]
mod tests {
    use super::{Metadata, Permission};
    use crate::posix::fcntl::OpenOptions;
    use core::ffi::CStr;

    #[test]
    fn stat_bin_sh() {
        use Permission::{
            GroupExecute, GroupRead, GroupWrite, OtherExecute, OtherRead, OtherWrite, UserExecute,
            UserRead, UserWrite,
        };

        let path = CStr::from_bytes_with_nul(b"/bin/sh\0").unwrap();
        let fd = OpenOptions::new().read(true).open(path).unwrap();

        let metadata = Metadata::from_fd(&fd).unwrap();

        // Assume the executable is greater than 1 KiB and less than 10 MiB.
        assert!(metadata.len() > 1024 && metadata.len() < 10_485_760);

        // Verify the executable is a file and not any other type.
        let mode = metadata.mode();

        assert!(mode.is_file());
        assert!(!mode.is_block_device());
        assert!(!mode.is_char_device());
        assert!(!mode.is_dir());
        assert!(!mode.is_fifo());
        assert!(!mode.is_symbolic_link());
        assert!(!mode.is_socket());

        // Verify the executable has the default permissions.
        let permissions = mode.permissions();
        assert!(permissions.has(UserRead));
        assert!(permissions.has(UserWrite));
        assert!(permissions.has(UserExecute));
        assert!(permissions.has(GroupRead));
        assert!(!permissions.has(GroupWrite));
        assert!(permissions.has(GroupExecute));
        assert!(permissions.has(OtherRead));
        assert!(!permissions.has(OtherWrite));
        assert!(permissions.has(OtherExecute));

        assert!(permissions.has_all(
            UserRead
                | UserWrite
                | UserExecute
                | GroupRead
                | GroupExecute
                | OtherRead
                | OtherExecute
        ));

        assert!(permissions.has_none(GroupWrite | OtherWrite));
    }
}

use crate::_sys::sys::clonefile::{fclonefileat, CLONE_ACL, CLONE_NOFOLLOW, CLONE_NOOWNERCOPY};
use crate::c::errno::check;
use crate::io::AsFd;
use core::ffi::CStr;
use core::num::NonZeroI32;

#[allow(missing_copy_implementations)]
#[derive(Debug, Default)]
pub struct Clone {
    flags: u32,
}

impl Clone {
    #[must_use]
    pub const fn clone_acl(self, clone_acl: bool) -> Self {
        self.set_flag_enabled(CLONE_ACL, clone_acl)
    }

    #[must_use]
    pub const fn no_follow(self, no_follow: bool) -> Self {
        self.set_flag_enabled(CLONE_NOFOLLOW, no_follow)
    }

    #[must_use]
    pub const fn no_owner_copy(self, no_owner_copy: bool) -> Self {
        self.set_flag_enabled(CLONE_NOOWNERCOPY, no_owner_copy)
    }

    pub fn fd(
        self,
        source: &impl AsFd,
        destination_directory: &impl AsFd,
        destination_file_name: impl AsRef<CStr>,
    ) -> Result<(), NonZeroI32> {
        let srcfd = source.as_fd();
        let dst_dirfd = destination_directory.as_fd();
        let dst = destination_file_name.as_ref().as_ptr();
        let flags = self.flags;

        // SAFETY: srcfd and dst_dirfd are guaranteed to be valid file descriptors, dst is
        // guaranteed to be a valid, nul-terminated C-style string, the system function will not
        // write to the string, and flags is guaranteed to be a valid combination.
        let _ = check(unsafe { fclonefileat(srcfd, dst_dirfd, dst, flags) })?;
        Ok(())
    }

    const fn set_flag_enabled(mut self, flag: u32, enable: bool) -> Self {
        if enable {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Clone;
    use crate::posix::fcntl::Open;
    use crate::posix::unistd::{
        create_unique_directory_and_open, remove_directory, unlink, ConfigurationString,
    };
    use crate::sys::stat::Metadata;
    use core::ffi::CStr;
    use core::mem;

    #[test]
    fn test_clone_fd() {
        let source_path = c"/System/Volumes/Data/Applications/Safari.app/Contents/Info.plist";
        let source = Open::default().path(source_path).unwrap();

        let mut buf: [u8; 512] = unsafe { mem::zeroed() };
        let len = ConfigurationString::TemporaryDirectory
            .get(Some(&mut buf))
            .unwrap()
            .unwrap()
            .get()
            -1 /* nul */;

        let template = b"rust-darwin-XXXXXX";
        let template_end = len + template.len();
        buf[len..template_end].copy_from_slice(template);

        let destination_directory =
            create_unique_directory_and_open(&mut buf[..=template_end]).unwrap();
        let destination_file_name = c"Info.plist";

        Clone::default()
            .fd(&source, &destination_directory, destination_file_name)
            .unwrap();

        let file_name = destination_file_name.to_bytes();
        let file_name_end = template_end + 1 + file_name.len();
        buf[template_end] = b'/';
        buf[(template_end + 1)..file_name_end].copy_from_slice(file_name);

        let file_path = CStr::from_bytes_with_nul(&buf[..=file_name_end]).unwrap();
        let cloned = Open::default().path(file_path).unwrap();

        let source_metadata = Metadata::from_fd(&source).unwrap();
        let cloned_metadata = Metadata::from_fd(&cloned).unwrap();

        assert_eq!(source_metadata.len(), cloned_metadata.len());
        assert_eq!(
            source_metadata.mode().into_raw(),
            cloned_metadata.mode().into_raw()
        );

        unlink(file_path).unwrap();

        buf[template_end] = 0;
        let directory_path = CStr::from_bytes_with_nul(&buf[..=template_end]).unwrap();
        remove_directory(directory_path).unwrap();
    }
}

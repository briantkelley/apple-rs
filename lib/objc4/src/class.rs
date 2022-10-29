use crate::sys::{class_getName, objc_class};
use crate::Object;
use core::cmp::{Eq, PartialEq};
use core::ffi::CStr;
use core::fmt::{self, Debug, Formatter};

impl objc_class {
    /// Returns the name of a class.
    #[must_use]
    pub fn name(&self) -> &CStr {
        let cls = self.as_ptr().cast();
        // SAFETY: `cls` is derived from a reference so it is guaranteed to be a valid pointer to an
        // Objective-C class.
        let name = unsafe { class_getName(cls) }.as_ptr();
        // SAFETY: `class_getName()` is guaranteed to return a valid C-style string.
        unsafe { CStr::from_ptr(name) }
    }
}

impl Debug for objc_class {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let name = self.name().to_str().map_err(|_| fmt::Error)?;
        f.write_str(name)
    }
}

impl Eq for objc_class {}
impl Object for objc_class {}

// Class objects are uniqued so a pointer comparison is sufficient for equality.
impl PartialEq for objc_class {
    fn eq(&self, other: &Self) -> bool {
        let lhs: *const _ = self;
        let rhs: *const _ = other;

        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::{id, Object};

    #[link(name = "Foundation", kind = "framework")]
    extern "C" {
        // From NSString.h
        static NSCharacterConversionException: id;
    }

    #[test]
    fn test_constant_string_class_name() {
        const EXPECTED: &[u8] = b"__NSCFConstantString";

        // object_getClassName()
        assert_eq!(
            unsafe { NSCharacterConversionException.class_name() }.to_bytes(),
            EXPECTED
        );

        // object_getClass(), class_getName()
        assert_eq!(
            unsafe { NSCharacterConversionException.class() }
                .name()
                .to_bytes(),
            EXPECTED
        );
    }

    #[test]
    fn test_meta_class() {
        const META_1: &[u8] = b"__NSCFConstantString";
        const META_2: &[u8] = b"NSObject";

        let class = unsafe { NSCharacterConversionException.class() };
        let meta_class = class.class();

        assert_ne!(class, meta_class);
        assert_eq!(meta_class.name().to_bytes(), META_1);

        // The class of the meta class is NSObject.
        let meta_class_class = meta_class.class();
        assert_ne!(meta_class, meta_class_class);
        assert_eq!(meta_class_class.name().to_bytes(), META_2);

        // The meta class of the NSObject meta class is itself.
        assert_eq!(meta_class_class, meta_class_class.class());
    }
}

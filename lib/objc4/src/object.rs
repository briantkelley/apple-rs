use crate::sys::{objc_class, objc_object, object_getClass, object_getClassName};
use core::ffi::CStr;
use core::fmt::{self, Debug, Formatter};

/// An trait that serves as the base type for Objective-C objects.
///
/// Includes bindings to the Objective-C runtime functions whose names begin with `object_`.
pub trait Object: Debug {
    /// Returns the class of the object.
    fn class(&self) -> &'static objc_class {
        let obj: *const _ = self;
        // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid pointer to an
        // Objective-C object.
        let cls = unsafe { object_getClass(obj as *mut _) };
        // SAFETY: `object_getClass()` guarantees non-null result for any valid Objective-C object.
        unsafe { &*cls }
    }

    /// Returns the class name of the object.
    fn class_name(&self) -> &'static CStr {
        let obj: *const _ = self;
        // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid pointer to an
        // Objective-C object.
        let name = unsafe { object_getClassName(obj as *mut _) }.as_ptr();
        // SAFETY: `object_getClassName()` is guaranteed to return a valid C-style string.
        unsafe { CStr::from_ptr(name) }
    }
}

impl Debug for objc_object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let class_name = self.class_name().to_str().map_err(|_| fmt::Error)?;
        let obj: *const _ = self;

        f.write_fmt(format_args!("<{class_name}: {obj:p}>"))
    }
}

impl Object for objc_object {}

use crate::id;
use crate::sys::{objc_class, objc_object, object_getClass, object_getClassName};
use core::ffi::CStr;
use core::fmt::{self, Debug, Formatter};

/// An trait that serves as the base type for Objective-C objects.
///
/// Includes bindings to the Objective-C runtime functions whose names begin with `object_`.
pub trait Object: Debug + Sized {
    /// Gets the Objective-C class representing the trait type.
    fn class_type() -> &'static objc_class;

    /// Returns the class of the object.
    fn class(&self) -> &objc_class {
        let obj = self.as_ptr();
        // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid pointer to an
        // Objective-C object.
        let cls = unsafe { object_getClass(obj) };
        // SAFETY: `object_getClass()` guarantees non-null result for any valid Objective-C object.
        unsafe { &*cls }
    }

    /// Returns the class name of the object.
    fn class_name(&self) -> &CStr {
        let obj = self.as_ptr();
        // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid pointer to an
        // Objective-C object.
        let name = unsafe { object_getClassName(obj) }.as_ptr();
        // SAFETY: `object_getClassName()` is guaranteed to return a valid C-style string.
        unsafe { CStr::from_ptr(name) }
    }

    /// Returns the non-null, generic Objective-C object pointer for the object instance.
    fn as_ptr(&self) -> id {
        let obj: *const _ = self;
        (obj as *mut Self).cast()
    }
}

impl Object for id {
    fn class_type() -> &'static objc_class {
        panic!() // The "any object" type does not have a class type.
    }

    fn as_ptr(&self) -> id {
        *self
    }
}

impl Debug for objc_object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let class_name = self.class_name().to_str().map_err(|_| fmt::Error)?;
        let obj = self.as_ptr();

        f.write_fmt(format_args!("<{class_name}: {obj:p}>"))
    }
}

impl Object for objc_object {
    fn class_type() -> &'static objc_class {
        panic!() // There is no root class type in Objective-C
    }
}

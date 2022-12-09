use crate::sys::{objc_class, objc_object, object_getClass, object_getClassName, sel_registerName};
use core::ffi::{c_char, c_void, CStr};
use core::fmt::{self, Debug, Formatter};

/// An trait that serves as the base type for Objective-C objects.
///
/// Includes bindings to the Objective-C runtime functions whose names begin with `object_`.
pub trait Object: Debug {
    fn class(&self) -> &'static objc_class {
        let obj: *const _ = self;
        // SAFETY: The reference is guaranteed to be a valid pointer.
        let cls = unsafe { object_getClass(obj as *mut _) };
        // SAFETY: `object_getClass()` guarantees non-null result for any valid Objective-C object.
        unsafe { &*cls }
    }

    fn class_name(&self) -> &'static CStr {
        let obj: *const _ = self;
        // SAFETY: The reference is guaranteed to be a valid pointer.
        let name = unsafe { object_getClassName(obj as *mut _) }.as_ptr();
        // SAFETY: `object_getClassName()` is guaranteed to return a valid C-style string.
        unsafe { CStr::from_ptr(name) }
    }
}

impl Debug for objc_object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: Use a compile-time constant
        let sel = unsafe { sel_registerName("debugDescription\0".as_ptr().cast()) };

        // -debugDescription and -description return NSString, which is implemented by Foundation,
        // not libobjc. This relies on the fact the dependency inversion is resolved at runtime.

        // The API notes indicate both `-debugDescription` and `-description` return non-nil values.
        let description: &Self = if msg_send!((bool)[self, respondsToSelector:(*const c_void)sel.as_ptr()])
        {
            msg_send!((claim nonnull id)[self, debugDescription])
        } else {
            msg_send!((claim nonnull id)[self, description])
        };

        let str = msg_send!((*const c_char)[description, UTF8String]);
        if str.is_null() {
            None::<Self>.fmt(f)
        } else {
            // SAFETY: str is guaranteed to be a valid C string pointer.
            unsafe { CStr::from_ptr(str) }.fmt(f)
        }
    }
}

impl Object for objc_object {}

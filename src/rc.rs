use crate::sys::objc_object;
use crate::{id, Object};
use core::ptr::NonNull;

/// The core functionality implemented by all smart pointers.
pub trait Rc {
    /// The interface implemented by the Objective-C object instance.
    type T: Object;

    /// Constructs a smart pointer from a raw, balanced, non-null Objective-C object instance pointer.
    ///
    /// To avoid a memory leak, the object must not require an additional release.
    fn new_retaining(obj: NonNull<objc_object>) -> Self;

    /// Constructs a smart pointer from a raw, non-null Objective-C object instance pointer, and
    /// takes ownership from the caller (i.e. balancing an outstand +1 retain count with a release).
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory unsafety (via over-release),
    /// even if the returned smart pointer is never accessed.
    unsafe fn new_transfer(obj: NonNull<objc_object>) -> Self;

    /// Consumes the smart pointer and transfers ownership of the raw, non-null Objective-C object
    /// instance pointer to the caller.
    ///
    /// To avoid a memory leak, the returned pointer must be released.
    #[must_use]
    fn into_retained_ptr(self) -> id;
}

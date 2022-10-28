use crate::sys::{objc_object, objc_release, objc_retain};
use crate::{id, Object};
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::forget;
use core::ops::{Deref, DerefMut};
use core::ptr::NonNull;

/// A pointer type for an Objective-C object instance.
///
/// Boxes provide unique ownership for an Objective-C object instance, and drop their contents when
/// they go out of scope.
pub struct Box<T>
where
    T: Object,
{
    pub(crate) obj: NonNull<objc_object>,
    phantom: PhantomData<T>,
}

impl<T> Box<T>
where
    T: Object,
{
    /// Constructs a new box from a raw, balanced, non-null Objective-C object instance pointer.
    ///
    /// To avoid a memory leak, the object must not require an additional release.
    #[must_use]
    pub fn with_retained(obj: NonNull<objc_object>) -> Self {
        // SAFETY: Caller is responsible for ensuring `obj` is a valid, balanced object pointer.
        let _ = unsafe { objc_retain(obj.as_ptr()) };
        Self {
            obj,
            phantom: PhantomData,
        }
    }

    /// Constructs a new box from a raw, non-null Objective-C object instance pointer, and takes
    /// ownership from the caller (i.e. balancing an outstand +1 retain count with a release).
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory unsafety (via over-release),
    /// even if the returned smart pointer is never accessed.
    #[must_use]
    pub const unsafe fn with_transfer(obj: NonNull<objc_object>) -> Self {
        Self {
            obj,
            phantom: PhantomData,
        }
    }

    /// Consumes the box and transfers ownership of the raw, non-null, retained Objective-C object
    /// instance pointer to the caller.
    ///
    /// To avoid a memory leak, the returned pointer must be released.
    #[must_use]
    pub fn into_retained_ptr(self) -> id {
        let obj = self.obj;
        forget(self);
        obj.as_ptr()
    }
}

impl<T> AsRef<T> for Box<T>
where
    T: Object,
{
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> Borrow<T> for Box<T>
where
    T: Object,
{
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> BorrowMut<T> for Box<T>
where
    T: Object,
{
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> Debug for Box<T>
where
    T: Object,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.obj.as_ptr().fmt(f)
    }
}

impl<T> Deref for Box<T>
where
    T: Object,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let obj = self.obj.as_ptr().cast();
        // SAFETY: An object pointer owned by `Box<T>` is guaranteed to be valid.
        unsafe { &*obj }
    }
}

impl<T> DerefMut for Box<T>
where
    T: Object,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        let obj = self.obj.as_ptr().cast();
        // SAFETY: An object pointer owned by `Box<T>` is guaranteed to be valid.
        unsafe { &mut *obj }
    }
}

impl<T> Drop for Box<T>
where
    T: Object,
{
    fn drop(&mut self) {
        let obj = self.obj.as_ptr();
        // SAFETY: An object pointer owned by `Box<T>` is guaranteed to be valid. The ownership must
        // be relinquished when the `Box<T>` instance is destroyed.
        unsafe { objc_release(obj) }
    }
}

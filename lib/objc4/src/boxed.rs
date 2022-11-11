use crate::sys::{objc_object, objc_release, objc_retain};
use crate::{Object, Upcast};
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

    /// Converts the type interface from `T` to `U`. Assumes the caller guarantees safety.
    ///
    /// # Safety
    ///
    /// If the Objective-C object instance is not a kind of `U`, undefined behavior may occur if the
    /// object is accessed or messaged.
    #[must_use]
    pub unsafe fn transmute_unchecked<U>(self) -> Box<U>
    where
        U: Object,
    {
        let new = Box::<U> {
            obj: self.obj,
            phantom: PhantomData,
        };
        forget(self);
        new
    }

    /// Safely upcasts the contents of the box from `T` to `U`.
    ///
    /// This is necessary because Rust does not support type inheritance and Objective-C objects
    /// cannot generally be represented as fat pointers.
    #[must_use]
    pub fn upcast<'a, U>(self) -> Box<U>
    where
        T: Object + Upcast<&'a T, &'a U> + 'a,
        U: Object + 'a,
    {
        let new = Box::<U> {
            obj: self.obj,
            phantom: PhantomData,
        };
        forget(self);
        new
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

impl<T> Eq for Box<T> where T: Eq + Object {}

impl<T, U> PartialEq<Box<U>> for Box<T>
where
    T: Object + PartialEq<U>,
    U: Object,
{
    fn eq(&self, other: &Box<U>) -> bool {
        **self == **other
    }
}

impl<T, U> PartialEq<U> for Box<T>
where
    T: Object + PartialEq<U>,
    U: Object,
{
    fn eq(&self, other: &U) -> bool {
        &**self == other
    }
}

use crate::sys::{objc_object, objc_release, objc_retain};
use crate::{Box, Object};
use core::borrow::Borrow;
use core::fmt::{self, Debug, Formatter};
use core::marker::PhantomData;
use core::mem::forget;
use core::ops::Deref;
use core::ptr::NonNull;

/// A thread-safe reference-counting pointer for an Objective-C object instance. ‘Arc’ is a double
/// entendre that stands for both ‘Atomically Reference Counted’ (like `std::sync::Arc`) and
/// ‘Automatic Reference Counting’ (like the Clang Objective-C [language feature][clang]).
///
/// The type `Arc<T>` provides shared ownership of an Objective-C object instance with an interface
/// trait of type `T`, allocated in the heap. Invoking [`clone`][clone] on `Arc` produces a new
/// `Arc` instance, which points to the same allocation on the heap as the source `Arc`, while
/// increasing a reference count. When the last `Arc` pointer to a given allocation is destroyed,
/// the value stored in that allocation (often referred to as “inner value”) is also dropped.
///
/// Shared references in Rust disallow mutation by default, and `Arc` is no exception: you cannot
/// generally obtain a mutable reference to something inside an Arc.
///
/// [clang]: https://clang.llvm.org/docs/AutomaticReferenceCounting.html
/// [clone]: Clone::clone
#[repr(transparent)]
pub struct Arc<T>
where
    T: Object,
{
    obj: NonNull<objc_object>,
    phantom: PhantomData<T>,
}

impl<T> Arc<T>
where
    T: Object,
{
    /// Creates a reference-counting pointer from a uniquely owned Objective-C object pointer.
    #[must_use]
    pub fn new(ptr: Box<T>) -> Self {
        let obj = ptr.obj;
        forget(ptr);
        Self {
            obj,
            phantom: PhantomData,
        }
    }

    /// Constructs a reference-counting pointer from a raw, balanced, non-null Objective-C object
    /// instance pointer.
    ///
    /// To avoid a memory leak, the object must not require an additional release.
    #[must_use]
    pub fn with_retain(obj: NonNull<objc_object>) -> Self {
        // SAFETY: Caller is responsible for ensuring `obj` is a valid, balanced object pointer.
        let _ = unsafe { objc_retain(obj.as_ptr()) };
        Self {
            obj,
            phantom: PhantomData,
        }
    }

    /// Constructs a reference-counting pointer from a raw, non-null Objective-C object instance
    /// pointer, and takes ownership from the caller (i.e. balancing an outstand +1 retain count
    /// with a release).
    ///
    /// # Safety
    ///
    /// This function is unsafe because improper use may lead to memory unsafety (via over-release),
    /// even if the returned reference-counting pointer is never accessed.
    #[must_use]
    pub const unsafe fn with_transfer(obj: NonNull<objc_object>) -> Self {
        Self {
            obj,
            phantom: PhantomData,
        }
    }
}

impl<T> AsRef<T> for Arc<T>
where
    T: Object,
{
    fn as_ref(&self) -> &T {
        self
    }
}

impl<T> Borrow<T> for Arc<T>
where
    T: Object,
{
    fn borrow(&self) -> &T {
        self
    }
}

impl<T> Clone for Arc<T>
where
    T: Object,
{
    fn clone(&self) -> Self {
        Self::with_retain(self.obj)
    }
}

impl<T> Debug for Arc<T>
where
    T: Object,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.obj.as_ptr().fmt(f)
    }
}

impl<T> Deref for Arc<T>
where
    T: Object,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let obj = self.obj.as_ptr().cast();
        // SAFETY: An object pointer owned by `Arc<T>` is guaranteed to be valid.
        unsafe { &*obj }
    }
}

impl<T> Drop for Arc<T>
where
    T: Object,
{
    fn drop(&mut self) {
        let obj = self.obj.as_ptr();
        // SAFETY: An object pointer owned by `Arc<T>` is guaranteed to be valid. The ownership must
        // be relinquished when the `Arc<T>` instance is destroyed.
        unsafe { objc_release(obj) }
    }
}

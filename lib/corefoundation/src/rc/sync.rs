//! A pointer type that provides memory management for shared object instances.
//!
//! An [`Arc<T>`] provides shared ownership of a Core Foundation object instance, and releases the
//! object instance when dropped.

use crate::boxed::Box;
use crate::ffi::ForeignFunctionInterface;
use crate::rc::impl_rc;
use core::mem::forget;
use core::ptr::NonNull;

/// A thread-safe reference-counting pointer for a Core Foundation object instance.
///
/// An `Arc<T>` provides shared ownership of a Core Foundation object instance, and releases the
/// object instance when dropped.
///
/// Invoking [`clone`] on `Arc<T>` produces a new `Arc<T>` instance, which points to the same Core
/// Foundation object instance as the source `Arc<T>`, while increasing a reference count.
///
/// Shared references in Rust disallow mutation by default, and `Arc<T>` is no exception: you cannot
/// generally obtain a mutable reference to something inside an `Arc<T>`.
///
/// [`clone`]: Clone::clone
pub struct Arc<T>(NonNull<T>)
where
    T: ForeignFunctionInterface;

impl<T> Arc<T>
where
    T: ForeignFunctionInterface,
{
    /// Constructs a new `Arc<T>` from a raw, non-null Core Foundation object instance pointer
    /// obtained from a function following [The Create Rule][].
    ///
    /// The object will be released when the new `Arc<T>` is dropped, balancing the initial retain
    /// from the create function.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of `T::Raw`.
    /// 3. You must enforce Rust’s aliasing rules if the lifetime provided by [`Arc<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while this [`Arc<T>`]
    ///    exists, the memory the pointer points to must not get mutated.
    /// 4. The pointer must point to an object instance compatible with the polymorphic Core
    ///    Foundation functions and the bindings implemented by `T`.
    /// 5. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    ///
    /// [The Create Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    #[inline]
    #[must_use]
    pub const unsafe fn with_create_rule(cf: NonNull<T::Raw>) -> Self {
        Self(cf.cast())
    }
}

impl_rc!(Arc);

impl<T> Clone for Arc<T>
where
    T: ForeignFunctionInterface,
{
    #[inline]
    fn clone(&self) -> Self {
        let cf = self.0.cast();
        // SAFETY: The creator of the smart pointer asserted `self.0` met all the safety criteria
        // of an `Arc<T>` by constructing the smart pointer.
        unsafe { T::from_get_rule(cf) }
    }
}

impl<T> From<Box<T>> for Arc<T>
where
    T: ForeignFunctionInterface,
{
    #[inline]
    fn from(value: Box<T>) -> Self {
        let cf = value.0;
        // Don't let `value` drop, causing `value.0` to be released, because its ownership of `cf`
        // is being transferred to the new `Arc<T>`, which will release `cf` when dropped.
        forget(value);
        Self(cf)
    }
}

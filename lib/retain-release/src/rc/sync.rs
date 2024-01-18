//! A pointer type that provides memory management for shared object instances.
//!
//! An [`Arc<T>`] expresses shared ownership of an object type. The smart pointer releases its
//! reference count on the object instance when dropped.

use crate::boxed::Box;
use crate::ffi::ForeignFunctionInterface;
use crate::rc::impl_rc;
use core::mem::forget;
use core::ptr::NonNull;

/// A thread-safe reference-counting pointer for an object instance.
///
/// An `Arc<T>` provides shared ownership of an object instance, and releases the object instance
/// when dropped.
///
/// Invoking [`clone`] on `Arc<T>` produces a new `Arc<T>` instance, which points to the same object
/// instance as the source `Arc<T>`, while increasing a reference count.
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
    /// Constructs a new `Arc<T>` from a raw, non-null, owned object instance pointer.
    ///
    /// The object will be released when the new `Arc<T>` is dropped, relinquishing the ownership
    /// that was transferred to the `Arc<T>` by the caller.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of `T::Raw`.
    /// 3. You must enforce Rust's aliasing rules if the lifetime provided by [`Arc<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while the [`Arc<T>`]
    ///    exists, the memory the pointer points to must not get mutated.
    /// 4. The pointer must point to an object instance that can be cast and dereferenced to an
    ///    instance of `T`.
    /// 5. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    #[inline]
    #[must_use]
    pub const unsafe fn from_owned_ptr(ptr: NonNull<T::Raw>) -> Self {
        Self(ptr.cast())
    }
}

impl_rc!(Arc);

impl<T> Clone for Arc<T>
where
    T: ForeignFunctionInterface,
{
    #[inline]
    fn clone(&self) -> Self {
        let ptr = self.0.cast();
        // SAFETY: The creator of the smart pointer asserted `self.0` met all the safety criteria
        // of an `Arc<T>` by constructing the smart pointer.
        unsafe { T::from_borrowed_ptr(ptr) }
    }
}

impl<T> From<Box<T>> for Arc<T>
where
    T: ForeignFunctionInterface,
{
    #[inline]
    fn from(value: Box<T>) -> Self {
        let ptr = value.0;
        // Don't let `value` drop, causing `value.0` to be released, because its ownership of `ptr`
        // is being transferred to the new `Arc<T>`, which will release `ptr` when dropped.
        forget(value);
        Self(ptr)
    }
}

// SAFETY: `Arc` is [`Send`] if `T` is both [`Send`] and [`Sync`].
//
// `Arc` does not enable `T` to become [`Send`]. Consider a handle to a thread-local resource. If an
// `Arc` was moved to another thread, the resource would likely leak. When the `Arc` is [`Drop`]ped,
// closing the handle may cause a runtime error or prematurely destruct a resource in use by the
// active thread. Therefore, [`Send`] is transitive, not additive.
//
// Implementing [`Send`] for `Arc` also requires that `T` be [`Sync`]. Consider a type with interior
// mutability. After sending the [`Clone`] of an `Arc` to another thread, the `T` may be accessed by
// two threads simultaneously. Therefore it is only safe to [`Send`] and `Arc` to another thread if
// `T` supports synchronous access.
//
// Apple's reference counting implementations are thread-safe, so `T` is the sole determining factor
// in whether it's safe to transfer ownership to another thread.
unsafe impl<T> Send for Arc<T> where T: ForeignFunctionInterface + Send + Sync {}

// SAFETY: `Arc` is [`Sync`] if `T` is both [`Send`] and [`Sync`].
//
// Implementing [`Sync`] for `Arc` also requires that `T` be [`Send`]. If an `&Arc<T>` is sent to
// another thread, [`Clone`] can be used to obtain an `Arc<T>` owned by the other thread. If all
// other `Arc<T>`s are [`Drop`]ped before this clone, `T` will [`Drop`] on the other thread due to
// the [`Clone`] having effectively performed a [`Send`].
//
// Apple's reference counting implementations are thread-safe, so `T` is the sole determining factor
// in whether it's safe to use allow parallel reference counting operations across threads.
unsafe impl<T> Sync for Arc<T> where T: ForeignFunctionInterface + Send + Sync {}

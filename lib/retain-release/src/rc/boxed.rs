//! A pointer type that provides memory management for uniquely owned object instances.
//!
//! A [`Box<T>`] expresses exclusive ownership of an object type. The smart pointer releases its
//! reference count on the object instance when dropped.

use super::impl_rc;
use crate::ffi::ForeignFunctionInterface;
use core::borrow::BorrowMut;
use core::ops::DerefMut;
use core::ptr::NonNull;

/// An owned (i.e., exclusive) pointer for an object instance.
///
/// A `Box<T>` provides shared ownership of an object instance, and releases the object instance
/// when dropped.
pub struct Box<T>(pub(super) NonNull<T>)
where
    T: ForeignFunctionInterface;

impl<T> Box<T>
where
    T: ForeignFunctionInterface,
{
    /// Constructs a new `Box<T>` from a raw, non-null uniquely owned object instance pointer.
    ///
    /// The new [`Box<T>`] **must** have exclusive ownership of the object instance pointer. If the
    /// object instance can be accessed in another context (e.g., global state), or the object
    /// instance is otherwise not exclusively pointed to by `ptr`,  construct a new [`Arc<T>`]
    /// instead (use of `Box<T>` with a shared object may result in undefined behavior).
    ///
    /// The object will be released when the new `Box<T>` is dropped, relinquishing the ownership
    /// that was transferred to the `Arc<T>` by the caller.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of `T::Raw`.
    /// 3. You must enforce Rust's aliasing rules if the lifetime provided by [`Box<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while the [`Box<T>`]
    ///    exists, the memory the pointer points to must not get accessed (read or written) through
    ///    any other pointer.
    /// 4. The pointer must point to an object instance that can be cast and dereferenced to an
    ///    instance of `T`.
    /// 5. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    ///
    /// [`Arc<T>`]: crate::sync::Arc
    #[inline]
    #[must_use]
    pub const unsafe fn from_owned_mut_ptr(ptr: NonNull<T::Raw>) -> Self {
        Self(ptr.cast())
    }
}

impl_rc!(Box);

impl<T> AsMut<T> for Box<T>
where
    T: ForeignFunctionInterface,
{
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> BorrowMut<T> for Box<T>
where
    T: ForeignFunctionInterface,
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> DerefMut for Box<T>
where
    T: ForeignFunctionInterface,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The creator of the smart pointer asserted all the [`NonNull::as_mut`] safety
        // criteria were met by constructing the smart pointer.
        unsafe { self.0.as_mut() }
    }
}

// SAFETY: `Box` is [`Send`] if `T` is [`Send`] because the instance of `T` is unaliased. Apple's
// reference counting implementations are thread-safe, so `T` is the sole determining factor in
// whether it's safe to transfer ownership to another thread.
unsafe impl<T> Send for Box<T> where T: ForeignFunctionInterface + Send {}

// SAFETY: `Box` is [`Sync`] if `T` is [`Sync`] because the instance of `T` is unaliased. Apple's
// reference counting implementations are thread-safe, so `T` is the sole determining factor in
// whether it's safe to use allow parallel reference counting operations across threads.
unsafe impl<T> Sync for Box<T> where T: ForeignFunctionInterface + Sync {}

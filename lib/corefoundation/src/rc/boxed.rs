//! A pointer type that provides memory management for uniquely owned object instances.
//!
//! A [`Box<T>`] acquires the exclusive ownership of a Core Foundation object instance, and releases
//! the object instance when dropped.

use super::impl_rc;
use crate::ffi::ForeignFunctionInterface;
use core::borrow::BorrowMut;
use core::ops::DerefMut;
use core::ptr::NonNull;

/// An owned (i.e., exclusive) pointer for a Core Foundation object instance.
pub struct Box<T>(pub(super) NonNull<T>)
where
    T: ForeignFunctionInterface;

impl<T> Box<T>
where
    T: ForeignFunctionInterface,
{
    /// Constructs a new `Box<T>` from a raw, non-null Core Foundation object instance pointer
    /// obtained from a function following [The Create Rule][].
    ///
    /// The new `Box<T>` **must** have exclusive ownership of the object instance pointer. If the
    /// object instance can be accessed from another context (e.g., via global state, Core
    /// Foundation internals, etc.), or the object instance is otherwise not exclusively pointed to
    /// by `cf`, construct a new [`Arc<T>`] instead (use of `Box<T>` with a shared object may result
    /// in undefined behavior).
    ///
    /// The object will be released when the new `Box<T>` is dropped, balancing the initial retain
    /// from the create function.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of `T::Raw`.
    /// 3. You must enforce Rust’s aliasing rules if the lifetime provided by [`Box<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while this [`Box<T>`]
    ///    exists, the memory the pointer points to must not get accessed (read or written) through
    ///    any other pointer.
    /// 4. The pointer must point to an object instance compatible with the polymorphic Core
    ///    Foundation functions and the bindings implemented by `T`.
    /// 5. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    ///
    /// [`Arc<T>`]: crate::sync::Arc
    /// [The Create Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    #[inline]
    #[must_use]
    pub const unsafe fn with_create_rule(cf: NonNull<T::Raw>) -> Self {
        Self(cf.cast())
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

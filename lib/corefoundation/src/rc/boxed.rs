//! A pointer type that provides memory management for uniquely owned object instances.
//!
//! A [`Box<T>`] acquires the exclusive ownership of a Core Foundation object instance, and releases
//! the object instance when dropped.

use super::impl_rc;
use crate::Object;
use core::borrow::BorrowMut;
use core::ops::DerefMut;
use core::ptr::NonNull;
use corefoundation_sys::CFRelease;

/// An owned (i.e., exclusive) pointer for a Core Foundation object instance.
pub struct Box<T>(pub(super) NonNull<T>)
where
    T: Object;

impl<T> Box<T>
where
    T: Object,
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
    /// This function is unsafe because:
    ///
    /// 1. If the Core Foundation object instance is not exclusively owned by the new `Box<T>`,
    ///    usage of the object instance may result in undefined behavior the object is mutated while
    ///    a reference has been obtained through [`Deref`], or if a mutable reference obtained
    ///    through [`DerefMut`] is not actually unique. If the Core Foundation object instance
    ///    cannot be exclusively owned by the new `Box<T>`, construct a new [`Arc<T>`] instead.
    /// 2. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    /// 3. `cf` must be compatible with the implementation of `T`, which can only be verified
    ///    through code inspection.
    ///
    /// [`Arc<T>`]: crate::sync::Arc
    /// [`Deref`]: core::ops::Deref
    /// [The Create Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    #[inline]
    #[must_use]
    pub const unsafe fn with_create_rule(cf: NonNull<T::Raw>) -> Self {
        Self(cf.cast())
    }

    // Note: There is no constructor for object instance pointers obtained from functions following
    // "The Get Rule" because, by definition, those objects cannot be exclusively owned.
}

impl_rc!(Box);

impl<T> AsMut<T> for Box<T>
where
    T: Object,
{
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> BorrowMut<T> for Box<T>
where
    T: Object,
{
    #[inline]
    fn borrow_mut(&mut self) -> &mut T {
        self
    }
}

impl<T> DerefMut for Box<T>
where
    T: Object,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The pointer is properly aligned (we assume it was allocated by a conforming
        // allocator), it is "dereferenceable", it points to an initialized instance of `T` (again,
        // we assume it was initialized by its create function), the smart pointer guarantees the
        // data will live at least as long as itself, and [`Box<T>`] guarantees the mutable
        // reference is unique.
        unsafe { self.0.as_mut() }
    }
}

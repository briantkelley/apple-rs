//! A pointer type that provides memory management for shared object instances.
//!
//! An [`Arc<T>`] provides shared ownership of a Core Foundation object instance, and releases the
//! object instance when dropped.

use crate::boxed::Box;
use crate::rc::impl_rc;
use crate::Object;
use core::mem::forget;
use core::ptr::NonNull;
use corefoundation_sys::{CFRelease, CFRetain};

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
    T: Object;

impl<T> Arc<T>
where
    T: Object,
{
    /// Constructs a new `Arc<T>` from a raw, non-null Core Foundation object instance pointer
    /// obtained from a function following [The Create Rule][].
    ///
    /// The object will be released when the new `Arc<T>` is dropped, balancing the initial retain
    /// from the create function.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    ///
    /// 1. If the Core Foundation object instance is mutable and is visible outside of the Rust
    ///    language boundary, usage of the object instance may result in undefined behavior the
    ///    object is mutated while a reference has been obtained through [`Deref`]. If Rust's
    ///    aliasing rules cannot be applied to the Core Foundation object instance (specifically
    ///    that the memory the pointer points does not get mutated while a shared reference has been
    ///    obtained through `Arc<T>`), it is **not** safe to use in any context.
    /// 2. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    /// 3. `cf` must be compatible with the implementation of `T`, which can only be verified
    ///    through code inspection.
    ///
    /// [`Deref`]: core::ops::Deref
    /// [The Create Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    #[inline]
    #[must_use]
    pub const unsafe fn with_create_rule(cf: NonNull<T::Raw>) -> Self {
        Self(cf.cast())
    }

    /// Constructs a new `Arc<T>` from a raw, non-null Core Foundation object instance pointer
    /// obtained from a function following [The Get Rule][].
    ///
    /// The object is retained by the `Arc<T>` to ensure its lifetime is at least as long as the
    /// `Arc<T>`, and the object will be released when the `Arc<T>` is dropped, balancing the retain
    /// added by this constructor.
    ///
    /// Note that if this constructor is incorrectly used in place of [`with_create_rule`], a memory
    /// leak will result, though this not considered unsound behavior.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    ///
    /// 1. If the Core Foundation object instance is mutable and is visible outside of the Rust
    ///    language boundary, usage of the object instance may result in undefined behavior the
    ///    object is mutated while a reference has been obtained through [`Deref`]. If Rust's
    ///    aliasing rules cannot be applied to the Core Foundation object instance (specifically
    ///    that the memory the pointer points does not get mutated while a shared reference has been
    ///    obtained through `Arc<T>`), it is **not** safe to use in any context.
    /// 2. `cf` must be compatible with the implementation of `T`, which can only be verified
    ///    through code inspection.
    ///
    /// [`Deref`]: core::ops::Deref
    /// [`with_create_rule`]: Self::with_create_rule
    /// [The Get Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-SW1
    #[inline]
    #[must_use]
    pub unsafe fn with_get_rule(cf: NonNull<T::Raw>) -> Self {
        {
            let cf = cf.as_ptr().cast();
            // Note: [`CFRetain`] is guaranteed to return its argument.
            // SAFETY: `cf` is a non-null pointer to a [`CFTypeRef`].
            let _ = unsafe { CFRetain(cf) };
        }
        Self(cf.cast())
    }
}

impl_rc!(Arc);

impl<T> Clone for Arc<T>
where
    T: Object,
{
    #[inline]
    fn clone(&self) -> Self {
        let cf = self.0.cast();
        // SAFETY: `self.0` is known to conform to all safety requirements given the prior
        // construction of `self`.
        unsafe { Self::with_get_rule(cf) }
    }
}

impl<T> From<Box<T>> for Arc<T>
where
    T: Object,
{
    #[inline]
    fn from(value: Box<T>) -> Self {
        let cf = value.0;
        // Don't let value drop (and release the value) because we're moving its retain into `Self`.
        forget(value);
        Self(cf)
    }
}

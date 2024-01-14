//! Apple defines types compatible with the polymorphic Core Foundation functions outside of the
//! Core Foundation framework (e.g., `CoreGraphics`, `CoreText`). The facilities this crate uses to
//! provide idiomatic Rust API bindings for Core Foundation are available to crates implementing
//! Rust API bindings for frameworks with Core Foundation-compatible types.
//!
//! [`ForeignFunctionInterface`] is the primary trait used to bridge the between the foreign
//! function interface and Rust. It provides a facility to retrieve the [`CFTypeRef`] pointer from
//! the Rust type for use in calling foreign functions. This trait **should not** be used by crates
//! utilizing the Rust API bindings; it's intended only for crates *implementing* Rust API bindings.
//! This is intentionally separate from [`Object`] so the FFI related functionality is not visible
//! by default when using the [`Object`] interface.
//!
//! [`CFTypeRef`]: corefoundation_sys::CFTypeRef
//! [`Object`]: crate::Object

use crate::boxed::Box;
use crate::sync::Arc;
use core::ptr::NonNull;

pub mod convert;

/// A trait to facilitate implementation of Rust bindings for frameworks that implement Core
/// Foundation object types.
///
/// This is separate from [`Object`] to limit the scope in which it may be misused—the trait must be
/// explicitly used in a module to gain visibility of these raw bindings facilities.
///
/// # Safety
///
/// This type is `unsafe` to implement because whether any particular pointer type is compatible
/// with the polymorphic Core Foundation functions cannot be verified at compile time. This must be
/// verified through code inspection.
///
/// [`Object`]: crate::Object
pub unsafe trait ForeignFunctionInterface {
    /// The type the [`CFTypeRef`] pointer points to.
    ///
    /// [`CFTypeRef`]: corefoundation_sys::CFTypeRef
    type Raw;

    /// `NULL`-checks the newly created but shared raw object instance pointer and places the
    /// instance in an `Arc<T>`.
    ///
    /// The object will be released when its new `Arc<T>` is dropped, balancing the initial retain
    /// rom the function following [The Create Rule][] that returned the raw pointer.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
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
    unsafe fn try_from_create_rule(cf: *const Self::Raw) -> Option<Arc<Self>>
    where
        Self: Sized,
    {
        NonNull::new(cf.cast_mut()).map(|cf| {
            // SAFETY: Caller asserts `cf` meets all safety requirements.
            unsafe { Self::from_create_rule(cf) }
        })
    }

    /// Places the newly created but shared raw object instance pointer in an `Arc<T>`.
    ///
    /// The object will be released when its new `Arc<T>` is dropped, balancing the initial retain
    /// rom the function following [The Create Rule][] that returned the raw pointer.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
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
    unsafe fn from_create_rule(cf: NonNull<Self::Raw>) -> Arc<Self>
    where
        Self: Sized,
    {
        // SAFETY: Caller asserts `cf` meets all safety requirements.
        unsafe { Arc::with_create_rule(cf) }
    }

    /// `NULL`-checks the newly created raw pointer with exclusive ownership of the object instance
    /// and places the instance in a `Box<T>`.
    ///
    /// The new `Box<T>` **must** have exclusive ownership of the object instance pointer. If the
    /// object instance can be accessed from another context (e.g., via global state, Core
    /// Foundation internals, etc.), or the object instance is otherwise not exclusively pointed to
    /// by `cf`, use [`try_from_create_rule`] instead (use of this constructor with a shared object
    /// may result in undefined behavior).
    ///
    /// The object will be released when its new `Box<T>` is dropped, balancing the initial retain
    /// from the function following [The Create Rule][] that returned the raw pointer.
    ///
    /// **Note:** If the object instance is immutable, use [`try_from_create_rule`] instead, even if
    /// the pointer has exclusive ownership. Immutable objects do not benefit from exclusive
    /// ownership, which enables mutable borrows. [`Arc<T>`] and [`Box<T>`] are used to encode Core
    /// Foundation's mutability rules into the type system, in addition to managing memory lifetime.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust’s aliasing rules if the lifetime provided by [`Box<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while this [`Box<T>`]
    ///    exists, the memory the pointer points to must not get accessed (read or written) through
    ///    any other pointer.
    /// 4. The pointer must point to an object instance compatible with the polymorphic Core
    ///    Foundation functions and the bindings implemented by `T`.
    /// 5. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    ///
    /// [`try_from_create_rule`]: Self::try_from_create_rule
    /// [The Create Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    #[inline]
    unsafe fn try_from_create_rule_mut(cf: *mut Self::Raw) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        NonNull::new(cf).map(|cf| {
            // SAFETY: Caller asserts `cf` meets all safety requirements.
            unsafe { Self::from_create_rule_mut(cf) }
        })
    }

    /// Places the newly created raw pointer with exclusive ownership of the object instance in a
    /// `Box<T>`.
    ///
    /// The new `Box<T>` **must** have exclusive ownership of the object instance pointer. If the
    /// object instance can be accessed from another context (e.g., via global state, Core
    /// Foundation internals, etc.), or the object instance is otherwise not exclusively pointed to
    /// by `cf`, use [`try_from_create_rule`] instead (use of this constructor with a shared object
    /// may result in undefined behavior).
    ///
    /// The object will be released when its new `Box<T>` is dropped, balancing the initial retain
    /// from the function following [The Create Rule][] that returned the raw pointer.
    ///
    /// **Note:** If the object instance is immutable, use [`try_from_create_rule`] instead, even if
    /// the pointer has exclusive ownership. Immutable objects do not benefit from exclusive
    /// ownership, which enables mutable borrows. [`Arc<T>`] and [`Box<T>`] are used to encode Core
    /// Foundation's mutability rules into the type system, in addition to managing memory lifetime.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust’s aliasing rules if the lifetime provided by [`Box<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while this [`Box<T>`]
    ///    exists, the memory the pointer points to must not get accessed (read or written) through
    ///    any other pointer.
    /// 4. The pointer must point to an object instance compatible with the polymorphic Core
    ///    Foundation functions and the bindings implemented by `T`.
    /// 5. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    ///
    /// [`try_from_create_rule`]: Self::try_from_create_rule
    /// [The Create Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    #[inline]
    #[must_use]
    unsafe fn from_create_rule_mut(cf: NonNull<Self::Raw>) -> Box<Self>
    where
        Self: Sized,
    {
        // SAFETY: Caller asserts `cf` meets all safety requirements.
        unsafe { Box::with_create_rule(cf) }
    }

    /// `NULL`-checks the existing, unowned shared raw object instance pointer obtained from a
    /// function following [The Get Rule][] and places the instance in an `Arc<T>`.
    ///
    /// The object will be retained before constructing the new `Arc<T>`, and will be released when
    /// the `Arc<T>` is dropped.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust’s aliasing rules if the lifetime provided by [`Arc<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while this [`Arc<T>`]
    ///    exists, the memory the pointer points to must not get mutated.
    /// 4. The pointer must point to an object instance compatible with the polymorphic Core
    ///    Foundation functions and the bindings implemented by `T`.
    ///
    /// [The Get Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-SW1
    #[inline]
    #[must_use]
    unsafe fn try_from_get_rule(cf: *const Self::Raw) -> Option<Arc<Self>>
    where
        Self: Sized,
    {
        NonNull::new(cf.cast_mut()).map(|cf| {
            // SAFETY: Caller asserts `cf` meets all safety requirements.
            unsafe { Self::from_get_rule(cf) }
        })
    }

    /// Places the existing, unowned shared raw object instance pointer obtained from a function
    /// following [The Get Rule][] in an `Arc<T>`.
    ///
    /// The object will be retained before constructing the new `Arc<T>`, and will be released when
    /// the `Arc<T>` is dropped.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust’s aliasing rules if the lifetime provided by [`Arc<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while this [`Arc<T>`]
    ///    exists, the memory the pointer points to must not get mutated.
    /// 4. The pointer must point to an object instance compatible with the polymorphic Core
    ///    Foundation functions and the bindings implemented by `T`.
    ///
    /// [The Get Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-SW1
    #[must_use]
    unsafe fn from_get_rule(cf: NonNull<Self::Raw>) -> Arc<Self>
    where
        Self: Sized;

    /// Decrements the reference count (retain count) of the foreign object-like type. If the
    /// reference count reaches zero, the object-like type releases/frees its resources and
    /// deallocates itself.
    ///
    /// # Safety
    ///
    /// After calling this associated function, the caller must ensure it **does not** use the
    /// reference passed as the argument again. Use of the reference as unsound as the underlying
    /// memory may have been freed. The argument type is a reference, not a move of `Self`, for ease
    /// of use with [`Drop::drop`].
    unsafe fn release(this: &mut Self);

    /// Gets the raw [`CFTypeRef`] pointer. This should only be used by binding implementations.
    ///
    /// This function is not unsafe but use of the pointer is. It **must not** be used in a way that
    /// would cause an additional reference to the object instance to be created (i.e., the object
    /// instance must not be retained by the callee), unless the function takes care to ensure the
    /// new reference is safe (i.e., takes ownership of a [`Box<T>`] and returns an [`Arc<T>`], or
    /// is an associated function with an [`Arc<T>`] input).
    ///
    /// [`CFTypeRef`]: corefoundation_sys::CFTypeRef
    #[inline]
    fn as_ptr(&self) -> *const Self::Raw {
        let ptr: *const _ = self;
        ptr.cast()
    }

    /// Gets the raw [`CFTypeRef`] pointer. This should only be used by binding implementations.
    ///
    /// This function is not unsafe but use of the pointer is. It **must not** be used in a way that
    /// would cause an additional reference to the object instance to be created (i.e., the object
    /// instance must not be retained by the callee).
    ///
    /// [`CFTypeRef`]: corefoundation_sys::CFTypeRef
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Raw {
        let ptr: *mut _ = self;
        ptr.cast()
    }
}

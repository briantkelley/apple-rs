//! Defines the primary trait for managing object ownership across the Rust/foreign interface
//! boundary.

use crate::boxed::Box;
use crate::sync::Arc;
use core::ptr::NonNull;

/// A trait for use in bridging between a foreign function interface with reference counting
/// semantics and Rust.
///
/// This trait **should not** be used by crates utilizing Rust API bindings; it's intended only for
/// crates *implementing* Rust API bindings.
pub trait ForeignFunctionInterface {
    /// The type of the foreign function interface pointer for which an an implementation of this
    /// trait provides bindings.
    type Raw;

    /// `NULL`-checks the shared (and immutable) raw object instance pointer and places the instance
    /// in an [`Arc<T>`].
    ///
    /// The object will be released when the returned [`Arc<T>`] is dropped, relinquishing the
    /// ownership that was transferred to the `Arc<T>` by the caller.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust's aliasing rules if the lifetime provided by [`Arc<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while the [`Arc<T>`] or
    ///    any [`clone`]s exist, the memory the pointer points to must not get mutated.
    /// 4. The pointer must point to an object instance that can be cast and dereferenced to an
    ///    instance of `T`.
    ///
    /// If the object instance does not have a retain that must be balanced, it will be
    /// over-released, which may result in undefined behavior.
    ///
    /// [`clone`]: [`Arc<T>::clone`]
    #[inline]
    unsafe fn try_from_owned_ptr(ptr: *const Self::Raw) -> Option<Arc<Self>>
    where
        Self: Sized,
    {
        NonNull::new(ptr.cast_mut()).map(|ptr| {
            // SAFETY: Caller asserts `ptr` meets all safety requirements.
            unsafe { Self::from_owned_ptr(ptr) }
        })
    }

    /// Places the shared (and immutable) raw object instance pointer in an [`Arc<T>`].
    ///
    /// The object will be released when the returned [`Arc<T>`] is dropped, relinquishing the
    /// ownership that was transferred to the `Arc<T>` by the caller.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust's aliasing rules if the lifetime provided by [`Arc<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while the [`Arc<T>`] or
    ///    any [`clone`]s exist, the memory the pointer points to must not get mutated.
    /// 4. The pointer must point to an object instance that can be cast and dereferenced to an
    ///    instance of `T`.
    ///
    /// If the object instance does not have a retain that must be balanced, it will be
    /// over-released, which may result in undefined behavior.
    ///
    /// [`clone`]: Arc::clone
    #[inline]
    #[must_use]
    unsafe fn from_owned_ptr(ptr: NonNull<Self::Raw>) -> Arc<Self>
    where
        Self: Sized,
    {
        // SAFETY: Caller asserts `ptr` meets all safety requirements.
        unsafe { Arc::from_owned_ptr(ptr) }
    }

    /// `NULL`-checks the unique (and mutable) raw object instance pointer and places the instance
    /// in a [`Box<T>`].
    ///
    /// The new [`Box<T>`] **must** have exclusive ownership of the object instance pointer. If the
    /// object instance can be accessed in another context (e.g., global state), or the object
    /// instance is otherwise not exclusively pointed to by `ptr`, use [`try_from_owned_ptr`]
    /// instead (use of this constructor with a shared object may result in undefined behavior).
    ///
    /// The object will be released when the returned [`Box<T>`] is dropped, relinquishing the
    /// ownership that was transferred to the `Box<T>` by the caller.
    ///
    /// **Note:** If the object instance is immutable, use [`try_from_owned_ptr`] instead, even if
    /// the pointer has exclusive ownership. Immutable objects do not benefit from [`Box<T>`], which
    /// allows mutable borrows.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust's aliasing rules if the lifetime provided by [`Box<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while the [`Box<T>`] or
    ///    [`Arc<T>`]s created from the `Box<T>` exist, the memory the pointer points to must not be
    ///    accessed (read or written) through any other pointer.
    /// 4. The pointer must point to an object instance that can be cast and dereferenced to an
    ///    instance of `T`.
    ///
    /// If the object instance does not have a retain that must be balanced, it will be
    /// over-released, which may result in undefined behavior.
    ///
    /// [`try_from_owned_ptr`]: Self::try_from_owned_ptr
    #[inline]
    unsafe fn try_from_owned_mut_ptr(ptr: *mut Self::Raw) -> Option<Box<Self>>
    where
        Self: Sized,
    {
        NonNull::new(ptr).map(|ptr| {
            // SAFETY: Caller asserts `ptr` meets all safety requirements.
            unsafe { Self::from_owned_mut_ptr(ptr) }
        })
    }

    /// Places the unique (and mutable) raw object instance pointer in a [`Box<T>`].
    ///
    /// The new [`Box<T>`] **must** have exclusive ownership of the object instance pointer. If the
    /// object instance can be accessed in another context (e.g., global state), or the object
    /// instance is otherwise not exclusively pointed to by `ptr`, use [`from_owned_ptr`] instead
    /// (use of this constructor with a shared object may result in undefined behavior).
    ///
    /// The object will be released when the returned [`Box<T>`] is dropped, relinquishing the
    /// ownership that was transferred to the `Box<T>` by the caller.
    ///
    /// **Note:** If the object instance is immutable, use [`from_owned_ptr`] instead, even if the
    /// pointer has exclusive ownership. Immutable objects do not benefit from [`Box<T>`], which
    /// allows mutable borrows.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust's aliasing rules if the lifetime provided by [`Box<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while the [`Box<T>`] or
    ///    [`Arc<T>`]s created from the `Box<T>` exist, the memory the pointer points to must not be
    ///    accessed (read or written) through any other pointer.
    /// 4. The pointer must point to an object instance that can be cast and dereferenced to an
    ///    instance of `T`.
    ///
    /// If the object instance does not have a retain that must be balanced, it will be
    /// over-released, which may result in undefined behavior.
    ///
    /// [`from_owned_ptr`]: Self::from_owned_ptr
    #[inline]
    #[must_use]
    unsafe fn from_owned_mut_ptr(ptr: NonNull<Self::Raw>) -> Box<Self>
    where
        Self: Sized,
    {
        // SAFETY: Caller asserts `ptr` meets all safety requirements.
        unsafe { Box::from_owned_mut_ptr(ptr) }
    }

    /// `NULL`-checks the shared (and immutable) unowned raw object instance pointer, adds a
    /// reference count, and places the instance in an [`Arc<T>`].
    ///
    /// The object is retained before constructing the returned [`Arc<T>`], and the object will be
    /// released when the returned `Arc<T>` is dropped.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust's aliasing rules if the lifetime provided by [`Arc<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while the [`Arc<T>`] or
    ///    any [`clone`]s exist, the memory the pointer points to must not get mutated.
    /// 4. The pointer must point to an object instance that can be cast and dereferenced to an
    ///    instance of `T`.
    ///
    /// If the object instance has a retain that must be balanced, it will be over-retained, which
    /// may result in a memory leak (though Rust does not consider leaks to be a safety related).
    ///
    /// [`clone`]: [`Arc<T>::clone`]
    #[inline]
    unsafe fn try_from_unowned_ptr(ptr: *const Self::Raw) -> Option<Arc<Self>>
    where
        Self: Sized,
    {
        NonNull::new(ptr.cast_mut()).map(|ptr| {
            // SAFETY: Caller asserts `ptr` meets all safety requirements.
            unsafe { Self::from_unowned_ptr(ptr) }
        })
    }

    /// Adds a reference count to the shared (and immutable) raw object instance pointer and places
    /// the instance in an [`Arc<T>`].
    ///
    /// The object is retained before constructing the returned [`Arc<T>`], and the object will be
    /// released when the returned `Arc<T>` is dropped.
    ///
    /// # Safety
    ///
    /// When calling this constructor, you must ensure all the following are true:
    ///
    /// 1. The pointer must be properly aligned.
    /// 2. The pointer must point to an initialized instance of [`Self::Raw`].
    /// 3. You must enforce Rust's aliasing rules if the lifetime provided by [`Arc<T>`] does not
    ///    wholly reflect the actual lifetime of the data. In particular, while the [`Arc<T>`] or
    ///    any [`clone`]s exist, the memory the pointer points to must not get mutated.
    /// 4. The pointer must point to an object instance that can be cast and dereferenced to an
    ///    instance of `T`.
    ///
    /// If the object instance has a retain that must be balanced, it will be over-retained, which
    /// may result in a memory leak (though Rust does not consider leaks to be a safety related).
    ///
    /// [`clone`]: [`Arc<T>::clone`]
    #[must_use]
    unsafe fn from_unowned_ptr(ptr: NonNull<Self::Raw>) -> Arc<Self>
    where
        Self: Sized;

    /// Decrements the reference count (retain count) of the foreign object. If the reference count
    /// reaches zero, the object implementations releases/frees its resources and deallocates
    /// itself.
    ///
    /// # Safety
    ///
    /// After calling this associated function, the caller must ensure it **does not** use any
    /// reference to `this` again. Use of the reference may caused undefined behavior if the
    /// underlying memory was freed. The argument type is a reference, not a move of `Self`, for
    /// easier use with [`Drop::drop`].
    ///
    /// This should only be called by [`Arc<T>`] and [`Box<T>`].
    unsafe fn release(this: &mut Self);

    /// Gets the raw object instance pointer of the foreign object. This should only be used by
    /// bindings implementations.
    ///
    /// This function is not unsafe but use of the pointer is. It **must not** be used in a way that
    /// would cause a reference count to be added to the object instance (i.e., the object instance
    /// must not be retained by the callee), unless the caller can guarantee the new reference
    /// cannot cause a violation of Rust's aliasing rules.
    #[inline]
    fn as_ptr(&self) -> *const Self::Raw {
        let ptr: *const _ = self;
        ptr.cast()
    }

    /// Gets the raw object instance pointer of the foreign object. This should only be used by
    /// bindings implementations.
    ///
    /// This function is not unsafe but use of the pointer is. It **must not** be used in a way that
    /// would cause a reference count to be added to the object instance (i.e., the object instance
    /// must not be retained by the callee). Rust's aliasing rules will be violated if a reference
    /// to the object instance escapes from the callee.
    #[inline]
    fn as_mut_ptr(&mut self) -> *mut Self::Raw {
        let ptr: *mut _ = self;
        ptr.cast()
    }
}

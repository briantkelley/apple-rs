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

use crate::boxed::Box;
use crate::sync::Arc;
use crate::Object;
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
pub unsafe trait ForeignFunctionInterface {
    /// The type the [`CFTypeRef`] pointer points to.
    ///
    /// [`CFTypeRef`]: corefoundation_sys::CFTypeRef
    type Raw;

    /// Places the newly created raw pointer with shared ownership of the object instance into an
    /// `Arc<T>`.
    ///
    /// The object will be released when its new `Arc<T>` is dropped, balancing the initial retain
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
    ///
    /// [`Deref`]: core::ops::Deref
    /// [The Create Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    #[inline]
    unsafe fn with_create_rule(cf: *const Self::Raw) -> Option<Arc<Self>>
    where
        Self: Object + Sized,
    {
        NonNull::new(cf.cast_mut()).map(|cf| {
            // SAFETY: Caller asserts `cf` meets all safety requirements.
            unsafe { Arc::with_create_rule(cf) }
        })
    }

    /// Places the newly created raw pointer that has exclusive ownership of the object instance
    /// into a `Box<T>`.
    ///
    /// The new `Box<T>` **must** have exclusive ownership of the object instance pointer. If the
    /// object instance can be accessed from another context (e.g., via global state, Core
    /// Foundation internals, etc.), or the object instance is otherwise not exclusively pointed to
    /// by `cf`, use [`with_create_rule`] instead (use of this constructor with a shared object may
    /// result in undefined behavior).
    ///
    /// The object will be released when its new `Box<T>` is dropped, balancing the initial retain
    /// from the function following [The Create Rule][] that returned the raw pointer.
    ///
    /// **Note:** If the object instance is immutable, use [`with_create_rule`] instead, even if the
    /// pointer has exclusive ownership. Immutable objects do not benefit from exclusive ownership,
    /// which enables mutable borrows. [`Arc<T>`] and [`Box<T>`] are used to encode Core
    /// Foundation's mutability rules into the type system, in addition to managing memory lifetime.
    ///
    /// # Safety
    ///
    /// This function is unsafe because:
    ///
    /// 1. If the Core Foundation object instance is not exclusively owned by the reference held by
    ///    `cf`, usage of the object instance may result in undefined behavior the object is mutated
    ///    while a reference has been obtained through [`Deref`], or if a mutable reference obtained
    ///    through [`DerefMut`] is not actually unique. If the Core Foundation object instance is
    ///    not exclusively owned by the `cf` pointer, use [`with_create_rule`] instead.
    /// 2. If the object instance does not have a retain that must be balanced, it will be
    ///    over-released, which may result in undefined behavior.
    ///
    /// [`Deref`]: core::ops::Deref
    /// [`DerefMut`]: core::ops::DerefMut
    /// [`with_create_rule`]: Self::with_create_rule
    /// [The Create Rule]: https://developer.apple.com/library/archive/documentation/CoreFoundation/Conceptual/CFMemoryMgmt/Concepts/Ownership.html#//apple_ref/doc/uid/20001148-103029
    #[inline]
    unsafe fn with_create_rule_mut(cf: *mut Self::Raw) -> Option<Box<Self>>
    where
        Self: Object + Sized,
    {
        NonNull::new(cf).map(|cf| {
            // SAFETY: Caller asserts `cf` meets all safety requirements.
            unsafe { Box::with_create_rule(cf) }
        })
    }

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

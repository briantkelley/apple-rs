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

    /// Gets the raw [`CFTypeRef`] pointer. This should only be used by binding implementations.
    ///
    /// This function is not unsafe but use of the pointer is. It **must not** be used in a way that
    /// would cause an additional reference to the object instance to be created (i.e., the object
    /// instance must not be retained by the callee), unless the function takes care to ensure the
    /// new reference is safe.
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

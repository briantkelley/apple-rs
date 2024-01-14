//! Apple defines types compatible with the polymorphic Core Foundation functions outside of the
//! Core Foundation framework (e.g., `CoreGraphics`, `CoreText`). The facilities this crate uses to
//! provide idiomatic Rust API bindings for Core Foundation are available to crates implementing
//! Rust API bindings for frameworks with Core Foundation-compatible types.
//!
//! The [`define_and_impl_type`] macro defines a new opaque type to represent the foreign type. It
//! also implements:
//!
//! * [`ForeignFunctionInterface`] to bridge the between the foreign function interface and Rust. It
//!   provides a facility to retrieve the [`CFTypeRef`] pointer from the Rust type for use in
//!   calling foreign functions. This trait **should not** be used by crates utilizing the Rust API
//!   bindings; it's intended only for crates *implementing* Rust API bindings.
//! * [`Object`], which identifies a type as compatible with the polymorphic Core Foundation
//!   functions.
//!
//! [`CFTypeRef`]: corefoundation_sys::CFTypeRef
//! [`ForeignFunctionInterface`]: crate::ffi::ForeignFunctionInterface

/// The base trait of all Core Foundation objects.
pub trait Object {}

/// Defines a new type on which to implement Rust bindings for a Core Foundation object type. This
/// macro also implements the [`Object`], [`Debug`] [`Eq`], and [`PartialEq`] traits on the new
/// type.
///
/// This macro also implements [`ForeignFunctionInterface`] on the new type. The instantiator
/// guarantees the safety of this by defining `$ty` as the bindings type for the `$raw_ty` Core
/// Foundation pointer type, which is compatible with the polymorphic Core Foundation functions and
/// the bindings implemented in `$ty`.
///
/// A new type is required to implement the many of the standard traits, as the type definition
/// originates in a separate `-sys` crate.
///
/// [`Debug`]: core::fmt::Debug
/// [`ForeignFunctionInterface`]: crate::ffi::ForeignFunctionInterface
#[macro_export]
macro_rules! define_and_impl_type {
    ($(#[$doc:meta])* $ty:ident, raw: $raw_ty:ident) => {
        $crate::opaque_type!($(#[$doc])* $ty);

        #[allow(unused_qualifications)]
        impl $crate::ffi::ForeignFunctionInterface for $ty {
            type Raw = $raw_ty;

            #[inline]
            unsafe fn from_borrowed_ptr(
                ptr: core::ptr::NonNull<Self::Raw>
            ) -> $crate::sync::Arc<Self>
            where
                Self: Sized,
            {
                let cf = ptr.as_ptr().cast();
                // SAFETY: `cf` is a non-null pointer to a [`CFTypeRef`].
                let cf = unsafe { corefoundation_sys::CFRetain(cf) }.cast_mut();
                // SAFETY: [`CFRetain`] is guaranteed to return its argument.
                let cf = unsafe { core::ptr::NonNull::new_unchecked(cf) }.cast();
                // SAFETY: Caller asserts `cf` meets all safety requirements.
                unsafe { $crate::sync::Arc::from_owned_ptr(cf) }
            }

            #[inline]
            unsafe fn release(this: &mut Self) {
                let cf = this.as_ptr().cast();
                // SAFETY: The creator of the `Self` instance asserted `Self::Raw` is compatible
                // with the polymorphic Core Foundation functions.
                unsafe { corefoundation_sys::CFRelease(cf) };
            }
        }

        impl $crate::Object for $ty {}

        #[allow(unused_qualifications)]
        impl core::fmt::Debug for $ty {
            /// Returns a string that contains a description of the object.
            ///
            /// The nature of the description differs by object. For example, a description of an
            /// array may include the description of each of element in the collection.
            ///
            /// You can use this method for debugging Core Foundation objects, but note, however,
            /// that the description for a given object may be different in different releases of
            /// the operating system. Do not depend on the content or format of the information
            /// returned by this function.
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let cf = <Self as $crate::ffi::ForeignFunctionInterface>::as_ptr(self).cast();
                // SAFETY: `cf` is a non-null pointer to a [`CFTypeRef`].
                let description = unsafe { corefoundation_sys::CFCopyDescription(cf) };
                // PANIC: [`CFCopyDescription`] never returns null for non-null inputs.
                let description_cf = core::ptr::NonNull::new(description.cast_mut())
                    .expect("CFCopyDescription returned NULL");
                // SAFETY: [`CFCopyDescription`] returns a [`CFStringRef`] following the create rule
                let string = unsafe { $crate::string::String::from_owned_ptr(description_cf) };

                write!(f, "{}", &*string)
            }
        }

        #[allow(unused_qualifications)]
        impl core::cmp::Eq for $ty {}

        #[allow(unused_qualifications)]
        impl core::cmp::PartialEq for $ty {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                let cf1 = <Self as $crate::ffi::ForeignFunctionInterface>::as_ptr(self).cast();
                let cf2 = <Self as $crate::ffi::ForeignFunctionInterface>::as_ptr(other).cast();

                // SAFETY: `cf1` and `cf2` are non-null pointers to [`CFTypeRef`]s.
                let result = unsafe { corefoundation_sys::CFEqual(cf1, cf2) };
                result != 0
            }
        }

        #[allow(unused_qualifications)]
        impl<D> core::cmp::PartialEq<D> for $ty
        where
            D: core::ops::Deref<Target = Self>,
        {
            #[inline]
            fn eq(&self, other: &D) -> bool {
                <Self as core::cmp::PartialEq>::eq(self, other)
            }
        }
    };
}

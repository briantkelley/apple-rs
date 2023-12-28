use crate::ffi::ForeignFunctionInterface;

/// The base trait of all Core Foundation objects.
pub trait Object: ForeignFunctionInterface {}

/// Declares a new type on which to implement Rust bindings for a Core Foundation object type. This
/// macro also implements the [`Object`], [`Eq`], and [`PartialEq`] traits on the new type.
///
/// A new type is required to implement the many of the standard traits, as the type definition
/// originates in a separate `-sys` crate.
///
/// The user of this macro must manually implement [`ForeignFunctionInterface`] on the new type.
#[macro_export]
macro_rules! declare_and_impl_type {
    ($(#[$doc:meta])* $ty:ident) => {
        $(#[$doc])*
        // LINT: This type is not intended to be user accessible.
        #[allow(missing_copy_implementations)]
        #[repr(C)]
        pub struct $ty {
            _data: [u8; 0],
            _marker: core::marker::PhantomData<(*const u8, core::marker::PhantomPinned)>,
        }

        impl $crate::Object for $ty {}

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

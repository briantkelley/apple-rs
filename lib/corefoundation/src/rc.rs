pub mod boxed;
pub mod sync;

macro_rules! impl_rc {
    ($name:ident) => {
        impl<T> AsRef<T> for $name<T>
        where
            T: $crate::Object,
        {
            #[inline]
            fn as_ref(&self) -> &T {
                self
            }
        }

        impl<T> core::borrow::Borrow<T> for $name<T>
        where
            T: $crate::Object,
        {
            #[inline]
            fn borrow(&self) -> &T {
                self
            }
        }

        impl<T> core::fmt::Debug for $name<T>
        where
            T: $crate::Object + core::fmt::Debug,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <T as core::fmt::Debug>::fmt(self, f)
            }
        }

        impl<T> core::ops::Deref for $name<T>
        where
            T: $crate::Object,
        {
            type Target = T;

            #[inline]
            fn deref(&self) -> &Self::Target {
                // SAFETY: The pointer is properly aligned (we assume it was allocated by a
                // conforming allocator), it is "dereferenceable", it points to an initialized
                // instance of `T` (again, we assume it was initialized by its create function), and
                // the smart pointer guarantees the data will live at least as long as itself.
                unsafe { self.0.as_ref() }
            }
        }

        impl<T> core::fmt::Display for $name<T>
        where
            T: $crate::Object + core::fmt::Display,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <T as core::fmt::Display>::fmt(self, f)
            }
        }

        impl<T> Drop for $name<T>
        where
            T: $crate::Object,
        {
            #[inline]
            fn drop(&mut self) {
                let cf = self.0.as_ptr().cast();
                // SAFETY: `cf` is a non-null pointer to a [`CFTypeRef`].
                unsafe { CFRelease(cf) }
            }
        }

        impl<T> Eq for $name<T> where T: $crate::Object + Eq {}

        impl<T> core::hash::Hash for $name<T>
        where
            T: $crate::Object + core::hash::Hash,
        {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: core::hash::Hasher,
            {
                <T as core::hash::Hash>::hash(self, state)
            }
        }

        impl<T> Ord for $name<T>
        where
            T: $crate::Object + Ord,
        {
            #[inline]
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                <T as Ord>::cmp(self, other)
            }
        }

        impl<T> PartialEq for $name<T>
        where
            T: $crate::Object + PartialEq,
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                <T as PartialEq>::eq(self, other)
            }

            // LINT: `T` may have an optimized version.
            #[allow(clippy::partialeq_ne_impl)]
            #[inline]
            fn ne(&self, other: &Self) -> bool {
                <T as PartialEq>::ne(self, other)
            }
        }

        impl<T> PartialEq<&T> for $name<T>
        where
            T: $crate::Object + PartialEq,
        {
            #[inline]
            fn eq(&self, other: &&T) -> bool {
                <T as PartialEq>::eq(self, other)
            }

            // LINT: `T` may have an optimized version.
            #[allow(clippy::partialeq_ne_impl)]
            #[inline]
            fn ne(&self, other: &&T) -> bool {
                <T as PartialEq>::ne(self, other)
            }
        }

        impl<T> PartialOrd for $name<T>
        where
            T: $crate::Object + PartialOrd,
        {
            #[inline]
            fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
                <T as PartialOrd>::partial_cmp(self, rhs)
            }

            #[inline]
            fn lt(&self, other: &Self) -> bool {
                <T as PartialOrd>::lt(self, other)
            }

            #[inline]
            fn le(&self, other: &Self) -> bool {
                <T as PartialOrd>::le(self, other)
            }

            #[inline]
            fn gt(&self, other: &Self) -> bool {
                <T as PartialOrd>::gt(self, other)
            }

            #[inline]
            fn ge(&self, other: &Self) -> bool {
                <T as PartialOrd>::ge(self, other)
            }
        }

        impl<T> PartialOrd<&T> for $name<T>
        where
            T: $crate::Object + PartialOrd,
        {
            #[inline]
            fn partial_cmp(&self, rhs: &&T) -> Option<core::cmp::Ordering> {
                <T as PartialOrd>::partial_cmp(self, rhs)
            }

            #[inline]
            fn lt(&self, other: &&T) -> bool {
                <T as PartialOrd>::lt(self, other)
            }

            #[inline]
            fn le(&self, other: &&T) -> bool {
                <T as PartialOrd>::le(self, other)
            }

            #[inline]
            fn gt(&self, other: &&T) -> bool {
                <T as PartialOrd>::gt(self, other)
            }

            #[inline]
            fn ge(&self, other: &&T) -> bool {
                <T as PartialOrd>::ge(self, other)
            }
        }

        impl<T> core::fmt::Pointer for $name<T>
        where
            T: $crate::Object,
        {
            #[inline]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Pointer::fmt(&self.0, f)
            }
        }

        // SAFETY: Core Foundation provides thread-safe reference counting, so if T is [`Send`],
        // it's safe to transfer ownership to another thread.
        unsafe impl<T> Send for $name<T> where T: $crate::Object + Send {}

        // SAFETY: Core Foundation provides thread-safe reference counting, so if T is [`Sync`],
        // it's safe to use allow parallel reference counting operations across threads.
        unsafe impl<T> Sync for $name<T> where T: $crate::Object + Sync {}
    };
}

use impl_rc;

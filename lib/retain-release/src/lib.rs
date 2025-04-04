//! Support for building idiomatic Rust bindings for foreign heap-allocated, reference-counted
//! objects.
//!
//! ## Getting Started
//!
//! The `ForeignFunctionInterface` trait facilitates conversion between the foreign interface's
//! object pointer type with reference counting semantics and the native Rust type implementing the
//! bindings. The trait is straightforward to implement:
//!
//! * Specify the type of the foreign interface object pointer with [`Raw`].
//! * Implement the retain operation in [`from_unowned_ptr`].
//! * Implement the release operation in [`release`].
//!
//! ```
//! # #[repr(C)]
//! # struct ForeignType(u8);
//! # struct RustBindings;
//! #
//! # extern "C" {
//! #     fn foreign_type_retain(ptr: *const ForeignType);
//! #     fn foreign_type_release(ptr: *const ForeignType);
//! # }
//! #
//! use core::ptr::NonNull;
//! use retain_release::ffi::ForeignFunctionInterface;
//! use retain_release::sync::Arc;
//!
//! impl ForeignFunctionInterface for RustBindings {
//!     type Raw = ForeignType;
//!
//!     unsafe fn from_unowned_ptr(ptr: NonNull<Self::Raw>) -> Arc<Self>
//!     where
//!         Self: Sized,
//!     {
//!         foreign_type_retain(ptr.as_ptr());
//!         Arc::from_owned_ptr(ptr)
//!     }
//!
//!     unsafe fn release(this: &mut Self) {
//!         let ptr = this.as_ptr().cast();
//!         foreign_type_release(ptr);
//!     }
//! }
//! ```
//!
//! Then, when implementing the native Rust type, use the appropriate constructor to wrap the
//! foreign interface object pointer in a smart pointer to gain automatic retain/release support and
//! the benefits of Rust's well-defined aliasing rules (assuming the smart pointer safety
//! requirements can be met).
//!
//! ```
//! # #[repr(C)]
//! # struct ForeignType(u8);
//! # struct RustBindings;
//! #
//! # extern "C" {
//! #     fn foreign_type_create_with_context(ctx: *const ()) -> *mut ForeignType;
//! #     fn foreign_type_get_current() -> *const ForeignType;
//! # }
//! #
//! use retain_release::boxed::Box;
//! use retain_release::ffi::ForeignFunctionInterface;
//! use retain_release::sync::Arc;
//!
//! # impl ForeignFunctionInterface for RustBindings {
//! #     type Raw = ForeignType;
//! #
//! #     unsafe fn from_unowned_ptr(ptr: core::ptr::NonNull<Self::Raw>) -> Arc<Self>
//! #     where
//! #         Self: Sized
//! #     { todo!() }
//! #
//! #     unsafe fn release(this: &mut Self) { todo!() }
//! # }
//! #
//! impl RustBindings {
//!     fn current() -> Arc<Self> {
//!         let ptr = unsafe { foreign_type_get_current() };
//!         unsafe { Self::try_from_unowned_ptr(ptr) }.unwrap()
//!     }
//!
//!     fn with_context(ctx: *const ()) -> Option<Box<Self>> {
//!         let ptr = unsafe { foreign_type_create_with_context(ctx) };
//!         unsafe { Self::try_from_owned_mut_ptr(ptr) }
//!     }
//! }
//! ```
//!
//! Use [`as_ptr`] and [`as_mut_ptr`] to get the foreign interface object pointer when calling
//! foreign interface functions. Note that a mutable reference can only be obtained through
//! [`Box<T>`], which is constructed using the [`from_owned_mut_ptr`] and [`try_from_owned_mut_ptr`]
//! associated functions, so the smart pointer type is also used to specify the mutability of the
//! foreign type.
//!
//! ```
//! # #[repr(C)]
//! # struct ForeignType(u8);
//! # struct RustBindings;
//! #
//! # extern "C" {
//! #     fn foreign_type_get_value(ptr: *const ForeignType) -> usize;
//! #     fn foreign_type_set_value(ptr: *mut ForeignType, value: usize);
//! # }
//! #
//! use retain_release::ffi::ForeignFunctionInterface;
//!
//! impl RustBindings {
//! #     fn as_ptr(&self) -> *const ForeignType { todo!() }
//! #     fn as_mut_ptr(&mut self) -> *mut ForeignType { todo!() }
//! #
//!     fn value(&self) -> usize {
//!         let ptr = self.as_ptr();
//!         unsafe { foreign_type_get_value(ptr) }
//!     }
//!
//!     fn set_value(&mut self, value: usize) {
//!         let ptr = self.as_mut_ptr();
//!         unsafe { foreign_type_set_value(ptr, value) }
//!     }
//! }
//! ```
//!
//! This trait **should not** be used by crates utilizing the Rust API bindings; it's intended only
//! for crates *implementing* Rust API bindings.
//!
//! ## Memory Management
//!
//! The `Box<T>` and `Arc<T>` smart pointers implemented by this crate are a true zero-cost
//! abstraction and support Apple's immutable/mutable type hierarchies with Rust's reference
//! semantics.
//!
//! The type name [`Box<T>`] signals to the reader that `T` is heap-allocated and that the instance
//! `T` is unique. Similarly, the type name [`Arc<T>`] indicates `T` is heap-allocated and that the
//! instance `T` is shared with other parts of the program.
//!
//! Both types [`Deref`] to `T`, the Rust type implementing the foreign object interface bindings,
//! which is crucial in making the abstraction zero-cost. When the smart pointer is dereferenced by
//! the compiler, it returns the foreign object instance pointer value as a reference to `T`, which
//! can be passed directly through to a foreign interface function.
//!
//! The implementations of [`Box<T>`] and [`Arc<T>`] for reference counted foreign object types are
//! virtually identical, with the primary difference being [`Box<T>`] also implements [`DerefMut`],
//! [`AsMut`], and [`BorrowMut`]. Therefore, [`Box<T>`] should only be used if the object instance a
//! mutable type uniquely owned by the raw pointer. Otherwise, immutable types and objects that may
//! be retained elsewhere should use [`Arc<T>`].
//!
//! [`Arc<T>`]: crate::sync::Arc
//! [`BorrowMut`]: core::borrow::BorrowMut
//! [`Box<T>`]: crate::boxed::Box
//! [`Deref`]: core::ops::Deref
//! [`DerefMut`]: core::ops::DerefMut
//! [`ForeignFunctionInterface`]: crate::ffi::ForeignFunctionInterface
//! [`Raw`]: crate::ffi::ForeignFunctionInterface::Raw
//! [`as_mut_ptr`]: crate::ffi::ForeignFunctionInterface::as_mut_ptr
//! [`as_ptr`]: crate::ffi::ForeignFunctionInterface::as_ptr
//! [`from_unowned_ptr`]: crate::ffi::ForeignFunctionInterface::from_unowned_ptr
//! [`from_owned_mut_ptr`]: crate::ffi::ForeignFunctionInterface::from_owned_mut_ptr
//! [`release`]: crate::ffi::ForeignFunctionInterface::release
//! [`try_from_owned_mut_ptr`]: crate::ffi::ForeignFunctionInterface::try_from_owned_mut_ptr

pub mod ffi;
mod rc;

pub use rc::{boxed, sync};

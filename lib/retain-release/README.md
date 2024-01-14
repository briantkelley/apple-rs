# retain-release

Support for building idiomatic Rust bindings for foreign heap-allocated, reference-counted objects.

## Getting Started

The `ForeignFunctionInterface` trait facilitates conversion between the foreign interface's object
pointer type with reference counting semantics and the native Rust type implementing the bindings.
The trait is straightforward to implement:

* Specify the type of the foreign interface object pointer with `Raw`.
* Implement the retain operation in `from_borrowed_ptr`.
* Implement the release operation in `release`.

```rust
use core::ptr::NonNull;
use retain_release::ffi::ForeignFunctionInterface;
use retain_release::sync::Arc;

impl ForeignFunctionInterface for RustBindings {
    type Raw = ForeignType;

    unsafe fn from_borrowed_ptr(ptr: NonNull<Self::Raw>) -> Arc<Self>
    where
        Self: Sized,
    {
        foreign_type_retain(ptr.as_ptr());
        Arc::from_owned_ptr(ptr)
    }

    unsafe fn release(this: &mut Self) {
        let ptr = this.as_ptr().cast();
        foreign_type_release(ptr);
    }
}
```

Then, when implementing the native Rust type, use the appropriate constructor to wrap the foreign
interface object pointer in a smart pointer to gain automatic retain/release support and the
benefits of Rust's well-defined aliasing rules (assuming the smart pointer safety requirements can
be met).

```rust
use retain_release::ffi::ForeignFunctionInterface;
use retain_release::boxed::Box;
use retain_release::sync::Arc;

impl RustBindings {
    fn current() -> Arc<Self> {
        let ptr = unsafe { foreign_type_get_current() };
        unsafe { Self::try_from_borrowed_ptr(ptr) }.unwrap()
    }

    fn with_context(ctx: *const ()) -> Option<Box<Self>> {
        let ptr = unsafe { foreign_type_create_with_context(ctx) };
        unsafe { Self::try_from_owned_mut_ptr(ptr) }
    }
}
```

Use `as_ptr` and `as_mut_ptr` to get the foreign interface object pointer when calling foreign
interface functions. Note that a mutable reference can only be obtained through `Box<T>`, which
constructed using the [`from_owned_mut_ptr`] and [`try_from_owned_mut_ptr`] associated functions, so
the smart pointer type is also used to specify the mutability of the foreign type.

```rust
use retain_release::ffi::ForeignFunctionInterface;

impl RustBindings {
    fn value(&self) -> usize {
        let ptr = self.as_ptr();
        unsafe { foreign_type_get_value(ptr) }
    }

    fn set_value(&mut self, value: usize) {
        let ptr = self.as_mut_ptr();
        unsafe { foreign_type_set_value(ptr, value) }
    }
}
```

This trait **should not** be used by crates utilizing the Rust API bindings; it's intended only for
crates *implementing* Rust API bindings.

## Memory Management

The `Box<T>` and `Arc<T>` smart pointers implemented by this crate fulfill the following
requirements:

* Are a true zero-cost abstraction.
* Show foreign interface objects are heap-allocated through the type system.
* Combine Rust's mutable references with Apple's immutable/mutable type hierarchy.

The type name `Box<T>` signals to the reader that `T` is heap-allocated and that the instance `T` is
unique. Similarly, the type name `Arc<T>` indicates `T` is heap-allocated and that the instance `T`
is shared with other parts of the program.

Both types `Deref` to `T`, the Rust type implementing the foreign object interface bindings, which
is crucial in making the abstraction zero-cost. When the smart pointer is dereferenced by the
compiler, it returns the foreign object instance pointer value as a reference to `T`, which can be
passed directly through to a foreign interface function.

The implementations of `Box<T>` and `Arc<T>` for reference counted foreign object types are
virtually identical, with the primary difference being `Box<T>` also implements `DerefMut`, `AsMut`,
and `BorrowMut`. Therefore, `Box<T>` should only be used if the object instance a mutable type
uniquely owned by the raw pointer. Otherwise, immutable types and objects that may be retained
elsewhere should use `Arc<T>`.

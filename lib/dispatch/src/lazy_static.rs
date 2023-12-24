use crate::Once;
use core::cell::UnsafeCell;
use core::fmt::{self, Debug, Formatter};
use core::mem::{needs_drop, swap, ManuallyDrop, MaybeUninit};
use core::ops::Deref;

/// A facility to initialize the value of a [`static` item][static-item] at runtime.
///
/// The initialization is thread-safe, though the effects of the initialization function called at
/// runtime are only guaranteed to be visible to an arbitrary thread when the result of the
/// initialization function is accessed through the `LazyStatic` (via the [`Deref`] trait).
///
/// A `LazyStatic<T>`'s initialization function may refer to another `LazyStatic<T>`, but a deadlock
/// will occur if the initialization functions are mutually recursive.
///
/// [static-item]: https://doc.rust-lang.org/reference/items/static-items.html
///
/// # Examples
///
/// ```
/// # use dispatch::LazyStatic;
/// fn expensive_computation_or_io() -> i32 {
///     2 + 2
/// }
///
/// static VALUE: LazyStatic<i32> = LazyStatic::new(expensive_computation_or_io);
///
/// println!("{}", *VALUE); // thread 1
/// println!("{}", *VALUE); // thread 2
/// ```
pub struct LazyStatic<T> {
    sentinel: Once,
    payload: UnsafeCell<Payload<T>>,
    #[cfg(not(feature = "dispatch_once_inline_fastpath"))]
    initialized: core::sync::atomic::AtomicBool,
}

union Payload<T> {
    initialize: MaybeUninit<fn() -> T>,
    value: ManuallyDrop<MaybeUninit<T>>,
}

impl<T> LazyStatic<T> {
    /// Constructs a new `LazyStatic<T>` that will call `initialize` to obtain its value on the
    /// first access (via the [`Deref`] trait).
    #[inline]
    pub const fn new(initialize: fn() -> T) -> Self {
        Self {
            sentinel: Once::new(),
            payload: UnsafeCell::new(Payload {
                initialize: MaybeUninit::new(initialize),
            }),
            #[cfg(not(feature = "dispatch_once_inline_fastpath"))]
            initialized: core::sync::atomic::AtomicBool::new(false),
        }
    }

    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn initialize(&self) {
        self.sentinel
            .dispatch_once_with_context(self, Self::initialize_callback);
    }

    fn initialize_callback(&self) {
        // SAFETY: [`dispatch_once_f`] guarantees that this executes exclusively and only once. The
        // only other mutable reference obtained is in [`<Self as Drop>::drop`], and Rust guarantees
        // that executes exclusively with respect to any other method on the instance.
        let payload = unsafe { &mut *self.payload.get() };

        // SAFETY: `payload.initialize` is initialized in [`Self::new`]. This function, which runs,
        // at most, once, is the only place the value of the union is changed.
        let initialize = unsafe { payload.take_initialize() };

        let value = initialize();
        payload.value = ManuallyDrop::new(MaybeUninit::new(value));

        #[cfg(not(feature = "dispatch_once_inline_fastpath"))]
        self.initialized
            .store(true, core::sync::atomic::Ordering::Release);
    }

    #[cfg(feature = "dispatch_once_inline_fastpath")]
    unsafe fn pending(&mut self) -> bool {
        // SAFETY: Caller asserts proper use of this function.
        unsafe { self.sentinel.pending() }
    }

    #[cfg(not(feature = "dispatch_once_inline_fastpath"))]
    unsafe fn pending(&mut self) -> bool {
        !self.initialized.load(core::sync::atomic::Ordering::Acquire)
    }
}

impl<T> Debug for LazyStatic<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "dispatch_once_inline_fastpath")]
        // SAFETY: This is actually unsafe as it may race with initialization on another thread.
        // But, in the worst case, it'll print an incorrect value of the initialize function
        // pointer, but otherwise there is no undefined behavior that may affect the runtime of the
        // process.
        let pending = unsafe { self.sentinel.pending_unsafe() };

        #[cfg(not(feature = "dispatch_once_inline_fastpath"))]
        // SAFETY: See above SAFETY comment.
        let pending = !self.initialized.load(core::sync::atomic::Ordering::Acquire);

        let (name, value): (&str, &dyn Debug) = if pending {
            // SAFETY: See above SAFETY comment.
            ("initialize", unsafe {
                (&*self.payload.get()).initialize.assume_init_ref()
            })
        } else {
            ("value", &**self)
        };

        f.debug_struct("LazyInit")
            .field("sentinel", &self.sentinel)
            .field(name, value)
            .finish()
    }
}

impl<T> Deref for LazyStatic<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.initialize();

        // SAFETY: [`Self::initialize_callback`] and [`<Self as Drop>::drop`] are the only two
        // places a mutable reference to `self.payload` is obtained. The former is no longer
        // executing and Rust guarantees the latter is not executing, so casting to `&T` is safe.
        let payload = unsafe { &*self.payload.get() };

        // SAFETY: `payload.value` is initialized after the above [`Self::initialize`] call
        // completes.
        unsafe { payload.value.assume_init_ref() }
    }
}

impl<T> Drop for LazyStatic<T> {
    #[inline]
    fn drop(&mut self) {
        if needs_drop::<T>() {
            // Use the const fn as the first, out-most condition to maximize the optimizer's ability
            // to elide dead code. Then, if the type implements `Drop`, check if it's initialized.

            // SAFETY: This check is safe because if the initialization callback is still pending it
            // will not happen (Rust guarantees this method has exclusive access), therefore there
            // is nothing to drop. If the initialization callback has occurred, [`dispatch_once_f`]
            // is still called (via [`Self::initialize`] below) to guarantee this thread has full
            // visibility of the initialization function's effects.
            if unsafe { self.pending() } {
                return;
            }

            self.initialize();

            let payload = self.payload.get_mut();
            // SAFETY: [`Self::initialize`] guarantees `payload.value` is properly initialized.
            drop(unsafe { payload.take_value() });
        }
    }
}

// SAFETY: See below comment on `impl Sync`.
unsafe impl<T> Send for LazyStatic<T> where T: Send {}

// SAFETY: The use of [`UnsafeCell`] inhibits automatic implementation of [`Sync`].
// [`LazyStatic<T>`] is [`Sync`]-safe because `payload.initialize` is properly initialized by
// [`LazyStatic<T>::new`], is then exclusively read in [`dispatch_once_f`], which exclusively writes
// `payload.value`, and, from there, it's safe to get a reference to `payload.value`.
unsafe impl<T> Sync for LazyStatic<T> where T: Sync {}

impl<T> Payload<T> {
    /// Moves the `initialize` field out of `self`, replacing it with [`MaybeUninit::uninit`].
    ///
    /// # Safety
    ///
    /// The caller must guarantee the `initialize` field is properly initialized.
    unsafe fn take_initialize(&mut self) -> fn() -> T {
        let mut initialize = MaybeUninit::uninit();
        // SAFETY: Caller asserts this union has a properly initialized `initialize` field.
        swap(&mut initialize, unsafe { &mut self.initialize });
        // SAFETY: Caller asserts `initialize` is properly initialized.
        unsafe { initialize.assume_init() }
    }

    /// Moves the `value` field out of `self`, replacing it with [`MaybeUninit::uninit`].
    ///
    /// # Safety
    ///
    /// The caller must guarantee the `value` field is properly initialized.
    unsafe fn take_value(&mut self) -> T {
        let mut value = ManuallyDrop::new(MaybeUninit::uninit());
        // SAFETY: Caller asserts this union has a properly initialized `value` field.
        swap(&mut value, unsafe { &mut self.value });

        let value = ManuallyDrop::into_inner(value);
        // SAFETY: Caller asserts `value` is properly initialized.
        unsafe { value.assume_init() }
    }
}

#[cfg(test)]
mod tests {
    use super::LazyStatic;
    use core::sync::atomic::{AtomicIsize, Ordering};

    #[test]
    fn initialize_once() {
        static VALUE: AtomicIsize = AtomicIsize::new(0);

        static LAZY_STATIC: LazyStatic<isize> = LazyStatic::new(|| {
            let magic = 41;

            let _ = VALUE.fetch_add(magic, Ordering::AcqRel);
            magic
        });

        assert_eq!(VALUE.load(Ordering::Acquire), 0);
        assert_eq!(*LAZY_STATIC, 41);
        assert_eq!(*LAZY_STATIC, 41);
        assert_eq!(VALUE.load(Ordering::Acquire), 41);
    }
}

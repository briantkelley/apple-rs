use core::mem::swap;
use core::sync::atomic::AtomicIsize;
use dispatch_sys::{c_void, dispatch_once_f};

/// Provides thread-safe, one-time execution of a function using [`dispatch_once_f`].
///
/// A `Once` must stored as a [`static` item][static-item]. The results of using a `Once` with
/// automatic or dynamic storage are undefined.
///
/// # Examples
///
/// ```
/// # use dispatch::Once;
/// enum Operation { /* ... */ }
/// fn subsystem_initialize() { /* ... */ }
/// fn subsystem_start_operation(op: Operation) { /* ... */ }
///
/// fn start_operation(op: Operation) {
///     static INITIALIZE: Once = Once::new();
///     INITIALIZE.dispatch_once(subsystem_initialize);
///
///     subsystem_start_operation(op);
/// }
/// ```
///
/// [static-item]: https://doc.rust-lang.org/reference/items/static-items.html
#[derive(Debug)]
pub struct Once(AtomicIsize);

struct UserCallback<T> {
    context: Option<T>,
    function: fn(T),
}

impl Once {
    /// Constructs a new sentinel to guarantee, at most, one-time execution of a function.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(AtomicIsize::new(0))
    }

    /// If this is the first function invocation through this sentinel, then `function` is called
    /// synchronously. Otherwise, no operation takes place.
    ///
    /// If `function` requires an argument, use [`Self::dispatch_once_with_context`].
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub fn dispatch_once(&self, function: fn()) {
        self.dispatch_once_with_context(function, |function| function());
    }

    /// If this is the first function invocation through this sentinel, then `function` is called
    /// synchronously. Otherwise, no operation takes place.
    ///
    /// The `context` value is moved into to `function` (or dropped if a function invocation has
    /// already occurred). The value *should* be cheap to construct as it is constructed every
    /// time this method is called, even if it is not used.
    ///
    /// If `function` does not require a `context` parameter, use [`Self::dispatch_once`].
    #[allow(clippy::inline_always)]
    #[inline(always)]
    pub fn dispatch_once_with_context<T>(&self, context: T, function: fn(T)) {
        let mut user_callback = UserCallback {
            context: Some(context),
            function,
        };
        let context: *mut _ = &mut user_callback;

        // SAFETY: The `context` pointer is a function-local and used correctly in
        // [`Self::dispatch_once_with_context_callback`], though this must be verified through code
        // inspection.
        unsafe {
            dispatch_once_f(
                self.0.as_ptr(),
                context.cast(),
                Self::dispatch_once_with_context_callback::<T>,
            );
        }
    }

    extern "C" fn dispatch_once_with_context_callback<T>(context: *mut c_void) {
        let context: *mut UserCallback<T> = context.cast();
        // SAFETY: [`Self::dispatch_once_with_context`] passes a reference to `context` as an opaque
        // pointer. This function is invoked synchronously so the pointer is guaranteed to be valid,
        // but code inspection is required to validate the pointers are of the same type.
        let user_callback = unsafe { &mut *context };

        let mut context = None;
        swap(&mut context, &mut user_callback.context);

        // SAFETY: [`Self::dispatch_once_with_context`] always initializes the `context` field of
        // [`UserCallback<T>`] to [`Some`]. The field is an [`Option<T>`] so this function can take
        // ownership of the user context by swapping the field with [`None`]. Then, the user context
        // is moved into the user callback function, where it will be dropped. If
        // [`Self::dispatch_once_with_context`] is called more than one time, the user context will
        // be dropped at the end of that function because the `context` field will retain its
        // [`Some`] value.
        let context = unsafe { context.unwrap_unchecked() };

        (user_callback.function)(context);
    }

    /// Gets a boolean value indicating whether the first function invocation this sentinel is
    /// pending (i.e., not yet occurred).
    ///
    /// This function requires exclusive access to `self` to safely read the sentinel value to
    /// determine if a function invocation has previously occurred. Without exclusive access, the
    /// return value may be incorrect due to a race with another thread.
    ///
    /// # Safety
    ///
    /// This function is unsafe because a `true` return value it does **not** imply the caller can
    /// elide future calls to [`Self::dispatch_once`] or [`Self::dispatch_once_with_context`]. The
    /// dispatch once function **must** always be called before accessing any resources effected by
    /// the callback function.
    #[cfg(feature = "dispatch_once_inline_fastpath")]
    #[inline]
    pub unsafe fn pending(&mut self) -> bool {
        // Although the [`dispatch_once_f`] wrapper does not perform an atomic read of `predicate`,
        // the implementation of [`dispatch_once_f`] does. The caller is likely checking whether any
        // function invocation occurred to determine what clean up work is necessary. So, use
        // acquire semantics, like the implementation of [`dispatch_once_f`], to ensure the latest
        // value is visible to this thread.
        self.0.load(core::sync::atomic::Ordering::Acquire) == 0
    }

    /// Gets a boolean value indicating whether the first function invocation this sentinel is
    /// pending (i.e., not yet occurred).
    ///
    /// # Safety
    ///
    /// This function cannot be used safely because it can race with other threads.
    #[cfg(feature = "dispatch_once_inline_fastpath")]
    pub(crate) unsafe fn pending_unsafe(&self) -> bool {
        self.0.load(core::sync::atomic::Ordering::Acquire) == 0
    }
}

impl Default for Once {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Once;
    use core::sync::atomic::{AtomicIsize, Ordering};

    #[test]
    fn once() {
        static INITIALIZE: Once = Once::new();
        static VALUE: AtomicIsize = AtomicIsize::new(0);

        fn set_value(value: isize) {
            VALUE.store(value, Ordering::Release);
        }

        assert_eq!(VALUE.load(Ordering::Acquire), 0);

        INITIALIZE.dispatch_once_with_context(13, set_value);
        INITIALIZE.dispatch_once_with_context(41, set_value);

        assert_eq!(VALUE.load(Ordering::Acquire), 13);
    }
}

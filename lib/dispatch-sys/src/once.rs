use crate::dispatch_function_t;
use core::ffi::c_void;

/// A predicate for use with [`dispatch_once_f`].
pub type dispatch_once_t = isize;

mod slowpath {
    use super::{c_void, dispatch_function_t, dispatch_once_t};

    extern "C" {
        // LINT: It's easier to keep this pub than to configure via the feature.
        #[allow(unreachable_pub)]
        pub fn dispatch_once_f(
            predicate: *mut dispatch_once_t,
            context: *mut c_void,
            function: dispatch_function_t,
        );
    }
}

#[allow(clippy::missing_safety_doc)] // same as [`slowpath::dispatch_once_f`]
#[cfg(feature = "dispatch_once_inline_fastpath")]
#[inline(always)]
pub unsafe fn dispatch_once_f(
    predicate: *mut dispatch_once_t,
    context: *mut c_void,
    function: dispatch_function_t,
) {
    use core::sync::atomic::{compiler_fence, Ordering};
    if unsafe { predicate.read() } == !0 {
        compiler_fence(Ordering::SeqCst);
    } else {
        unsafe { slowpath::dispatch_once_f(predicate, context, function) };
    }
}

#[cfg(not(feature = "dispatch_once_inline_fastpath"))]
pub use slowpath::dispatch_once_f;

use crate::sys::activity::{
    _os_activity_initiate_f, _os_activity_label_useraction, OS_ACTIVITY_FLAG_DEFAULT,
};
use crate::sys::trace_base::__dso_handle;
use crate::trace_base::LogString;
use core::ffi::c_void;

pub fn initiate<F>(description: LogString, mut function: F)
where
    F: FnMut(),
{
    // SAFETY: This matches the canonical mechanics of `<os/activity.h>`.
    let dso: *const _ = unsafe { &__dso_handle };
    let context: *mut _ = &mut function;

    // SAFETY: This matches the canonical mechanics of `<os/activity.h>`.
    unsafe {
        _os_activity_initiate_f(
            dso.cast(),
            description,
            OS_ACTIVITY_FLAG_DEFAULT,
            context.cast(),
            initiate_function::<F>,
        );
    }
}

pub fn label_useraction(name: LogString) {
    // SAFETY: This matches the canonical mechanics of `<os/activity.h>`.
    let dso: *const _ = unsafe { &__dso_handle };

    // SAFETY: This matches the canonical mechanics of `<os/activity.h>`.
    unsafe { _os_activity_label_useraction(dso.cast(), name) };
}

extern "C" fn initiate_function<F>(context: *mut c_void)
where
    F: FnMut(),
{
    let function = context.cast::<F>();

    // SAFETY: The type F is guaranteed to be correct via this function's use in `initiate()` and
    // the pointer is safe to dereference as this function is called synchronously.
    let function = unsafe { &mut *function };
    function();
}

#[cfg(test)]
mod tests {
    use crate::{activity_initiate, activity_label_useraction};

    #[test]
    fn activity() {
        let mut called = false;
        activity_initiate!(b"rust-os-activity", || {
            activity_label_useraction!(b"OS Activity User Action from Rust!");
            called = true;
        });
        assert!(called);
    }
}

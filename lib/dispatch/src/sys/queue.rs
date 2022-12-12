use crate::sys::dispatch_function_t;
use core::ffi::c_void;

#[repr(C)]
pub(crate) struct dispatch_queue_s([u8; 0]);

pub(crate) type dispatch_queue_t = *mut dispatch_queue_s;

extern "C" {
    pub(crate) fn dispatch_async_f(
        queue: dispatch_queue_t,
        context: *mut c_void,
        work: dispatch_function_t,
    );

    pub(crate) static _dispatch_main_q: dispatch_queue_s;

    pub(crate) fn dispatch_get_global_queue(identifier: isize, flags: usize) -> dispatch_queue_t;
}

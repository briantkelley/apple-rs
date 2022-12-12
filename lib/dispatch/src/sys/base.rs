use core::ffi::c_void;

pub(crate) type dispatch_function_t = extern "C" fn(_: *mut c_void);

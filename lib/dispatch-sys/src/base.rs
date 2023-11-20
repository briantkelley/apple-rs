pub use core::ffi::c_void;

pub type dispatch_function_t = extern "C" fn(*mut c_void);

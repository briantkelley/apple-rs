#![allow(non_camel_case_types)]

use crate::trace_base::LogString;
use core::ffi::c_void;

pub(crate) type os_activity_flag_t = u32;

pub(crate) const OS_ACTIVITY_FLAG_DEFAULT: os_activity_flag_t = 0;

extern "C" {
    pub(crate) fn _os_activity_initiate_f(
        dso: *const c_void,
        description: LogString,
        flags: os_activity_flag_t,
        context: *mut c_void,
        function: extern "C" fn(*mut c_void),
    );

    pub(crate) fn _os_activity_label_useraction(dso: *const c_void, name: LogString);
}

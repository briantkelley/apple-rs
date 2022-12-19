use core::ffi::c_uint;

pub(crate) const QOS_CLASS_USER_INTERACTIVE: c_uint = 0x21;
pub(crate) const QOS_CLASS_USER_INITIATED: c_uint = 0x19;
pub(crate) const QOS_CLASS_DEFAULT: c_uint = 0x15;
pub(crate) const QOS_CLASS_UTILITY: c_uint = 0x11;
pub(crate) const QOS_CLASS_BACKGROUND: c_uint = 0x09;

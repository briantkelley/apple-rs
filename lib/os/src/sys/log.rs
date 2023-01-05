#![allow(non_camel_case_types)]

use crate::trace_base::LogString;

pub(crate) type os_log_t = *const usize;

extern "C" {
    pub(crate) static _os_log_default: usize;
    pub(crate) static _os_log_disabled: usize;
}

pub(crate) type os_log_type_t = u8;

pub(crate) const OS_LOG_TYPE_DEFAULT: os_log_type_t = 0x00;
pub(crate) const OS_LOG_TYPE_INFO: os_log_type_t = 0x01;
pub(crate) const OS_LOG_TYPE_DEBUG: os_log_type_t = 0x02;
pub(crate) const OS_LOG_TYPE_ERROR: os_log_type_t = 0x10;
pub(crate) const OS_LOG_TYPE_FAULT: os_log_type_t = 0x11;

extern "C" {
    pub(crate) fn os_log_type_enabled(oslog: os_log_t, ty: os_log_type_t) -> bool;

    pub(crate) fn _os_log_impl(
        dso: *const u32,
        log: os_log_t,
        ty: os_log_type_t,
        format: LogString,
        buf: *const u8,
        size: u32,
    );

    pub(crate) fn _os_log_debug_impl(
        dso: *const u32,
        log: os_log_t,
        ty: os_log_type_t,
        format: LogString,
        buf: *const u8,
        size: u32,
    );

    pub(crate) fn _os_log_error_impl(
        dso: *const u32,
        log: os_log_t,
        ty: os_log_type_t,
        format: LogString,
        buf: *const u8,
        size: u32,
    );

    pub(crate) fn _os_log_fault_impl(
        dso: *const u32,
        log: os_log_t,
        ty: os_log_type_t,
        format: LogString,
        buf: *const u8,
        size: u32,
    );
}

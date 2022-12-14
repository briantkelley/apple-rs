use core::ffi::{c_char, c_void};
use core::mem::size_of;
use core::ptr::NonNull;

//
// <objc/objc.h>
//

#[allow(missing_copy_implementations, non_camel_case_types)]
#[repr(C)]
pub struct objc_class {
    // The struct must have at least one field to be FFI safe. `"$OBJC4/runtime/objc-runtime-new.h"`
    // shows this type as inheriting from `objc_object`, so emulate that to create a proper C type.
    isa: [u8; size_of::<usize>()],
}

// SAFETY: All Objective-C pointer types are safe to send across threads.
unsafe impl Send for objc_class {}

// SAFETY: The Objective-C class type is safe to share across threads. The type is opaque and the
// runtime class API is fully thread safe.
unsafe impl Sync for objc_class {}

pub type Class = *mut objc_class;

#[allow(missing_copy_implementations, non_camel_case_types)]
#[repr(C)]
pub struct objc_object([u8; 0]);

#[allow(non_camel_case_types)]
pub type id = *mut objc_object;

#[link(name = "objc")]
extern "C" {
    pub(super) fn object_getClassName(obj: id) -> NonNull<c_char>;
}

//
// <objc/runtime.h>
//

#[link(name = "objc")]
extern "C" {
    pub(super) fn object_getClass(obj: id) -> Class;

    pub(super) fn class_getName(cls: Class) -> NonNull<c_char>;

    pub(super) fn sel_registerName(str: *const c_char) -> NonNull<c_void>;
}

//
// "$OBJC4/runtime/objc-internal.h"
//

// These symbols aren't explicitly defined in any public header, but they are emitted by clang when
// using `-fobjc-arc` making them part of the system ABI (despite the warning in the header).

#[link(name = "objc")]
extern "C" {
    pub(super) fn objc_alloc(cls: Class) -> id;
    pub(super) fn objc_opt_new(cls: Class) -> id;
    pub(super) fn objc_retain(obj: id) -> id;
    pub(super) fn objc_release(obj: id);
}

#[repr(C)]
pub(crate) struct dispatch_object_s([u8; 0]);

pub(crate) type dispatch_object_t = *mut dispatch_object_s;

extern "C" {
    pub(crate) fn dispatch_release(object: dispatch_object_t);
}

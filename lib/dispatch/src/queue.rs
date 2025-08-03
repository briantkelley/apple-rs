extern crate alloc;

use crate::{sys, Object};
use alloc::boxed::Box;
use core::ffi::{c_char, c_void, CStr};
use core::fmt::{self, Debug, Formatter};
use darwin::sys::qos;

#[repr(C)]
pub struct Queue([u8; 0]);

impl Queue {
    #[must_use]
    pub fn global() -> &'static Self {
        Self::global_with_qos(qos::Class::default())
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn global_with_qos(qos: qos::Class) -> &'static Self {
        let qos = Into::<u32>::into(qos) as usize;
        // SAFETY: `qos` is guaranteed to be a valid value.
        #[allow(clippy::cast_possible_wrap)]
        let queue: *mut Self = unsafe { sys::dispatch_get_global_queue(qos as isize, 0) }.cast();
        // SAFETY: The pointer is owned by the system and valid for the lifetime of the process.
        unsafe { queue.as_ref() }.unwrap()
    }

    #[must_use]
    pub fn main() -> &'static Self {
        let queue: *const _ = unsafe { &sys::_dispatch_main_q };
        let queue = queue.cast();
        // SAFETY: The pointer is owned by the system and valid for the lifetime of the process.
        unsafe { &*queue }
    }

    pub fn dispatch_fn_once<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let queue: *const _ = self;
        let queue = queue.cast_mut().cast();
        let context = Box::into_raw(Box::new(f)).cast();
        // SAFETY: The reference is guaranteed to be a valid pointer, the context is guaranteed to
        // be a valid pointer, and Self::call_boxed_fn_once::<F> has the correct signature.
        unsafe { sys::dispatch_async_f(queue, context, Self::call_boxed_fn_once::<F>) }
    }

    extern "C" fn call_boxed_fn_once<F>(context: *mut c_void)
    where
        F: FnOnce() + Send + 'static,
    {
        // SAFETY: This is called by dispatch_fn_once(), which only ever passes a boxed `F` as the
        // context parameter.
        let f = unsafe { Box::<F>::from_raw(context.cast()) };
        (*f)();
    }
}

impl Debug for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        #[link(name = "objc")]
        extern "C" {
            fn object_getClassName(obj: *const c_void) -> *const c_char;
        }

        let obj: *const _ = self;
        let obj = obj.cast();
        // SAFETY: The reference is guaranteed to be a valid pointer.
        let class_name = unsafe { object_getClassName(obj) };
        // SAFETY: object_getClassName always returns a valid C-style string.
        let class_name = unsafe { CStr::from_ptr(class_name) };

        f.write_fmt(format_args!(
            "<{}: {:p}>",
            class_name.to_str().unwrap(),
            obj
        ))
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        let queue: *mut _ = self;
        // SAFETY: The reference is guaranteed to be a valid pointer.
        unsafe { sys::dispatch_release(queue.cast()) };
    }
}

impl Object for Queue {}

#[cfg(test)]
mod tests {
    use super::{qos, Queue};
    use core::sync::atomic::{AtomicBool, Ordering};

    #[test]
    fn test_global_queues() {
        let queue1: *const _ = Queue::main();
        let queue2: *const _ = Queue::global_with_qos(qos::Class::Background);
        let queue3: *const _ = Queue::global_with_qos(qos::Class::Utility);
        let queue4: *const _ = Queue::global_with_qos(qos::Class::Default);
        let queue5: *const _ = Queue::global_with_qos(qos::Class::UserInitiated);
        let queue6: *const _ = Queue::global_with_qos(qos::Class::UserInteractive);

        assert!(!queue1.is_null());
        assert!(!queue2.is_null());
        assert!(!queue3.is_null());
        assert!(!queue4.is_null());
        assert!(!queue5.is_null());
        assert!(!queue6.is_null());

        assert_ne!(queue1, queue2);
        assert_ne!(queue1, queue3);
        assert_ne!(queue1, queue4);
        assert_ne!(queue1, queue5);
        assert_ne!(queue1, queue6);

        assert_ne!(queue2, queue3);
        assert_ne!(queue2, queue4);
        assert_ne!(queue2, queue5);
        assert_ne!(queue2, queue6);

        assert_ne!(queue3, queue4);
        assert_ne!(queue3, queue5);
        assert_ne!(queue3, queue6);

        assert_ne!(queue4, queue5);
        assert_ne!(queue3, queue6);

        assert_ne!(queue5, queue6);
    }

    #[test]
    fn test_dispatch_async() {
        extern "C" {
            fn usleep(microseconds: u32) -> i32;
        }
        static RESULT: AtomicBool = AtomicBool::new(false);

        assert!(!RESULT.load(Ordering::Acquire));
        Queue::global().dispatch_fn_once(|| {
            assert!(!RESULT.load(Ordering::Acquire));
            RESULT.store(true, Ordering::Release);
        });

        // Hopefully 0.25 seconds is enough time to complete.
        // TODO: Use a semaphore with a timeout.
        let _ = unsafe { usleep(250_000) };
        assert!(RESULT.load(Ordering::Acquire));
    }
}

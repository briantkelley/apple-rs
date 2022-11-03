use core::hash::Hash;
use core::ptr::NonNull;
use objc4::{id, msg_send, sel, Box, Object};

/// A protocol that objects adopt to provide functional copies of themselves.
pub trait NSCopying: Eq + Hash + Object {
    /// The object type returned by the copy.
    type Result: Object;

    /// Returns the object returned by `-copyWithZone:`.
    ///
    /// # Panics
    ///
    /// The Swift API notes for this method specify the return type is non-null. Typically the
    /// Objective-C runtime will trap if allocation fails. However, if a subclass overrides this
    /// method and returns `nil`, this binding method will panic.
    #[inline]
    fn copy(&self) -> Box<Self::Result> {
        let obj = msg_send!(id)(self.as_ptr(), sel![COPY]);
        // SAFETY: Objects retured by selectors beginning with ‘copy’ must be released.
        NonNull::new(obj)
            .map(|obj| unsafe { Box::with_transfer(obj) })
            .unwrap()
    }
}

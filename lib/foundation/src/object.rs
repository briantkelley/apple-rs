use objc4::{msg_send, Box, Object};

pub trait NSCopying: Object {
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
        msg_send!((box_transfer nonnull id)[self, copy])
    }
}

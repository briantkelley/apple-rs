use crate::sys::{objc_alloc, objc_opt_new};
use crate::{id, image_info, objc_class, objc_object, Box, Object, Rc};
use core::ptr::NonNull;

extern_class!(objc, kind = dylib, pub NSObject);

selector!(HASH = "hash");
selector!(IS_EQUAL_ = "isEqual:");
selector!(IS_PROXY = "isProxy");
selector!(SUPERCLASS = "superclass");

image_info!();

/// The group of methods that are fundamental to all Objective-C objects.
pub trait NSObjectProtocol: Object {
    /// Returns a Boolean value that indicates whether the receiver and a given object are equal.
    #[inline]
    fn is_equal(&self, object: &impl Object) -> bool {
        msg_send!(bool, id)(self.as_ptr(), sel![IS_EQUAL_], object.as_ptr())
    }

    /// Returns an integer that can be used as a table address in a hash table structure.
    #[inline]
    fn hash(&self) -> usize {
        msg_send!(usize)(self.as_ptr(), sel![HASH])
    }

    /// Returns the class object for the receiver’s superclass
    #[inline]
    fn superclass(&self) -> Option<NonNull<objc_class>> {
        let cls = msg_send!(*mut objc_class)(self.as_ptr(), sel![SUPERCLASS]);
        NonNull::new(cls)
    }

    /// Returns a Boolean value that indicates whether the receiver does not descend from
    /// [`NSObjectInterface`].
    #[inline]
    fn is_proxy(&self) -> bool {
        msg_send!(bool)(self.as_ptr(), sel![IS_PROXY])
    }
}

/// The root class of most Objective-C class hierarchies, from which subclasses inherit a basic
/// interface to the runtime system and the ability to behave as Objective-C objects.
pub trait NSObjectInterface: NSObjectProtocol {
    /// Returns a new instance of the class.
    ///
    /// After calling this function, the caller is responsible for ensuring the object pointer is
    /// released, which is typically handled by `Box`-ing the return value of an `-init` method.
    ///
    /// # Panics
    ///
    /// The Swift API notes for this method specify the return type is non-null. Typically the
    /// Objective-C runtime will trap if allocation fails. However, if a subclass overrides this
    /// method and returns `nil`, this binding method will panic.
    #[must_use]
    fn alloc() -> NonNull<objc_object> {
        let cls: *const _ = Self::class_type();
        // SAFETY: Rust code never reads through the reference that is passed as a pointer to the
        // Objective-C runtime, which is the owner of the data structure.
        NonNull::new(unsafe { objc_alloc(cls as *mut _) }).unwrap()
    }

    /// A new instance of the receiver.
    ///
    /// This function is a combination of `+alloc` and `-init`. Like `+alloc`, it initializes the
    /// `isa` instance variable of the new object so it points to the class data structure. It then
    /// invokes the `-init` method to complete the initialization process.
    ///
    /// # Panics
    ///
    /// The Swift API notes for this method specify the return type is non-null. Typically the
    /// Objective-C runtime will trap if allocation fails. However, if a subclass overrides `+alloc`
    /// or `-init` and returns `nil`, this binding method will panic.
    #[must_use]
    fn new() -> Box<Self> {
        let cls: *const _ = Self::class_type();
        // SAFETY: The trait implementation guarantees `cls` is a valid Objective-C class.
        let obj = NonNull::new(unsafe { objc_opt_new(cls as *mut _) }).unwrap();
        // SAFETY: Objects retured by selectors beginning with ‘new’ must be released.
        unsafe { Box::new_transfer(obj) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class() {
        assert_eq!(NSObject::new().class(), unsafe { &NSObjectClass });
    }

    #[test]
    fn test_hash() {
        let o = NSObject::new();
        assert_eq!(o.hash(), o.as_ptr() as usize);
    }

    #[test]
    fn test_is_equal() {
        let a = NSObject::new();
        let b = NSObject::new();

        assert!(a.is_equal(&*a));
        assert!(b.is_equal(&*b));

        assert!(!a.is_equal(&*b));
        assert!(!b.is_equal(&*a));
    }

    #[test]
    fn test_is_proxy() {
        assert!(!NSObject::new().is_proxy());
    }

    #[test]
    fn test_superclass() {
        assert!(matches!(NSObject::new().superclass(), None));
    }
}

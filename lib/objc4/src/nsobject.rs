use crate::sys::{objc_alloc, objc_opt_new};
use crate::{id, objc_class, objc_object, Box, Object};
use core::ptr::NonNull;

extern_class!(objc, kind = dylib, pub NSObject 'cls);

/// The group of methods that are fundamental to all Objective-C objects.
pub trait NSObjectProtocol: Object {
    /// Returns a Boolean value that indicates whether the receiver and a given object are equal.
    #[inline]
    fn is_equal(&self, object: &dyn Object) -> bool {
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
pub trait NSObjectInterface: NSObjectProtocol {}

/// The root meta class of most Objective-C class hierarchies, from which subclasses inherit a basic
/// interface to the runtime system and the ability to behave as Objective-C objects.
pub trait NSObjectClassInterface {
    /// The concrete type that implements class instance interface.
    type Instance: NSObjectInterface;

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
    fn alloc(&self) -> NonNull<objc_object> {
        let cls: *const _ = self;
        // SAFETY: `self` is a reference so it is guaranteed to be a valid pointer to an Objective-C
        // meta class object.
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
    #[allow(clippy::wrong_self_convention)]
    #[must_use]
    fn new(&self) -> Box<Self::Instance> {
        let cls: *const _ = self;
        // SAFETY: The trait implementation guarantees `cls` is a valid Objective-C class.
        let obj = NonNull::new(unsafe { objc_opt_new(cls as *mut _) }).unwrap();
        // SAFETY: Objects retured by selectors beginning with ‘new’ must be released.
        unsafe { Box::with_transfer(obj) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class() {
        let lhs: *const _ = NSObjectClass.new().class();
        let rhs: *const _ = NSObjectClass;
        assert_eq!(lhs, rhs.cast());
    }

    #[test]
    fn test_hash() {
        let o = NSObjectClass.new();
        assert_eq!(o.hash(), o.as_ptr() as usize);
    }

    #[test]
    fn test_is_equal() {
        let a = NSObjectClass.new();
        let b = NSObjectClass.new();

        assert!(a.is_equal(&*a));
        assert!(b.is_equal(&*b));

        assert!(!a.is_equal(&*b));
        assert!(!b.is_equal(&*a));
    }

    #[test]
    fn test_is_proxy() {
        assert!(!NSObjectClass.new().is_proxy());
    }

    #[test]
    fn test_superclass() {
        assert!(matches!(NSObjectClass.new().superclass(), None));
    }
}

use crate::NSCopying;
use objc4::{
    extern_class, id, msg_send, sel, Box, NSObjectClassInterface, NSObjectInterface, Object,
};

extern_class!(Foundation, pub NSDictionary<Key, Value>, NSObject 'cls; Key: NSCopying, Value: Object);

/// A static collection of objects associated with unique keys.
#[allow(clippy::len_without_is_empty)]
pub trait NSDictionaryInterface: NSObjectInterface + NSCopying {
    /// The object type that identifies values in the dictionary.
    type Key: NSCopying;

    /// The object type of the value indexed by `Key`.
    type Value: Object;

    /// Returns the value associated with a given key.
    fn get(&self, k: &Self::Key) -> Option<&Self::Value> {
        let obj =
            msg_send!(id, id)(self.as_ptr(), sel![OBJECTFORKEY_], k.as_ptr()) as *const Self::Value;
        // SAFETY: If the dictionary contains the value, the pointer is guaranteed to be valid.
        unsafe { obj.as_ref() }
    }

    /// The number of entries in the dictionary.
    fn len(&self) -> usize {
        msg_send!(usize)(self.as_ptr(), sel![COUNT])
    }
}

extern_class!(Foundation, pub NSMutableDictionary, NSDictionary<Key, Value>, NSObject 'cls; Key: NSCopying, Value: Object);

impl<Key, Value> NSCopying for NSDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
{
    type Result = Self;
}

/// A dynamic collection of objects associated with unique keys.
pub trait NSMutableDictionaryInterface: NSDictionaryInterface {
    /// Removes a given key and its associated value from the dictionary.
    fn remove(&mut self, k: &Self::Key) {
        msg_send!((), id)(self.as_ptr(), sel![REMOVEOBJECTFORKEY_], k.as_ptr());
    }

    /// Adds a given key-value pair to the dictionary.
    fn set(&mut self, k: &Self::Key, v: Box<Self::Value>) {
        msg_send!((), id, id)(
            self.as_ptr(),
            sel![SETOBJECT_FORKEY_],
            v.as_ptr(),
            k.as_ptr(),
        );
    }
}

impl<Key, Value> NSCopying for NSMutableDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
{
    type Result = NSDictionary<Key, Value>;
}

impl<Key, Value> NSMutableDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
{
    /// A new instance of the receiver.
    #[must_use]
    pub fn new() -> Box<Self> {
        let obj = NSMutableDictionaryClass.new();
        unsafe { obj.transmute_unchecked::<Self>() }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{NSString, NSStringClass, NSStringClassInterface, NSStringInterface};

    #[test]
    fn test_add_get_remove() {
        let mut dict = NSMutableDictionary::<NSString, NSString>::new();

        let key = NSStringClass.from_str("key");
        let value = NSStringClass.from_str("value");

        dict.set(&key, value);
        assert_eq!(dict.len(), 1);

        let value = dict.get(&key).unwrap();
        assert_eq!(
            unsafe { value.as_c_str() }.unwrap().to_str().unwrap(),
            "value"
        );

        dict.remove(&key);
        assert_eq!(dict.len(), 0);
    }
}

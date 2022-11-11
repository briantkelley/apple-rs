use crate::NSCopying;
use core::hash::{Hash, Hasher};
use objc4::{
    extern_class, id, msg_send, Box, NSObjectClassInterface, NSObjectInterface, NSObjectProtocol,
    Object,
};

extern_class!(Foundation, pub NSDictionary<Key, Value>, NSObject 'cls; Key: NSCopying, Value: Object; -PartialEq);

#[allow(clippy::len_without_is_empty)]
pub trait NSDictionaryInterface:
    NSObjectInterface + NSCopying<Result = NSDictionary<Self::Key, Self::Value>>
{
    /// The type that identifies values in the dictionary.
    type Key: NSCopying;

    /// The type of the value indexed by `Key`.
    type Value: Object;

    #[inline]
    fn get(&self, k: &Self::Key) -> Option<&Self::Value> {
        let obj = msg_send!((id)[self, objectForKey:(id)k]).cast::<Self::Value>();
        // SAFETY: If the dictionary contains the value, the pointer is guaranteed to be valid.
        unsafe { obj.as_ref() }
    }

    #[inline]
    fn len(&self) -> usize {
        msg_send!((usize)[self, count])
    }

    #[inline]
    fn is_equal_to_dictionary(
        &self,
        other: &impl NSDictionaryInterface<Key = Self::Key, Value = Self::Value>,
    ) -> bool {
        msg_send!((bool)[self, isEqualToDictionary:(id)other])
    }
}

impl<Key, Value> Hash for NSDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(NSObjectProtocol::hash(self));
    }
}

impl<Key, Value> NSCopying for NSDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
{
    type Result = Self;
}

impl<T, Key, Value> PartialEq<T> for NSDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
    T: NSDictionaryInterface<Key = Key, Value = Value>,
{
    fn eq(&self, other: &T) -> bool {
        self.is_equal_to_dictionary(other)
    }
}

extern_class!(Foundation, pub NSMutableDictionary, NSDictionary<Key, Value>, NSObject 'cls; Key: NSCopying, Value: Object; -PartialEq);

pub trait NSMutableDictionaryInterface: NSDictionaryInterface {
    #[inline]
    fn remove(&mut self, k: &Self::Key) {
        msg_send!([self, removeObjectForKey:(id)k]);
    }

    #[inline]
    fn set(&mut self, k: &Self::Key, v: Box<Self::Value>) {
        msg_send!([self, setObject:(id)&*v forKey:(id)k]);
    }
}

impl<Key, Value> NSMutableDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
{
    #[must_use]
    pub fn new() -> Box<Self> {
        let obj = NSMutableDictionaryClass.new();
        unsafe { obj.transmute_unchecked::<Self>() }
    }
}

impl<Key, Value> Hash for NSMutableDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(NSObjectProtocol::hash(self));
    }
}

impl<Key, Value> NSCopying for NSMutableDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
{
    type Result = NSDictionary<Key, Value>;
}

impl<T, Key, Value> PartialEq<T> for NSMutableDictionary<Key, Value>
where
    Key: NSCopying,
    Value: Object,
    T: NSDictionaryInterface<Key = Key, Value = Value>,
{
    fn eq(&self, other: &T) -> bool {
        self.is_equal_to_dictionary(other)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        string_literal, NSNumberClass, NSNumberClassInterface, NSString, NSStringClass,
        NSStringClassInterface, NSStringInterface,
    };
    use objc4::NSObject;

    #[test]
    fn test_add_get_remove() {
        string_literal!(static KEY: NSString = "key");
        string_literal!(static VALUE: NSString = "value");

        let mut dict = NSMutableDictionary::<NSString, NSString>::new();

        dict.set(KEY, VALUE.copy());
        assert_eq!(dict.len(), 1);

        let value = dict.get(KEY).unwrap();
        assert_eq!(
            unsafe { value.as_c_str() }.unwrap().to_str().unwrap(),
            "value"
        );

        dict.remove(KEY);
        assert_eq!(dict.len(), 0);
    }

    #[test]
    fn test_equal() {
        let string = NSStringClass.from_str("string");
        let number = NSStringClass.from_str("number");

        let mut dict1 = NSMutableDictionary::<NSString, NSObject>::new();
        dict1.set(&string, NSStringClass.from_str("value").upcast());
        dict1.set(&number, NSNumberClass.from_i32(0xf00d).upcast());

        let dict2 = dict1.copy();

        assert_eq!(dict1, dict2);
        assert_eq!(dict2, dict1);
    }

    #[test]
    fn test_upcast() {
        let string = NSStringClass.from_str("string");
        let number = NSStringClass.from_str("number");

        let mut dict_mut = NSMutableDictionary::<NSString, NSObject>::new();

        dict_mut.set(&string, NSStringClass.from_str("value").upcast());
        dict_mut.set(&number, NSNumberClass.from_i32(0xf00d).upcast());
        assert_eq!(dict_mut.len(), 2);

        let dict = dict_mut.upcast::<NSDictionary<NSString, NSObject>>();
        assert_eq!(dict.len(), 2);
        assert!(dict
            .get(&string)
            .unwrap()
            .is_equal(&*NSStringClass.from_str("value").upcast::<NSObject>()));
        assert!(dict
            .get(&number)
            .unwrap()
            .is_equal(&*NSNumberClass.from_i32(0xf00d).upcast::<NSObject>()));

        let object = dict.upcast::<NSObject>();
        assert!(matches!(
            object.class_name().to_str().unwrap().find("Dictionary"),
            Some(_)
        ));
    }
}

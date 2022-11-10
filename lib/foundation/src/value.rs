use crate::NSCopying;
use core::hash::{Hash, Hasher};
use core::ptr::NonNull;
use objc4::{
    extern_class, id, msg_send, Box, NSObjectClassInterface, NSObjectInterface, NSObjectProtocol,
};

extern_class!(Foundation, pub NSValue, NSObject 'cls);

/// A simple container for a single C or Objective-C data item.
pub trait NSValueInterface: NSObjectInterface + NSCopying<Result = Self> {}

impl Hash for NSValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(NSObjectProtocol::hash(self));
    }
}

impl NSCopying for NSValue {
    type Result = Self;
}

extern_class!(Foundation, pub NSNumber 'cls, NSValue, NSObject 'cls; -PartialEq);

/// An object wrapper for primitive scalar numeric values.
pub trait NSNumberClassInterface: NSObjectClassInterface {
    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `i8`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_i8(&self, value: i8) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithChar:(i8)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `u8`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_u8(&self, value: u8) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithUnsignedChar:(u8)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `i16`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_i16(&self, value: i16) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithShort:(i16)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `u16`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_u16(&self, value: u16) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithUnsignedShort:(u16)value]))
                .unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `i32`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_i32(&self, value: i32) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithInt:(i32)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `u32`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_u32(&self, value: u32) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithUnsignedInt:(u32)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `i64`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_i64(&self, value: i64) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithLongLong:(i64)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `u64`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_u64(&self, value: u64) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithUnsignedLongLong:(u64)value]))
                .unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `f32`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_f32(&self, value: f32) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithFloat:(f32)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `f64`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_f64(&self, value: f64) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithDouble:(f64)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `bool`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_bool(&self, value: bool) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithBool:(bool)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `isize`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_isize(&self, value: isize) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithInteger:(isize)value])).unwrap(),
        )
    }

    /// Creates and returns an `NSNumber` object containing a given value, treating it as a `usize`.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_usize(&self, value: usize) -> Box<Self::Instance> {
        Box::with_retained(
            NonNull::new(msg_send!((id)[self.as_ptr(), numberWithUnsignedInteger:(usize)value]))
                .unwrap(),
        )
    }
}

/// An object wrapper for primitive scalar numeric values.
pub trait NSNumberInterface: NSValueInterface {
    /// The number object's value expressed as a `i8`.
    #[inline]
    fn as_i8(&self) -> i8 {
        msg_send!((i8)[self.as_ptr(), charValue])
    }

    /// The number object's value expressed as a `u8`.
    #[inline]
    fn as_u8(&self) -> u8 {
        msg_send!((u8)[self.as_ptr(), unsignedCharValue])
    }

    /// The number object's value expressed as a `i16`.
    #[inline]
    fn as_i16(&self) -> i16 {
        msg_send!((i16)[self.as_ptr(), shortValue])
    }

    /// The number object's value expressed as a `u16`.
    #[inline]
    fn as_u16(&self) -> u16 {
        msg_send!((u16)[self.as_ptr(), unsignedShortValue])
    }

    /// The number object's value expressed as a `i32`.
    #[inline]
    fn as_i32(&self) -> i32 {
        msg_send!((i32)[self.as_ptr(), intValue])
    }

    /// The number object's value expressed as a `u32`.
    #[inline]
    fn as_u32(&self) -> u32 {
        msg_send!((u32)[self.as_ptr(), unsignedIntValue])
    }

    /// The number object's value expressed as a `i64`.
    #[inline]
    fn as_i64(&self) -> i64 {
        msg_send!((i64)[self.as_ptr(), longLongValue])
    }

    /// The number object's value expressed as a `u64`.
    #[inline]
    fn as_u64(&self) -> u64 {
        msg_send!((u64)[self.as_ptr(), unsignedLongLongValue])
    }

    /// The number object's value expressed as a `f32`.
    #[inline]
    fn as_f32(&self) -> f32 {
        msg_send!((f32)[self.as_ptr(), floatValue])
    }

    /// The number object's value expressed as a `f64`.
    #[inline]
    fn as_f64(&self) -> f64 {
        msg_send!((f64)[self.as_ptr(), doubleValue])
    }

    /// The number object's value expressed as a `bool`.
    #[inline]
    fn as_bool(&self) -> bool {
        msg_send!((bool)[self.as_ptr(), boolValue])
    }

    /// The number object's value expressed as a `isize`.
    #[inline]
    fn as_isize(&self) -> isize {
        msg_send!((isize)[self.as_ptr(), integerValue])
    }

    /// The number object's value expressed as a `usize`.
    #[inline]
    fn as_usize(&self) -> usize {
        msg_send!((usize)[self.as_ptr(), unsignedIntegerValue])
    }

    /// Returns a Boolean value that indicates whether the number object’s value and a given number
    /// are equal.
    #[inline]
    fn is_equal_to_number(&self, other: &impl NSNumberInterface) -> bool {
        msg_send!((bool)[self.as_ptr(), isEqualToNumber:(id)other.as_ptr()])
    }
}

impl Hash for NSNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(NSObjectProtocol::hash(self));
    }
}

impl NSCopying for NSNumber {
    type Result = Self;
}

impl<T> PartialEq<T> for NSNumber
where
    T: NSNumberInterface,
{
    fn eq(&self, other: &T) -> bool {
        self.is_equal_to_number(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_and_one() {
        struct Value(bool, i8, u8, f32);

        let zero = Value(false, 0, 0, 0.0);
        let one = Value(true, 1, 1, 1.0);

        for value in [zero, one] {
            let numbers = [
                NSNumberClass.from_i8(value.1),
                NSNumberClass.from_u8(value.2),
                NSNumberClass.from_i16(i16::from(value.1)),
                NSNumberClass.from_u16(u16::from(value.2)),
                NSNumberClass.from_i32(i32::from(value.1)),
                NSNumberClass.from_u32(u32::from(value.2)),
                NSNumberClass.from_i64(i64::from(value.1)),
                NSNumberClass.from_u64(u64::from(value.2)),
                NSNumberClass.from_f32(value.3),
                NSNumberClass.from_f64(f64::from(value.3)),
                NSNumberClass.from_bool(value.0),
                NSNumberClass.from_isize(isize::from(value.1)),
                NSNumberClass.from_usize(usize::from(value.2)),
            ];

            for number in &numbers {
                assert_eq!(number.as_i8(), value.1);
                assert_eq!(number.as_u8(), value.2);
                assert_eq!(number.as_i16(), i16::from(value.1));
                assert_eq!(number.as_u16(), u16::from(value.2));
                assert_eq!(number.as_i32(), i32::from(value.1));
                assert_eq!(number.as_u32(), u32::from(value.2));
                assert_eq!(number.as_i64(), i64::from(value.1));
                assert_eq!(number.as_u64(), u64::from(value.2));
                #[allow(clippy::float_cmp)]
                {
                    assert_eq!(number.as_f32(), value.3);
                    assert_eq!(number.as_f64(), f64::from(value.3));
                }
                assert_eq!(number.as_bool(), value.0);
                assert_eq!(number.as_isize(), isize::from(value.1));
                assert_eq!(number.as_usize(), usize::from(value.2));

                for number2 in &numbers {
                    assert_eq!(number, number2);
                }
            }
        }
    }
}

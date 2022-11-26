use crate::{NSComparisonResult, NSCopying};
use core::cmp::Ordering;
use objc4::{extern_class, id, msg_send, Box, NSObjectClassInterface, NSObjectInterface};

extern_class!(Foundation, pub NSValue, NSObject 'cls);

pub trait NSValueInterface: NSObjectInterface + NSCopying<Result = Self> {}

impl NSCopying for NSValue {
    type Result = Self;
}

extern_class!(Foundation, pub NSNumber 'cls, NSValue, NSObject 'cls; -PartialEq);

pub trait NSNumberClassInterface: NSObjectClassInterface {
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_i8(&self, value: i8) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithChar:(i8)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_u8(&self, value: u8) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithUnsignedChar:(u8)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_i16(&self, value: i16) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithShort:(i16)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_u16(&self, value: u16) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithUnsignedShort:(u16)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_i32(&self, value: i32) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithInt:(i32)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_u32(&self, value: u32) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithUnsignedInt:(u32)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_i64(&self, value: i64) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithLongLong:(i64)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_u64(&self, value: u64) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithUnsignedLongLong:(u64)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_f32(&self, value: f32) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithFloat:(f32)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_f64(&self, value: f64) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithDouble:(f64)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_bool(&self, value: bool) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithBool:(bool)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_isize(&self, value: isize) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithInteger:(isize)value])
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_usize(&self, value: usize) -> Box<Self::Instance> {
        msg_send!((box_retain nonnull id)[self, numberWithUnsignedInteger:(usize)value])
    }
}

pub trait NSNumberInterface: NSValueInterface + Ord + PartialOrd {
    #[inline]
    fn as_i8(&self) -> i8 {
        msg_send!((i8)[self, charValue])
    }

    #[inline]
    fn as_u8(&self) -> u8 {
        msg_send!((u8)[self, unsignedCharValue])
    }

    #[inline]
    fn as_i16(&self) -> i16 {
        msg_send!((i16)[self, shortValue])
    }

    #[inline]
    fn as_u16(&self) -> u16 {
        msg_send!((u16)[self, unsignedShortValue])
    }

    #[inline]
    fn as_i32(&self) -> i32 {
        msg_send!((i32)[self, intValue])
    }

    #[inline]
    fn as_u32(&self) -> u32 {
        msg_send!((u32)[self, unsignedIntValue])
    }

    #[inline]
    fn as_i64(&self) -> i64 {
        msg_send!((i64)[self, longLongValue])
    }

    #[inline]
    fn as_u64(&self) -> u64 {
        msg_send!((u64)[self, unsignedLongLongValue])
    }

    #[inline]
    fn as_f32(&self) -> f32 {
        msg_send!((f32)[self, floatValue])
    }

    #[inline]
    fn as_f64(&self) -> f64 {
        msg_send!((f64)[self, doubleValue])
    }

    #[inline]
    fn as_bool(&self) -> bool {
        msg_send!((bool)[self, boolValue])
    }

    #[inline]
    fn as_isize(&self) -> isize {
        msg_send!((isize)[self, integerValue])
    }

    #[inline]
    fn as_usize(&self) -> usize {
        msg_send!((usize)[self, unsignedIntegerValue])
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
        msg_send!((bool)[self, isEqualToNumber:(id)other])
    }
}

impl Ord for NSNumber {
    fn cmp(&self, other: &Self) -> Ordering {
        msg_send!((NSComparisonResult)[self, compare:(id)other]).into()
    }
}

impl<T> PartialOrd<T> for NSNumber
where
    T: NSNumberInterface,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(msg_send!((NSComparisonResult)[self, compare:(id)other]).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare() {
        let one = NSNumberClass.from_i32(1);
        let two = NSNumberClass.from_i32(2);

        assert!(matches!(one.partial_cmp(&*two), Some(Ordering::Less)));
        assert!(matches!(one.cmp(&*two), Ordering::Less));
        assert!(matches!(one.partial_cmp(&*one), Some(Ordering::Equal)));
        assert!(matches!(one.cmp(&*one), Ordering::Equal));
        assert!(matches!(two.partial_cmp(&*one), Some(Ordering::Greater)));
        assert!(matches!(two.cmp(&*one), Ordering::Greater));
    }

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
                    assert!(matches!(
                        number.partial_cmp(&**number2),
                        Some(Ordering::Equal)
                    ));
                    assert!(matches!(number.cmp(number2), Ordering::Equal));
                }
            }
        }
    }
}

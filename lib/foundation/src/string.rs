extern crate alloc;

use crate::{NSComparisonResult, NSCopying};
use alloc::string::{String, ToString};
use core::cmp::Ordering;
use core::ffi::{c_char, CStr};
use core::fmt::{self, Debug, Formatter};
use objc4::{
    extern_class, id, msg_send, objc_object, Box, NSObjectClassInterface, NSObjectInterface,
};

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum NSStringEncoding {
    ASCII = 1,
    UTF8 = 4,
    UTF16 = 10,
    UTF16BigEndian = 0x9000_0100,
    UTF16LittleEndian = 0x9400_0100,
    UTF32 = 0x8c00_0100,
    UTF32BigEndian = 0x9800_0100,
    UTF32LittleEndian = 0x9c00_0100,
}

extern_class!(Foundation, pub NSString 'cls, NSObject 'cls; -PartialEq);

pub trait NSStringClassInterface: NSObjectClassInterface {
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_bytes(&self, buf: &[u8], encoding: NSStringEncoding) -> Option<Box<Self::Instance>> {
        msg_send!((box_transfer nullable id)[self.alloc().as_ptr(), initWithBytes:(*const u8)buf.as_ptr()
                                                                           length:(usize)buf.len()
                                                                         encoding:(usize)encoding as usize])
    }

    /// Returns an `NSString` object initialized by copying the characters from a given slice of
    /// UTF8-encoded bytes.
    ///
    /// # Panics
    ///
    /// Panics if `s` is not a well-formed UTF-8 string slice.
    #[allow(clippy::wrong_self_convention)]
    #[must_use]
    fn from_str(&self, s: &str) -> Box<Self::Instance> {
        self.from_bytes(s.as_bytes(), NSStringEncoding::UTF8)
            .unwrap()
    }
}

pub trait NSStringInterface:
    NSObjectInterface + NSCopying<Result = NSString> + Ord + PartialOrd + ToString
{
    #[inline]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    fn len(&self) -> usize {
        msg_send!((usize)[self, length])
    }

    /// A null-terminated UTF-8 representation of the string.
    ///
    /// # Safety
    ///
    /// This method is unsafe because the returned reference is only valid through the current
    /// autorelease scope, which is not well-defined.
    #[inline]
    unsafe fn to_c_str(&self) -> Option<&CStr> {
        let str = msg_send!((*const c_char)[self, UTF8String]);
        if str.is_null() {
            None
        } else {
            // SAFETY: str is guaranteed to be a valid C string pointer.
            Some(unsafe { CStr::from_ptr(str) })
        }
    }
}

impl NSCopying for NSString {
    type Result = Self;
}

impl Ord for NSString {
    fn cmp(&self, other: &Self) -> Ordering {
        msg_send!((NSComparisonResult)[self, compare:(id)other]).into()
    }
}

impl<T> PartialEq<T> for NSString
where
    T: NSStringInterface,
{
    fn eq(&self, other: &T) -> bool {
        msg_send!((bool)[self, isEqualToString:(id)other])
    }
}

impl<T> PartialOrd<T> for NSString
where
    T: NSStringInterface,
{
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(msg_send!((NSComparisonResult)[self, compare:(id)other]).into())
    }
}

impl ToString for NSString {
    fn to_string(&self) -> String {
        // SAFETY: The CStr is copied into the String before the autorelease scope can change.
        unsafe { self.to_c_str() }
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

// `_ConstantString` is an "Object" special case. It's statically allocated and its fields are not
// Objective-C ivars. Instead of accomodating this outlier scenario in `extern_class()`, manually
// expand and tweak the relevant parts of the macro.

extern "C" {
    #[doc(hidden)]
    pub static __CFConstantStringClassReference: usize;
}

#[allow(clippy::module_name_repetitions, missing_copy_implementations)]
#[doc(hidden)]
#[repr(C)]
pub struct __CFConstantString {
    pub _isa: &'static usize,
    pub _flags: u32,
    pub _str: *const u8,
    pub _length: usize,
}

unsafe impl Sync for __CFConstantString {}

impl Debug for __CFConstantString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let obj: *const _ = self;
        let obj: *const objc_object = obj.cast();
        // SAFETY: The reference is guaranteed to be a valid pointer.
        unsafe { &*obj }.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string_literal;
    use crate::tests::AddHasher;
    use core::hash::Hash;

    string_literal!(static HELLO_WORLD: NSString = "Hello, World!");

    #[test]
    fn test_copy() {
        let orig = HELLO_WORLD;
        let copy = orig.copy();

        assert_eq!(*orig, *copy);
        assert_eq!(orig, &copy);

        assert_eq!(*copy, *orig);
        assert_eq!(&copy, orig);
    }

    #[test]
    fn test_compare() {
        string_literal!(static A: NSString = "A");
        string_literal!(static B: NSString = "B");

        assert!(matches!(A.partial_cmp(B), Some(Ordering::Less)));
        assert!(matches!(A.cmp(B), Ordering::Less));
        assert!(matches!(A.partial_cmp(A), Some(Ordering::Equal)));
        assert!(matches!(A.cmp(A), Ordering::Equal));
        assert!(matches!(B.partial_cmp(A), Some(Ordering::Greater)));
        assert!(matches!(B.cmp(A), Ordering::Greater));
    }

    #[test]
    fn test_conversion() {
        let str = "Hello, World!";
        let string = NSStringClass.from_str(str);

        assert_eq!(&string.to_string(), str);
    }

    #[test]
    fn test_equality() {
        let data = HELLO_WORLD;
        let heap = NSStringClass.from_str("Hello, World!");

        assert_eq!(*data, *heap);
        assert_eq!(*heap, *data);

        let mut data_hasher = AddHasher(0);
        let mut heap_hasher = AddHasher(0);
        data.hash(&mut data_hasher);
        heap.hash(&mut heap_hasher);
        assert_eq!(data_hasher.0, heap_hasher.0);
    }

    #[test]
    fn test_len() {
        string_literal!(static EMPTY: NSString = "");

        assert_eq!(EMPTY.len(), 0);
        assert_eq!(HELLO_WORLD.len(), 13);

        assert!(EMPTY.is_empty());
        assert!(!HELLO_WORLD.is_empty());
    }

    #[test]
    fn test_literal() {
        let str = &HELLO_WORLD;

        assert_eq!(str.len(), 13);
        assert_eq!(&str.to_string(), "Hello, World!");
    }
}

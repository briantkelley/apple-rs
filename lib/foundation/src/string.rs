use crate::NSCopying;
use core::ffi::{c_char, CStr};
use core::fmt::{self, Debug, Formatter};
use core::ptr::NonNull;
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
        let obj = msg_send!((id)[self.alloc().as_ptr(), initWithBytes:(*const u8)buf.as_ptr()
                                                                                 length:(usize)buf.len()
                                                                               encoding:(usize)encoding as usize]);
        // SAFETY: Objects retured by selectors beginning with ‘init’ consume their argument
        // (selectors beginning with ‘alloc’ must also be released) and must be released.
        NonNull::new(obj).map(|obj| unsafe { Box::with_transfer(obj) })
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

#[allow(clippy::len_without_is_empty)]
pub trait NSStringInterface: NSObjectInterface + NSCopying<Result = NSString> {
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
    unsafe fn as_c_str(&self) -> Option<&CStr> {
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

impl<T> PartialEq<T> for NSString
where
    T: NSStringInterface,
{
    fn eq(&self, other: &T) -> bool {
        msg_send!((bool)[self, isEqualToString:(id)other])
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
        // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid pointer to an
        // Objective-C object.
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
    fn test_conversion() {
        let str = "Hello, World!";
        let string = NSStringClass.from_str(str);

        assert_eq!(str.len(), 13);
        assert_eq!(unsafe { string.as_c_str() }.unwrap().to_str().unwrap(), str);
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
    fn test_literal() {
        let str = &HELLO_WORLD;

        assert_eq!(str.len(), 13);
        assert_eq!(
            unsafe { str.as_c_str() }.unwrap().to_str().unwrap(),
            "Hello, World!"
        );
    }
}

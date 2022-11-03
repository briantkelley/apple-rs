use crate::NSCopying;
use core::ffi::{c_char, CStr};
use core::fmt::{self, Debug, Formatter};
use core::hash::{Hash, Hasher};
use core::ptr::NonNull;
use objc4::{
    extern_class, id, msg_send, objc_object, sel, Box, NSObjectClassInterface, NSObjectInterface,
    NSObjectProtocol, Object,
};

/// The following constants are provided by `NSString` as possible string encodings.
#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum NSStringEncoding {
    /// Strict 7-bit ASCII encoding within 8-bit chars; ASCII values 0…127 only.
    ASCII = 1,

    /// An 8-bit representation of Unicode characters, suitable for transmission or storage by ASCII-based systems.
    UTF8 = 4,

    /// 16-bit UTF encoding.
    UTF16 = 10,

    /// [`NSStringEncoding::UTF16`] encoding with explicit endianness specified.
    UTF16BigEndian = 0x9000_0100,

    /// [`NSStringEncoding::UTF16`] encoding with explicit endianness specified.
    UTF16LittleEndian = 0x9400_0100,

    /// 32-bit UTF encoding.
    UTF32 = 0x8c00_0100,

    /// [`NSStringEncoding::UTF32`] encoding with explicit endianness specified.
    UTF32BigEndian = 0x9800_0100,

    /// [`NSStringEncoding::UTF32`] encoding with explicit endianness specified.
    UTF32LittleEndian = 0x9c00_0100,
}

extern_class!(Foundation, pub NSString 'cls, NSObject 'cls; -PartialEq);

/// A static, plain-text Unicode string object.
pub trait NSStringClassInterface: NSObjectClassInterface {
    /// Returns an `NSString` object containing a given number of bytes from a given buffer of bytes
    /// interpreted in a given encoding.
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    #[must_use]
    fn from_bytes(&self, buf: &[u8], encoding: NSStringEncoding) -> Option<Box<Self::Instance>> {
        let obj = msg_send!(id, *const u8, usize, usize)(
            self.alloc().as_ptr(),
            sel![INITWITHBYTES_LENGTH_ENCODING_],
            buf.as_ptr(),
            buf.len(),
            encoding as usize,
        );
        // SAFETY: Objects retured by selectors beginning with ‘alloc’ must be released.
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

/// A static, plain-text Unicode string object.
#[allow(clippy::len_without_is_empty)]
pub trait NSStringInterface: NSObjectInterface + NSCopying<Result = NSString> {
    /// The number of UTF-16 code units in the receiver.
    #[inline]
    fn len(&self) -> usize {
        msg_send!(usize)(self.as_ptr(), sel![LENGTH])
    }

    /// Returns a boolean value that indicates whether a given string is equal to the receiver using
    /// a literal Unicode-based comparison.
    #[inline]
    fn is_equal_to_string(&self, other: &impl NSStringInterface) -> bool {
        msg_send!(bool, id)(self.as_ptr(), sel![ISEQUALTOSTRING_], other.as_ptr())
    }

    /// A null-terminated UTF-8 representation of the string.
    ///
    /// # Safety
    ///
    /// This method is unsafe because the returned reference is only valid through the current
    /// autorelease scope, which is not well-defined.
    #[inline]
    unsafe fn as_c_str(&self) -> Option<&CStr> {
        let str = msg_send!(*const c_char)(self.as_ptr(), sel![UTF8STRING]);
        if str.is_null() {
            None
        } else {
            // SAFETY: str is guaranteed to be a valid C string pointer.
            Some(unsafe { CStr::from_ptr(str) })
        }
    }
}

impl Hash for NSString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(NSObjectProtocol::hash(self));
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
        self.is_equal_to_string(other)
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

impl Eq for __CFConstantString {}
impl NSObjectInterface for __CFConstantString {}
impl NSObjectProtocol for __CFConstantString {}
impl NSStringInterface for __CFConstantString {}
impl Object for __CFConstantString {}
unsafe impl Sync for __CFConstantString {}

impl Debug for __CFConstantString {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let obj = self.as_ptr();
        // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid pointer to an
        // Objective-C object.
        unsafe { &*obj }.fmt(f)
    }
}

impl Hash for __CFConstantString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(NSObjectProtocol::hash(self));
    }
}

impl NSCopying for __CFConstantString {
    type Result = NSString;

    #[inline]
    fn copy(&self) -> Box<Self::Result> {
        let ptr = self.as_ptr();
        // SAFETY: ptr is derived from a reference and therefore cannot be null.
        let obj = unsafe { NonNull::new_unchecked(ptr) };
        objc4::Box::with_retained(obj)
    }
}

impl<T> PartialEq<T> for __CFConstantString
where
    T: NSStringInterface,
{
    fn eq(&self, other: &T) -> bool {
        self.is_equal_to_string(other)
    }
}

impl<T> PartialEq<Box<T>> for __CFConstantString
where
    T: NSStringInterface,
{
    fn eq(&self, other: &Box<T>) -> bool {
        self.is_equal_to_string(&**other)
    }
}

impl PartialEq<objc_object> for __CFConstantString {
    fn eq(&self, other: &objc_object) -> bool {
        self.is_equal(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string_literal;

    string_literal!(static HELLO_WORLD: NSString = "Hello, World!");

    struct AddHasher(u64);

    impl Hasher for AddHasher {
        fn finish(&self) -> u64 {
            self.0
        }

        fn write(&mut self, bytes: &[u8]) {
            let value = match bytes.len() {
                0 => 0_u64,
                1 => u64::from(bytes[0]),
                2 => u64::from(u16::from_ne_bytes([bytes[0], bytes[1]])),
                4 => u64::from(u32::from_ne_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])),
                8 => u64::from_ne_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
                ]),
                _ => bytes
                    .iter()
                    .fold(0_u64, |sum, byte| sum.wrapping_add(u64::from(*byte))),
            };
            self.0 = self.0.wrapping_add(value);
        }
    }

    #[test]
    fn test_copy() {
        let orig = &HELLO_WORLD;
        let copy = orig.copy();

        assert!(orig.is_equal(&*copy));
        assert!(orig.is_equal_to_string(&*copy));
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
        let data = &HELLO_WORLD;
        let heap = NSStringClass.from_str("Hello, World!");

        assert_eq!(
            NSObjectProtocol::hash(*data),
            NSObjectProtocol::hash(&*heap)
        );

        assert!(data.is_equal(&*heap));
        assert!(heap.is_equal(*data));

        assert_eq!(*data, &*heap);
        assert_eq!(&*heap, *data);

        assert_eq!(
            NSObjectProtocol::hash(*data),
            NSObjectProtocol::hash(&*heap)
        );

        let mut data_hasher = AddHasher(0);
        let mut heap_hasher = AddHasher(0);
        Hash::hash(data, &mut data_hasher);
        Hash::hash(&*heap, &mut heap_hasher);
        assert_eq!(data_hasher.finish(), heap_hasher.finish());
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

use crate::NSCopying;
use core::ffi::{c_char, CStr};
use core::ptr::NonNull;
use objc4::{extern_class, id, msg_send, sel, Box, NSObjectClassInterface, NSObjectInterface};

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

extern_class!(Foundation, pub NSString 'cls, NSObject 'cls);

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
pub trait NSStringInterface: NSObjectInterface + NSCopying {
    /// The number of UTF-16 code units in the receiver.
    #[inline]
    fn len(&self) -> usize {
        msg_send!(usize)(self.as_ptr(), sel![LENGTH])
    }

    /// Returns a boolean value that indicates whether a given string is equal to the receiver using
    /// a literal Unicode-based comparison.
    #[inline]
    fn is_equal_to_string(&self, other: &dyn NSStringInterface<Result = NSString>) -> bool {
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

impl NSCopying for NSString {
    type Result = Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string_literal;
    use objc4::NSObjectProtocol;

    string_literal!(static HELLO_WORLD: NSString = "Hello, World!");

    #[test]
    fn test_copy() {
        let orig = unsafe { &HELLO_WORLD };
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
        let data = unsafe { &HELLO_WORLD };
        let heap = NSStringClass.from_str("Hello, World!");

        assert_eq!(data.hash(), heap.hash());
        assert!(data.is_equal(&*heap));
    }

    #[test]
    fn test_literal() {
        let str = unsafe { &HELLO_WORLD };

        assert_eq!(str.len(), 13);
        assert_eq!(
            unsafe { str.as_c_str() }.unwrap().to_str().unwrap(),
            "Hello, World!"
        );
    }
}

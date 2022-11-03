/// Emits a compile-time constant `NSString`.
#[macro_export]
macro_rules! string_literal {
    ($vis:vis static $ident:ident: NSString = $value:literal) => {
        objc4::paste::paste! {
            #[link_section = "__DATA,__cfstring"]
            static [< _ $ident >]: $crate::__CFConstantString = $crate::__CFConstantString {
                // SAFETY: This pointer is not read through Rust. It's fully managed and only passed
                // to the Objective-C runtime.
                _isa: unsafe { &$crate::__CFConstantStringClassReference },
                _flags: 0x7C8, // Not 100% sure what this is, but Clang hard-codes for UTF-8
                _str: concat!($value, "\0").as_ptr(),
                _length: $value.len(),
            };
            // SAFETY: `__CFConstantStringClassReference` *is* an `NSString` subclass.
            $vis static $ident: &$crate::NSString = unsafe { core::mem::transmute::<_, _>(&[< _ $ident >]) };
        }
    };
}

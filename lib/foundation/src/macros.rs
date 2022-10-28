/// Emits a compile-time constant `NSString`.
#[macro_export]
macro_rules! string_literal {
    (static $ident:ident: NSString = $value:literal) => {
        objc4::paste::paste!{
            string_literal!(pub(self) static $ident: NSString = $value, [! $value:len !]);
        }
    };
    ($vis:vis static $ident:ident: NSString = $value:literal, $value_len:literal) => {
        core::arch::global_asm!(
            "    .pushsection __TEXT,__cstring,cstring_literals",
            concat!("l_", stringify!($ident), "$cstring:"),
            concat!("    .asciz   \"", $value, "\""),
            "",
            "    .section     __DATA,__cfstring",
            "    .p2align 3",
            concat!("_", stringify!($ident), ":"),
            "    .quad    ___CFConstantStringClassReference",
            "    .long    1992", // Not 100% sure what this is, but Clang hard-codeds for UTF-8
            "    .space   4",
            concat!("    .quad    l_", stringify!($ident), "$cstring"),
            concat!("    .quad   ", $value_len),
            "    .popsection",
        );

        extern "C" {
            $vis static $ident: $crate::NSString;
        }
    };
}

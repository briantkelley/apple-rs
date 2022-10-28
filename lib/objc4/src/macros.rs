pub use paste;

/// Defines a new Rust type for an Objective-C class defined in an external library and implements
/// all the given class hierarchy traits.
#[macro_export]
macro_rules! extern_class {
    // -kind
    ($library:ident, $vis:vis $($class:ident),+) => {
        $crate::extern_class!($library, kind = framework, $vis $($class),+);
    };
    // +kind
    ($library:ident, kind = $kind:ident, $vis:vis $($class:ident),+) => {
        $crate::extern_class!(@ $library; $kind; $vis $($class),+);
    };
    // private impl
    (@ $library:ident; $kind:ident; $vis:vis $ident:ident, $($super:ident),+) => {
        $crate::extern_class!(@ $library; $kind; $vis $ident);

        $crate::paste::paste! {
            $(impl [< $super Interface >] for $ident {})+
        }
    };
    (@ $library:ident; $kind:ident; $vis:vis $ident:ident) => {
        core::arch::global_asm!(
            "    .pushsection __DATA,__objc_classrefs,regular,no_dead_strip",
            "    .p2align 3",
            concat!("_", stringify!($ident), "ClassReference:"),
            concat!("    .quad    _OBJC_CLASS_$_", stringify!($ident)),
            "    .popsection",
        );

        $crate::paste::paste! {
            #[link(name = "" $library, kind = "" $kind)]
            extern "C" {
                #[link_name = "OBJC_CLASS_$_" $ident]
                static [< $ident Class >]: $crate::objc_class;
            }
        }

        #[allow(missing_copy_implementations, missing_docs)]
        #[repr(C)]
        $vis struct $ident (
            [u8; core::mem::size_of::<usize>()],
        );

        impl core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let obj = $crate::Object::as_ptr(self);
                // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid
                // pointer to an Objective-C object.
                unsafe { &*obj }.fmt(f)
            }
        }

        $crate::paste::paste! {
            impl $crate::Object for $ident {
                #[inline]
                fn class_type() -> &'static $crate::objc_class {
                    // SAFETY: Rust code never reads through the reference. The reference is passed
                    // to the Objective-C runtime, which is the owner of the data structure.
                    unsafe { &[< $ident Class >] }
                }
            }

            impl $crate::NSObjectProtocol for $ident {}
            impl [< $ident Interface >] for $ident {}
        }
    };
}

/// A macro to type cast `objc_msgSend` with the correct return type and argument types so the
/// compiler can pass the arguments as required by the ABI.
#[macro_export]
macro_rules! msg_send {
    ($ret:ty $(, $ty:ty)*) => {
        // SAFETY: Assume the user of the macro provided the correct return type, receiver type,
        // selector instance, and argument types.
        unsafe {
            let untyped: unsafe extern "C" fn() = $crate::objc_msgSend;
            core::mem::transmute::<
                _,
                extern "C" fn($crate::id, *const u8 $(, $ty)*) -> $ret,
            >(untyped)
        }
    };
}

/// A convenience macro to wrap the read of a selector symbol in an `unsafe` block.
#[allow(clippy::crate_in_macro_def)]
#[macro_export]
macro_rules! sel {
    [$cmd:ident] => {
        // SAFETY: Rust code never reads through the reference. The reference is passed to the
        // Objective-C runtime, which is the owner of the data type.
        unsafe { crate::sel::$cmd }
    }
}

/// Create a symbol for a selector. Requires providing the literal spelling as used by the runtime.
#[macro_export]
macro_rules! selector {
    ($ident:ident = $name:literal) => {
        core::arch::global_asm!(
            "    .pushsection __TEXT,__objc_methname,cstring_literals",
            concat!("l_SELECTOR_NAME_", stringify!($ident), ":"),
            concat!("    .asciz   \"", $name, "\""),
            "",
            "    .section     __DATA,__objc_selrefs,literal_pointers,no_dead_strip",
            "    .p2align 3",
            concat!("_SELECTOR_", stringify!($ident), ":"),
            concat!("    .quad    l_SELECTOR_NAME_", stringify!($ident)),
            "    .popsection",
        );

        extern "C" {
            $crate::paste::paste! {
                #[link_name = "SELECTOR_" $ident]
                pub(super) static $ident: *const u8;
            }
        }
    };
}

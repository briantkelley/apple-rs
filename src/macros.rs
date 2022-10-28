pub use paste;

/// Defines a new Rust type for an Objective-C class defined in an external library and implements
/// all the given class hierarchy traits.
#[macro_export]
macro_rules! extern_class {
    // -kind, -vis
    ($library:ident, $ident:ident) => {
        extern_class!($library, $ident,);
    };
    ($library:ident, $ident:ident, $($super:ident),*) => {
        extern_class!($library, kind = framework, $ident, $($super),*);
    };
    // +kind, -vis
    ($library:ident, kind = $kind:ident, $ident:ident) => {
        extern_class!($library, kind = $kind, $ident,);
    };
    ($library:ident, kind = $kind:ident, $ident:ident, $($super:ident),*) => {
        extern_class!($library, kind = $kind, pub(self) $ident, $($super),*);
    };
    // -kind, +vis
    ($library:ident, $vis:vis $ident:ident) => {
        extern_class!($library, $vis $ident,);
    };
    ($library:ident, $vis:vis $ident:ident, $($super:ident),*) => {
        extern_class!($library, kind = framework, $vis $ident, $($super),*);
    };
    // +kind, +vis
    ($library:ident, kind = $kind:ident, $vis:vis $ident:ident) => {
        extern_class!($library, kind = $kind, $vis $ident,);
    };
    ($library:ident, kind = $kind:ident, $vis:vis $ident:ident, $($super:ident),*) => {
        core::arch::global_asm!(
            "    .pushsection __DATA,__objc_classrefs,regular,no_dead_strip",
            "    .p2align 3",
            concat!("_", stringify!($ident), "ClassReference:"),
            concat!("    .quad    _OBJC_CLASS_$_", stringify!($ident)),
            "    .popsection",
        );

        paste::paste! {
            #[link(name = "" $library, kind = "" $kind)]
            extern "C" {
                #[link_name = "OBJC_CLASS_$_" $ident]
                static [< $ident Class >]: $crate::objc_class;
            }
        }

        #[allow(missing_copy_implementations, missing_docs)]
        #[repr(C)]
        $vis struct $ident {
            _isa: [u8; core::mem::size_of::<usize>()],
        }

        impl core::fmt::Debug for $ident {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let obj = self.as_ptr();
                // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid
                // pointer to an Objective-C object.
                unsafe { &*obj }.fmt(f)
            }
        }

        paste::paste! {
            impl $crate::Object for $ident {
                #[inline]
                fn class_type() -> &'static $crate::objc_class {
                    // SAFETY: Rust code never reads through the reference. The reference is passed
                    // to the Objective-C runtime, which is the owner of the data structure.
                    unsafe { &[< $ident Class >] }
                }
            }

            impl NSObjectProtocol for $ident {}
            $(impl [< $super Interface >] for $ident {})*
            impl [< $ident Interface >] for $ident {}
        }
    };
}

/// Use in each crate that defines or uses Objective-C metadata (e.g. class, selectors, categories).
///
/// Adds an `__objc_imageinfo` segment to the `__DATA` section so that the runtime will load and fix
/// up the Objective-C metadata.
#[macro_export]
macro_rules! image_info {
    () => {
        core::arch::global_asm!(
            "    .pushsection __DATA,__objc_imageinfo,regular,no_dead_strip",
            "L_OBJC_IMAGE_INFO:",
            "    .long    0",
            "    .long    0",
            "    .popsection",
        );
    };
}

/// A macro to type cast `objc_msgSend` with the correct return type and argument types so the
/// compiler can pass the arguments as required by the ABI.
#[macro_export]
macro_rules! msg_send {
    ($ret:ty) => {
        // SAFETY: Assume the user of the macro provided the correct return type, receiver type, and
        // selector instance.
        unsafe {
            let untyped: unsafe extern "C" fn() = $crate::objc_msgSend;
            core::mem::transmute::<
                _,
                extern "C" fn($crate::id, *const u8) -> $ret,
            >(untyped)
        }
    };
    ($ret:ty, $($ty:ty),+) => {
        // SAFETY: Assume the user of the macro provided the correct return type, receiver type,
        // selector instance, and argument types.
        unsafe {
            let untyped: unsafe extern "C" fn() = $crate::objc_msgSend;
            core::mem::transmute::<
                _,
                extern "C" fn($crate::id, *const u8, $($ty),+) -> $ret,
            >(untyped)
        }
    };
}

/// A convenience macro to wrap the read of a selector symbol in an `unsafe` block.
#[macro_export]
macro_rules! sel {
    [$cmd:ident] => {
        // SAFETY: Rust code never reads through the reference. The reference is passed to the
        // Objective-C runtime, which is the owner of the data type.
        unsafe { $cmd }
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
            paste::paste! {
                #[link_name = "SELECTOR_" $ident]
                static $ident: *const u8;
            }
        }
    };
}

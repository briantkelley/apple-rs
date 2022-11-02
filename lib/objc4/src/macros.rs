pub use paste;

/// Defines a new Rust type for an Objective-C class defined in an external library and implements
/// all the given class hierarchy traits.
#[macro_export]
macro_rules! extern_class {
    // without link kind
    ($library:ident, $vis:vis $($class:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?),+ $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@1 $library; framework; $vis $($class $($class_interface)? $(< $($class_param),+ >)?),+ $(; $($param : $ty),+)?);
    };
    // with link kind
    ($library:ident, kind = $kind:ident, $vis:vis $($class:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?),+ $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@1 $library; $kind; $vis $($class $($class_interface)? $(< $($class_param),+ >)?),+ $(; $($param : $ty),+)?);
    };
    // private impl
    (@1 $library:ident; $kind:ident; $vis:vis $class:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)? $(, $super:ident $($super_class_interface:lifetime)? $(< $($super_param:ident),+ >)?)* $(; $($param:ident : $ty:path),+)?) => {
        // <Class>Class and <Class> type definitions; Debug and Object implementations
        $crate::extern_class!(@2 $library; $kind; $vis $class $(< $($class_param),+ >)? $(; $($param : $ty),+)?);
        // <Class>ClassInterface, <Class>Interface, and Upcast<&Class, &Super> implementations
        $crate::extern_class!(@3 $class; $class $($class_interface)? $(< $($class_param),+ >)? $(, $super $($super_class_interface)? $(< $($super_param),+ >)?)* $(; $($param : $ty),+)?);
    };
    (@2 $library:ident; $kind:ident; $vis:vis $class:ident $(< $($class_param:ident),+ >)? $(; $($param:ident : $ty:path),+)?) => {
        core::arch::global_asm!(
            "    .pushsection __DATA,__objc_classrefs,regular,no_dead_strip",
            "    .p2align 3",
            concat!("_", stringify!($class), "ClassReference:"),
            concat!("    .quad    _OBJC_CLASS_$_", stringify!($class)),
            "    .popsection",
        );

        $crate::paste::paste! {
            #[link(name = "" $library, kind = "" $kind)]
            extern "C" {
                #[link_name = "OBJC_CLASS_$_" $class]
                static [< _ $class Class >]: [< $class ClassType >];
            }

            #[allow(missing_docs, non_upper_case_globals)]
            $vis static [< $class Class >]: &[< $class ClassType >] = unsafe { &[< _ $class Class >] };

            #[allow(missing_copy_implementations, missing_docs)]
            #[repr(transparent)]
            $vis struct [< $class ClassType >] (
                $crate::objc_class,
            );

            impl core::fmt::Debug for [< $class ClassType >] {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    self.0.fmt(f)
                }
            }
        }

        #[allow(missing_copy_implementations, missing_docs)]
        #[repr(C)]
        $vis struct $class $(< $($param : $ty),+ >)? (
            [u8; core::mem::size_of::<usize>()],
            $($(core::marker::PhantomData<$param>,)+)?
        );

        impl $(< $($param),+ >)? core::fmt::Debug for $class $(< $($param),+ >)?
        $(where $($param : $ty),+)?
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let obj = $crate::Object::as_ptr(self);
                // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid
                // pointer to an Objective-C object.
                unsafe { &*obj }.fmt(f)
            }
        }

        $crate::paste::paste! {
            impl $(< $($param),+ >)? $crate::Object for $class $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {}
        }
    };
    (@3 $class:ident; $(; $($param:ident : $ty:path),+)?) => {};
    (@3 $class:ident; $interface:ident $($interface_class_interface:lifetime)? $(< $($interface_param:ident),+ >)? $(, $super:ident $($super_class_interface:lifetime)? $(< $($super_param:ident),+ >)?)* $(; $($param:ident : $ty:path),+)?) => {
        // Expand from root to derived so `cargo expand` shows root to derived.
        $crate::extern_class!(@3 $class; $($super $($super_class_interface)? $(< $($super_param),+ >)?),* $(; $($param : $ty),+)?);

        // <Interface>ClassInterface
        $crate::extern_class!(@4 $class, $interface $($interface_class_interface)? $(; $($param : $ty),+)?);
        // <Interface>Interface
        $crate::extern_class!(@5 $class; $interface $(< $($interface_param),+ >)? $(; $($param : $ty),+)?);
        // Upcast<&Class, &Super>
        $crate::extern_class!(@6 $class; $($super $(< $($super_param),+ >)?),* $(; $($param : $ty),+)?);
    };
    (@4 $class:ident, $interface:ident $(; $($param:ident : $ty:path),+)?) => {};
    (@4 $class:ident, NSObject 'cls) => {
        $crate::paste::paste! {
            impl $crate::NSObjectClassInterface for [< $class ClassType >] {
                type Instance = $class;
            }
        }
    };
    (@4 $class:ident, NSObject 'cls; $($param:ident : $ty:path),+) => {
        $crate::paste::paste! {
            impl $crate::NSObjectClassInterface for [< $class ClassType >] {
                // Use `id` for types with generic parameters. Otherwise, the class type would
                // require generic parameters to specify the generic types on the associated type,
                // which creates *n* class types where only 1 exists.
                type Instance = $crate::NSObject;
            }
        }
    };
    (@4 $class:ident, $interface:ident 'cls $(; $($param:ident : $ty:path),+)?) => {
        $crate::paste::paste! {
            impl [< $interface ClassInterface >] for [< $class ClassType >] {}
        }
    };
    (@5 $class:ident; NSObject $(; $($param:ident : $ty:path),+)?) => {
        $crate::paste::paste! {
            impl $(< $($param),+ >)? $crate::NSObjectProtocol for $class $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {}

            impl $(< $($param),+ >)? $crate::NSObjectInterface for $class $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {}
       }
    };
    (@5 $class:ident; $interface:ident $(< $($interface_param:ident),+ >)? $(; $($param:ident : $ty:path),+)?) => {
        $crate::paste::paste! {
            impl $(< $($param),+ >)? [< $interface Interface >] for $class $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {
                $($(type $interface_param = $interface_param;)+)?
            }
        }
    };
    (@6 $class:ident; $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@7 $class; $crate::objc_object $(; $($param : $ty),+)?);
    };
    (@6 $class:ident; NSObject $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@7 $class; $crate::NSObject $(; $($param : $ty),+)?);
    };
    (@6 $class:ident; $super:path $(, $_super:path)* $(; $($param:ident : $ty:path),+)?) => {
        $crate::extern_class!(@7 $class; $super $(; $($param : $ty),+)?);
    };
    (@7 $class:ident; $super:path $(; $($param:ident : $ty:path),+)?) => {
        impl<'a $(, $($param),+ )? > $crate::Upcast< &'a $class $(< $($param),+ >)?, &'a $super > for $class $(< $($param),+ >)?
        $(where $($param : $ty),+)?
        {
            fn upcast(from: &Self) -> & $super {
                let ptr: *const Self = from;
                let ptr = ptr.cast();
                // SAFETY: We trust the class hierarchy specification is correct.
                unsafe { &*ptr }
            }
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

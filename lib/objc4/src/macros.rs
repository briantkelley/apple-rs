pub use paste;

/// Defines a new Rust type for an Objective-C class defined in an external library and implements
/// all the given class hierarchy traits.
#[macro_export]
macro_rules! extern_class {
    // without link kind
    ($library:ident, $vis:vis $($class:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?),+ $(; $($param:ident : $ty:path),+)? $(; $(-$skip:ident),+)?) => {
        $crate::extern_class!(@1 $library; framework; $vis $($class $($class_interface)? $(< $($class_param),+ >)?),+ $(; $($param : $ty),+)? $(; $(-$skip),+)?);
    };
    // with link kind
    ($library:ident, kind = $kind:ident, $vis:vis $($class:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)?),+ $(; $($param:ident : $ty:path),+)? $(; $(-$skip:ident),+)?) => {
        $crate::extern_class!(@1 $library; $kind; $vis $($class $($class_interface)? $(< $($class_param),+ >)?),+ $(; $($param : $ty),+)? $(; $(-$skip),+)?);
    };
    // private impl
    (@1 $library:ident; $kind:ident; $vis:vis $class:ident $($class_interface:lifetime)? $(< $($class_param:ident),+ >)? $(, $super:ident $($super_class_interface:lifetime)? $(< $($super_param:ident),+ >)?)* $(; $($param:ident : $ty:path),+)? $(; $(-$skip:ident),+)?) => {
        // <Class>Class and <Class> type definitions; Debug and Object implementations
        $crate::extern_class!(@2 $library; $kind; $vis $class $(< $($class_param),+ >)? $(; $($param : $ty),+)?);
        // PartialEq trait
        $crate::extern_class!(@8 $class $(; $($param : $ty),+)? $(; $(-$skip),+)?);
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
                let obj: *const _ = self;
                let obj: *const $crate::objc_object = obj.cast();
                // SAFETY: `obj` is derived from a reference so it is guaranteed to be a valid
                // pointer to an Objective-C object.
                unsafe { &*obj }.fmt(f)
            }
        }

        impl $(< $($param),+ >)? $crate::Object for $class $(< $($param),+ >)?
        $(where $($param : $ty),+)?
        {}
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
            impl $(< $($param),+ >)? Eq for $class $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {}

            #[allow(unused_qualifications)]
            impl $(< $($param),+ >)? core::hash::Hash for $class $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {
                fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                    let hash = $crate::msg_send!((usize)[self, hash]);
                    state.write_usize(hash);
                }
            }

            impl $(< $($param),+ >)? PartialEq<$crate::Box<Self>> for $class $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {
                fn eq(&self, other: &$crate::Box<Self>) -> bool {
                    self == core::ops::Deref::deref(other)
                }
            }

            impl $(< $($param),+ >)? PartialEq<$crate::objc_object> for $class $(< $($param),+ >)?
            $(where $($param : $ty),+)?
            {
                fn eq(&self, other: &$crate::objc_object) -> bool {
                    $crate::msg_send!((bool)[self, isEqual:(id)other])
                }
            }

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
        impl<'a $(, $($param),+ )?> $crate::Upcast<&'a $class $(< $($param),+ >)?, &'a $super> for $class $(< $($param),+ >)?
        $(where $($param : $ty),+)?
        {
            fn upcast(from: &Self) -> &$super {
                let ptr: *const Self = from;
                let ptr = ptr.cast();
                // SAFETY: We trust the class hierarchy specification is correct.
                unsafe { &*ptr }
            }
        }
    };
    (@8 $class:ident $(; $($param:ident : $ty:path),+)?) => {
        // The macro invocation does not contain -PartialEq so implement the default.
        $crate::paste::paste! {
            impl< T $(, $($param),+)?> PartialEq<T> for $class $(< $($param),+ >)?
            where
                T: [< $class Interface >]
                $($(, $param : $ty)+)?
            {
                fn eq(&self, other: &T) -> bool {
                    $crate::msg_send!((bool)[self, isEqual:(id)other])
                }
            }
        }
    };
    (@8 $class:ident $(; $($param:ident : $ty:path),+)?; -PartialEq) => {
    };
}

/// A macro to call `objc_msgSend` with the correct return type and argument types so the compiler
/// can pass the arguments as required by the ABI.
#[macro_export]
macro_rules! msg_send {
    ([$self:expr, $cmd:ident]) => {
        $crate::msg_send!(@1 (), $self, $cmd)
    };
    ([$self:expr, $($cmd:ident : ($($ty:tt)+) $arg:expr)+]) => {
        $crate::msg_send!(@1 (), $self, $($cmd, ($($ty)+), $arg)+)
    };
    (($ret:ty)[$self:expr, $cmd:ident]) => {
        $crate::msg_send!(@1 $ret, $self, $cmd)
    };
    (($ret:ty)[$self:expr, $($cmd:ident : ($($ty:tt)+) $arg:expr)+]) => {
        $crate::msg_send!(@1 $ret, $self, $($cmd, ($($ty)+), $arg)+)
    };
    (@1 $ret:ty, $self:expr, $($cmd:ident $(, ($($ty:tt)+), $arg:expr)?)+) => {
        $crate::__msg_send_helper!(@ $ret, $self, $($cmd $(, ($($ty)+), $arg)?)+)
    };
    (@2 $cmd:ident) => {
        stringify!($cmd)
    };
    (@2 $($cmd:ident, ($($ty:tt)+)),+) => {
        concat!($(stringify!($cmd), ":"),+)
    };
    (@3 $arg:expr, id) => {
        ($arg) as *const _ as $crate::id
    };
    (@3 $arg:expr, $crate::id) => {
        ($arg) as *const _ as $crate::id
    };
    (@3 $arg:expr, $($ty:tt)+) => {
        $arg
    };
}

#[cfg(target_arch = "aarch64")]
#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_helper {
    (@ $ret:ty, $self:expr, $($cmd:ident $(, ($($ty:tt)+), $arg:expr)?)+) => {
        // SAFETY: Assume the user of the macro provided the correct return type, receiver type,
        // selector instance, and argument types.
        unsafe {
            extern "C" {
                #[allow(clashing_extern_declarations)]
                #[link_name = concat!("objc_msgSend$", $crate::msg_send!(@2 $($cmd $(, ($($ty)+))?),+))]
                fn objc_msgSend(receiver: $crate::id, _cmd: *const u8 $($(, $cmd: $($ty)+)?)+) -> $ret;
            }
            core::arch::asm!(
                "    .pushsection __DATA,__objc_imageinfo,regular,no_dead_strip",
                "    .long    0",
                "    .long    0",
                "    .popsection",
            );
            #[allow(invalid_value, trivial_casts)]
            objc_msgSend(
                $self as *const _ as *mut $crate::objc_object,
                core::mem::MaybeUninit::uninit().assume_init()
                $($(, $crate::msg_send!(@3 $arg, $($ty)+))?)+
            )
        }
    };
}

#[cfg(target_arch = "x86_64")]
#[doc(hidden)]
#[macro_export]
macro_rules! __msg_send_helper {
    (@ $ret:ty, $self:expr, $($cmd:ident $(, $ty:ty, $arg:expr)?)+) => {
        // SAFETY: Assume the user of the macro provided the correct return type, receiver type,
        // selector instance, and argument types.
        unsafe {
            #[link(name = "objc")]
            extern "C" {
                /// Sends a message with a simple return value to an instance of a class.
                #[allow(clashing_extern_declarations)]
                fn objc_msgSend();
            }
            let cmd: *const u8;
            core::arch::asm!(
                "    .pushsection __DATA,__objc_imageinfo,regular,no_dead_strip",
                "    .long    0",
                "    .long    0",
                "    .section __TEXT,__objc_methname,cstring_literals",
                "2:",
                concat!("    .asciz   \"", $crate::msg_send!(@2 $($cmd $(, $ty)?),+), "\""),
                "",
                "    .section     __DATA,__objc_selrefs,literal_pointers,no_dead_strip",
                "    .p2align 3",
                "3:",
                "    .quad    2b",
                "    .popsection",
                "mov    {x}, [rip + 3b]",
                x = out(reg) cmd,
                options(nomem, nostack, pure),
            );
            let untyped: unsafe extern "C" fn() = objc_msgSend;
            let typed = core::mem::transmute::<
                _,
                unsafe extern "C" fn($crate::id, *const u8 $($(, $ty)?)+) -> $ret,
            >(untyped);
            #[allow(trivial_casts)]
            typed($self as *const _ as *mut $crate::objc_object, cmd $($(, $arg)?)+)
        }
    };
}

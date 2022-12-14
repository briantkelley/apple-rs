pub use paste;

#[macro_export]
macro_rules! activity_initiate {
    ($description:literal, $function:expr) => {
        {
            $crate::log_string!(static DESCRIPTION = $description);
            $crate::activity::initiate(DESCRIPTION, $function);
        }
    };
}

#[macro_export]
macro_rules! activity_label_useraction {
    ($label:literal) => {
        {
            $crate::log_string!(static LABEL = $label);
            $crate::activity::label_useraction(LABEL);
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _log_impl {
    ($log:expr, $ty:ident, $format:literal $(, $item:expr)*) => {
        {
            use $crate::log::BuilderItem;

            $crate::log_string!(static FORMAT = $format);
            if let Some(builder) = ($log).$ty(FORMAT) {
                builder
                    $(.item($item))*
                    .log();
            }
        }
    };
}

#[macro_export]
macro_rules! log {
    ($log:expr, $format:literal $(, $item:expr)*) => {
        $crate::_log_impl!($log, log, $format $(, $item)*)
    };
}

#[macro_export]
macro_rules! log_info {
    ($log:expr, $format:literal $(, $item:expr)*) => {
        $crate::_log_impl!($log, info, $format $(, $item)*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($log:expr, $format:literal $(, $item:expr)*) => {
        $crate::_log_impl!($log, debug, $format $(, $item)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($log:expr, $format:literal $(, $item:expr)*) => {
        $crate::_log_impl!($log, error, $format $(, $item)*)
    };
}

#[macro_export]
macro_rules! log_fault {
    ($log:expr, $format:literal $(, $item:expr)*) => {
        $crate::_log_impl!($log, fault, $format $(, $item)*)
    };
}

#[macro_export]
macro_rules! log_string {
    (static $ident:ident = $literal:literal) => {
        $crate::paste::paste! {
            #[link_section = "__TEXT,__oslogstring,cstring_literals"]
            static [< $ident _CSTRING >]: $crate::trace_base::_LogStringImpl<[u8; $literal.len()]> =
                $crate::trace_base::_LogStringImpl {
                    _str: *$literal,
                    _nul: 0,
                };
            static $ident: $crate::trace_base::LogString = $crate::trace_base::LogString {
                ptr: {
                    let log_string: *const _ = &[< $ident _CSTRING >];
                    log_string.cast()
                },
            };
        }
    };
}

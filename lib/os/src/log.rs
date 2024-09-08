use crate::sys::log::{
    _os_log_debug_impl, _os_log_default, _os_log_disabled, _os_log_error_impl, _os_log_fault_impl,
    _os_log_impl, os_log_t, os_log_type_enabled, os_log_type_t, OS_LOG_TYPE_DEBUG,
    OS_LOG_TYPE_DEFAULT, OS_LOG_TYPE_ERROR, OS_LOG_TYPE_FAULT, OS_LOG_TYPE_INFO,
};
use crate::sys::trace_base::__dso_handle;
use crate::trace_base::LogString;
use core::fmt::{self, Debug, Formatter};

#[derive(Clone, Copy)]
enum Kind {
    Scalar,
}

#[derive(Clone, Copy, Debug)]
pub enum Privacy {
    Private,
    Public,
    Sensitive,
}

pub trait BuilderItem<U>: Sized
where
    U: Copy + Debug,
{
    type Builder;

    fn item(self, value: U) -> Self::Builder;
    fn item_with_privacy(self, value: U, privacy: Privacy) -> Self::Builder;
}

#[repr(C, align(16))]
struct AlignedBuffer<T>(Buffer<T>);

#[repr(C, packed)]
struct Buffer<T> {
    summary: u8,
    items_len: u8,
    items: T,
}

#[allow(missing_debug_implementations)]
pub struct Builder<T> {
    parameters: Parameters,
    buffer: AlignedBuffer<T>,
}

#[repr(C, packed)]
pub struct Item<T>
where
    T: Copy + Debug,
{
    descriptor: u8,
    size: u8,
    value: T,
}

#[allow(missing_debug_implementations)]
#[repr(C, packed)]
pub struct Items<A, B>(A, B);

#[allow(missing_copy_implementations, missing_debug_implementations)]
#[derive(Debug)]
#[repr(transparent)]
pub struct Log(os_log_t);

struct Parameters {
    log: os_log_t,
    func: unsafe extern "C" fn(*const u32, os_log_t, os_log_type_t, LogString, *const u8, u32),
    ty: os_log_type_t,
    format: LogString,
}

// Item Descriptor Flags
const PRIVATE: u8 = 1 << 0;
const PUBLIC: u8 = 1 << 1;
const SENSITIVE: u8 = (1 << 2) | PRIVATE;

// Buffer Summary Flags
const HAS_PRIVATE_ITEMS: u8 = 1 << 0;

impl<T> AlignedBuffer<T> {
    const fn raw_parts(&self) -> (*const u8, u32) {
        let buf: *const _ = self;

        #[allow(clippy::cast_possible_truncation)] // truncation will never happen
        (buf.cast(), size_of::<Buffer<T>>() as u32)
    }
}

impl AlignedBuffer<()> {
    const fn new() -> Self {
        Self(Buffer {
            summary: 0,
            items_len: 0,
            items: (),
        })
    }
}

impl<T> Builder<T> {
    #[allow(clippy::missing_const_for_fn)] // false positive
    fn append<U>(self, item: Item<U>) -> Builder<Items<T, Item<U>>>
    where
        U: Copy + Debug,
    {
        Builder {
            parameters: self.parameters,
            buffer: AlignedBuffer(Buffer {
                summary: self.buffer.0.summary | item.summary_flags(),
                items_len: self.buffer.0.items_len + 1,
                items: Items(self.buffer.0.items, item),
            }),
        }
    }

    pub fn log(self) {
        // SAFETY: This matches the canonical mechanics of `<os/log.h>`.
        let dso: *const _ = unsafe { &__dso_handle };
        let (buf, size) = self.buffer.raw_parts();

        let Parameters {
            log,
            func,
            ty,
            format,
        } = self.parameters;

        // SAFETY: This matches the canonical mechanics of `<os/log.h>`.
        unsafe { (func)(dso, log, ty, format, buf, size) };
    }
}

impl<T> Item<T>
where
    T: Copy + Debug,
{
    const fn new(value: T, kind: Kind, privacy: Option<Privacy>) -> Self {
        let privacy: u8 = match privacy {
            None => 0,
            Some(Privacy::Private) => PRIVATE,
            Some(Privacy::Public) => PUBLIC,
            Some(Privacy::Sensitive) => SENSITIVE,
        };

        let kind: u8 = match kind {
            Kind::Scalar => 0,
        };

        #[allow(clippy::cast_possible_truncation)] // truncation will never happen
        Self {
            descriptor: privacy | (kind << 4),
            size: size_of::<T>() as u8,
            value,
        }
    }

    const fn summary_flags(&self) -> u8 {
        if (self.descriptor & PRIVATE) == 0 {
            0
        } else {
            HAS_PRIVATE_ITEMS
        }
    }
}

impl<T> Debug for Item<T>
where
    T: Copy + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let value = self.value;

        f.debug_struct("Item")
            .field("descriptor", &self.descriptor)
            .field("size", &self.size)
            .field("value", &value)
            .finish()
    }
}

impl Log {
    #[must_use]
    pub fn disabled() -> Self {
        // SAFETY: This matches the canonical mechanics of `<os/log.h>`.
        Self(unsafe { &_os_log_disabled })
    }

    #[must_use]
    pub fn log(self, format: LogString) -> Option<Builder<()>> {
        Self::with_parameters(Parameters {
            log: self.0,
            func: _os_log_impl,
            ty: OS_LOG_TYPE_DEFAULT,
            format,
        })
    }

    #[must_use]
    pub fn info(self, format: LogString) -> Option<Builder<()>> {
        Self::with_parameters(Parameters {
            log: self.0,
            func: _os_log_impl,
            ty: OS_LOG_TYPE_INFO,
            format,
        })
    }

    #[must_use]
    pub fn debug(self, format: LogString) -> Option<Builder<()>> {
        Self::with_parameters(Parameters {
            log: self.0,
            func: _os_log_debug_impl,
            ty: OS_LOG_TYPE_DEBUG,
            format,
        })
    }

    #[must_use]
    pub fn error(self, format: LogString) -> Option<Builder<()>> {
        Self::with_parameters(Parameters {
            log: self.0,
            func: _os_log_error_impl,
            ty: OS_LOG_TYPE_ERROR,
            format,
        })
    }

    #[must_use]
    pub fn fault(self, format: LogString) -> Option<Builder<()>> {
        Self::with_parameters(Parameters {
            log: self.0,
            func: _os_log_fault_impl,
            ty: OS_LOG_TYPE_FAULT,
            format,
        })
    }

    fn with_parameters(parameters: Parameters) -> Option<Builder<()>> {
        // SAFETY: This matches the canonical mechanics of `<os/log.h>`.
        if unsafe { os_log_type_enabled(parameters.log, parameters.ty) } {
            Some(Builder {
                parameters,
                buffer: AlignedBuffer::new(),
            })
        } else {
            None
        }
    }
}

impl Default for Log {
    fn default() -> Self {
        // SAFETY: This matches the canonical mechanics of `<os/log.h>`.
        Self(unsafe { &_os_log_default })
    }
}

macro_rules! builder_scalar_item {
    ($ty:ty) => {
        impl<T> BuilderItem<$ty> for Builder<T> {
            type Builder = Builder<Items<T, Item<$ty>>>;

            fn item(self, value: $ty) -> Self::Builder {
                self.append(Item::new(value, Kind::Scalar, None))
            }

            fn item_with_privacy(self, value: $ty, privacy: Privacy) -> Self::Builder {
                self.append(Item::new(value, Kind::Scalar, Some(privacy)))
            }
        }
    };
}

builder_scalar_item!(i32);
builder_scalar_item!(u32);

#[cfg(test)]
mod test {
    use super::{AlignedBuffer, BuilderItem, Log, Privacy};
    use crate::{log, log_debug, log_error, log_fault, log_info};
    use core::slice;

    log_string!(static UNUSED = b"");

    #[test]
    fn buffer_empty() {
        let buffer = AlignedBuffer::<()>::new();
        let (buf, size) = buffer.raw_parts();

        assert!(!buf.is_null());
        assert_eq!((buf as usize) & 0x0F, 0);
        assert_eq!(size, 2);
    }

    #[test]
    fn builder_integers() {
        let builder = Log::default()
            .error(UNUSED)
            .unwrap()
            .item(0xdead_beef_u32.to_be());

        let (buf, size) = builder.buffer.raw_parts();
        let bytes = unsafe { slice::from_raw_parts(buf, 2 + 6) };

        assert_eq!(size as usize, bytes.len());
        assert_eq!(bytes, &[0x00, 0x01, 0x00, 0x04, 0xde, 0xad, 0xbe, 0xef]);

        let builder = builder.item_with_privacy(0xda_bb1e_i32.to_be(), Privacy::Sensitive);

        let (buf, size) = builder.buffer.raw_parts();
        let bytes = unsafe { slice::from_raw_parts(buf, 2 + 6 + 6) };

        assert_eq!(size as usize, bytes.len());
        assert_eq!(
            bytes,
            &[0x01, 0x02, 0x00, 0x04, 0xde, 0xad, 0xbe, 0xef, 0x05, 0x04, 0x00, 0xda, 0xbb, 0x1e]
        );
    }

    #[test]
    fn log_macros() {
        let a: i32 = -1;
        let b: u32 = 0x8000_0015;

        log!(Log::default(), b"minus one = %d; mask = %x", a, b);
        log_info!(Log::default(), b"minus one = %d; mask = %x", a, b);
        log_debug!(Log::default(), b"minus one = %d; mask = %x", a, b);
        log_error!(Log::default(), b"minus one = %d; mask = %x", a, b);
        log_fault!(Log::default(), b"minus one = %d; mask = %x", a, b);
    }
}

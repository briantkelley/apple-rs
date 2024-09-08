pub use core::ffi::c_void;

pub type Boolean = u8;
pub type UInt8 = u8;
pub type UniChar = u16;
pub type UTF32Char = u32;

pub type CFTypeID = usize;
pub type CFOptionFlags = usize;
pub type CFHashCode = usize;
pub type CFIndex = isize;

/// Base "type" of all "CF objects", and polymorphic functions on them
pub type CFTypeRef = *const c_void;

declare_cf_type!(__CFString, CFStringRef, CFMutableStringRef);

/// Constant used by some functions to indicate failed searches.
pub const kCFNotFound: CFIndex = -1;

/// Range type
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(C)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}

declare_cf_type!(__CFAllocator, CFAllocatorRef);

extern "C" {
    /// This is a synonym for `NULL`, if you'd rather use a named constant.
    pub static kCFAllocatorDefault: CFAllocatorRef;

    /// Default system allocator; you rarely need to use this.
    pub static kCFAllocatorSystemDefault: CFAllocatorRef;

    /// This allocator uses `malloc()`, `realloc()`, and `free()`. This should not be generally
    /// used; stick to [`kCFAllocatorDefault`] whenever possible. This allocator is useful as the
    /// `bytesDeallocator` in `CFData` or `contentsDeallocator` in `CFString` where the memory was
    /// obtained as a result of `malloc()` type functions.
    pub static kCFAllocatorMalloc: CFAllocatorRef;

    /// This allocator explicitly uses the default malloc zone, returned by `malloc_default_zone()`.
    /// It should only be used when an object is safe to be allocated in non-scanned memory.
    pub static kCFAllocatorMallocZone: CFAllocatorRef;

    /// Null allocator which does nothing and allocates no memory. This allocator is useful as the
    /// `bytesDeallocator` in `CFData` or `contentsDeallocator` in `CFString` where the memory
    /// should not be freed.
    pub static kCFAllocatorNull: CFAllocatorRef;

    /// Special allocator argument to [`CFAllocatorCreate`] which means "use the functions given in
    /// the context to allocate the allocator itself as well".
    pub static kCFAllocatorUseContext: CFAllocatorRef;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct CFAllocatorContext {
    version: CFIndex,
    info: *mut c_void,
    retain: extern "C" fn(info: *const c_void) -> *const c_void,
    release: extern "C" fn(info: *const c_void),
    copyDescription: extern "C" fn(info: *const c_void) -> CFStringRef,
    allocate:
        extern "C" fn(allocSize: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> *mut c_void,
    reallocate: extern "C" fn(
        ptr: *mut c_void,
        newsize: CFIndex,
        hint: CFOptionFlags,
        info: *mut c_void,
    ) -> *mut c_void,
    deallocate: extern "C" fn(ptr: *mut c_void, info: *mut c_void),
    preferredSize: extern "C" fn(size: CFIndex, hint: CFOptionFlags, info: *mut c_void) -> CFIndex,
}

extern "C" {
    pub fn CFAllocatorCreate(
        allocator: CFAllocatorRef,
        context: &CFAllocatorContext,
    ) -> CFAllocatorRef;

    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean;
    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;
    pub fn CFCopyDescription(cf: CFTypeRef) -> CFStringRef;
}

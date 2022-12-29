#![allow(non_camel_case_types)]

const USER_FUNCTION: u32 = 0x8000_0000;

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum FunctionID {
    // System Calls
    open = 5,
    unlink = 10,
    rmdir = 137,
    fstat = 339,
    fclonefileat = 517,

    // User Functions
    confstr = USER_FUNCTION | 99,
    mkdtemp = USER_FUNCTION | 140,
    mkstemp = USER_FUNCTION | 145,

    // Binding Functions
    CStr__from_bytes_with_nul = USER_FUNCTION | 100_010,
}

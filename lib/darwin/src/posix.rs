//! Darwin's implementation of the Portable Operating System Interface (POSIX). Its deployment in
//! macOS is registered as a [UNIX 03](https://www.opengroup.org/openbrand/register/apple.htm)
//! product.

#[cfg(feature = "experimental")]
pub mod fcntl;
pub mod unistd;

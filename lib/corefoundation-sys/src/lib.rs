//! Direct FFI bindings to Apple's `CoreFoundation` Clang module.
//!
//! The Clang module definition can be found in the iOS and macOS SDKs included with Xcode at
//! `$SDKROOT/System/Library/Frameworks/CoreFoundation.framework/Modules/module.modulemap`.

mod sys;

pub use sys::base::*;
pub use sys::string::*;
pub use sys::string_encoding_ext::*;

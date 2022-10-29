//! All the selectors used by this crate.

use objc4::selector;

/// Defines all the selectors used by this crate.
#[macro_export]
macro_rules! all_selectors {
    () => {
        selector!(COPY = "copy");
        selector!(INITWITHBYTES_LENGTH_ENCODING_ = "initWithBytes:length:encoding:");
        selector!(ISEQUALTOSTRING_ = "isEqualToString:");
        selector!(LENGTH = "length");
        selector!(UTF8STRING = "UTF8String");
    };
}

objc4::all_selectors!();
all_selectors!();

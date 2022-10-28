//! All the selectors used by this crate.

use crate::selector;

/// Defines all the selectors used by this crate.
#[macro_export]
macro_rules! all_selectors {
    () => {
        selector!(HASH = "hash");
        selector!(IS_EQUAL_ = "isEqual:");
        selector!(IS_PROXY = "isProxy");
        selector!(SUPERCLASS = "superclass");

        core::arch::global_asm!(
            "    .pushsection __DATA,__objc_imageinfo,regular,no_dead_strip",
            "L_OBJC_IMAGE_INFO:",
            "    .long    0",
            "    .long    0",
            "    .popsection",
        );
    };
}

all_selectors!();

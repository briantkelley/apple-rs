# dispatch

Execute code concurrently on multicore hardware by submitting work to dispatch queues managed by
the system.

This crate aims to proivde idiomatic Rust bindings to Apple's Dispatch Clang module (located at
`$SDKROOT/usr/include/dispatch/module.modulemap`), in the same way Apple's Swift overlay for
the [Dispatch framework](https://developer.apple.com/documentation/DISPATCH) provides idiomatic
Swift bindings.

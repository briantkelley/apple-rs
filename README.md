# apple-rs

Idiomatic Rust bindings for iOS and macOS system libraries and frameworks.

This repository aims to provide zero-overhead idiomatic bindings to iOS and macOS APIs, including
(eventually) Objective-C and Swift interfaces. See the [`experimental/objective-c-bindings`](https://github.com/briantkelley/apple-rs/tree/experimental/objective-c-bindings/)
branch for a potential approach for implementing Rust bindings to Objective-C interfaces.

## Crates

This repository contains multiple crates to simplify development as the bindings here are
implemented "on demand" which may necessarily require changes in multiple crates to add bindings for
any dependencies.

* [`darwin`](lib/darwin): Bindings to Apple's Darwin Clang module (located at `$SDKROOT/usr/include/module.modulemap`).
* [`dispatch`](lib/dispatch): Bindings to Apple's Dispatch Clang module (located at `$SDKROOT/usr/include/dispatch/module.modulemap`).
* [`os`](lib/os): Bindings to Apple's OS Clang module (located at `$SDKROOT/usr/include/module.modulemap`).

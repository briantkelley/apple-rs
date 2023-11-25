# apple-rs

Idiomatic Rust bindings for iOS and macOS system libraries and frameworks.

This repository aims to provide zero-overhead idiomatic bindings to iOS and macOS APIs, including
(eventually) Objective-C and Swift interfaces. See the [`experimental/objective-c-bindings`](https://github.com/briantkelley/apple-rs/tree/experimental/objective-c-bindings/)
branch for a potential approach for implementing Rust bindings to Objective-C interfaces.

## Crates

This repository contains multiple crates to simplify development. The bindings here are implemented
"on demand" which may necessarily require changes in multiple crates to implement the full
dependency graph.

**Idiomatic Rust Bindings:**

* [`darwin`](lib/darwin): Bindings to Apple's Darwin Clang module (located at `$SDKROOT/usr/include/module.modulemap`).
* [`dispatch`](lib/dispatch): Execute code concurrently on multicore hardware by submitting work to
  dispatch queues managed by the system.
* [`os`](lib/os): Bindings to Apple's OS Clang module (located at `$SDKROOT/usr/include/module.modulemap`).

**Raw Rust Bindings:**

* [`dispatch-sys`](lib/dispatch-sys): Raw Rust bindings to Apple's Dispatch Clang module (located at
  `$SDKROOT/usr/include/dispatch/module.modulemap`).

### Interface Stability

Most crates have an `experimental` feature that disables, by default, functions, modules, traits,
types, etc. that are still under active development and subject to change. Otherwise, the public
interface of each crate is expected to remain stable (i.e. source compatible).
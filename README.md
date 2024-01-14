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

* [`corefoundation`](lib/corefoundation) Core Foundation is a framework that provides fundamental
  software services useful to application services, application environments, and to applications
  themselves. Core Foundation also provides abstractions for common data types, facilitates
  internationalization with Unicode string storage, and offers a suite of utilities such as plug-in
  support, XML property lists, URL resource access, and preferences.
* [`dispatch`](lib/dispatch): Execute code concurrently on multicore hardware by submitting work to
  dispatch queues managed by the system.
* [`retain-release`](lib/retain-release): Support for building idiomatic Rust bindings for foreign
  heap-allocated, reference-counted objects. Used by [`corefoundation`](lib/corefoundation).

**Raw Rust Bindings:**

* [`corefoundation-sys`](lib/corefoundation-sys): Raw Rust bindings to Apple's CoreFoundation Clang
  module (located at `$SDKROOT/System/Library/Frameworks/CoreFoundation.framework/Modules/module.modulemap`).
* [`dispatch-sys`](lib/dispatch-sys): Raw Rust bindings to Apple's Dispatch Clang module (located at
  `$SDKROOT/usr/include/dispatch/module.modulemap`).

**Experimental Rust Bindings:**

* [`darwin`](lib/darwin): Bindings to Apple's Darwin Clang module (located at `$SDKROOT/usr/include/module.modulemap`).
* [`os`](lib/os): Bindings to Apple's OS Clang module (located at `$SDKROOT/usr/include/module.modulemap`).

Some crates have an `experimental` feature that disables, by default, functions, modules, traits,
types, etc. that are still under active development and subject to change. 

# Open Issues

Although the major version number of the crates in this repository is `0`, the public interfaces are
expected to remain stable (i.e. source compatible) as additional functionality is added. Unstable
interfaces require use of the `experimental` feature.

Before crates can be published with a major version of `1`, the active issues in the
[1.0.0 milestone](https://github.com/briantkelley/apple-rs/milestone/1) need to be resolved.

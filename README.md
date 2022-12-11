# apple-rs

Idiomatic Rust bindings for iOS and macOS system libraries and frameworks.

This repository aims to provide zero-overhead idiomatic bindings to iOS and macOS APIs, including
Objective-C and (eventually) Swift interfaces.

## Crates

This repository contains multiple crates to simplify development as the bindings here are
implemented "on demand" which may necessarily require changes in multiple crates to add bindings for
any dependencies.

* [`foundation`](lib/foundation): Bindings to the Foundation framework.
* [`objc4`](lib/objc4): Bindings to the Objective-C runtime and facilities to build idiomatic Rust
  interfaces for Objective-C interfaces.

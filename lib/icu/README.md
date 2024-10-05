# icu

This crate aims to provide idiomatic Rust bindings to Apple's ICU Clang module (located at
`$SDKROOT/usr/include/unicode/module.modulemap`, or in the Apple OSS Distributions
[ICU](https://github.com/apple-oss-distributions/ICU/blob/ICU-74000.403/modules/ICU.modulemap)
GitHub project), and aims to be a source-compatible, drop in replacement for various
[`unicode-rs`](https://github.com/unicode-rs) crates for the purpose of reducing binary size by
relying on system-provided Unicode databases.

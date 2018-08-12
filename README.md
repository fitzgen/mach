[![Build Status][travis_ci_badge]][travis_ci] [![Latest Version]][crates.io] [![docs]][docs.rs]

A Rust interface to the **user-space** API of the Mach 3.0 kernel exposed in
`/usr/include/mach` that underlies macOS.

This library does not expose the **kernel-space** API of the Mach 3.0 kernel
exposed in
`SDK/System/Library/Frameworks/Kernel.framework/Versions/A/Headers/mach`. 

That is, if you are writing a kernel-resident device drivers or some other
kernel extensions you have to use something else. The user-space kernel API is
often API-incompatible with the kernel space one, and even in the cases where
they match, they are sometimes ABI incompatible such that using this library
would have **undefined behavior**.

# Platform support

The following table describes the current CI set-up:

| Target                | Min. Rust |   XCode   | build | ctest | run |
|-----------------------|-----------|-----------|-------|-------|-----|
| `x86_64-apple-darwin` |  1.13.0   | 6.4 - 9.4 | ✓     | ✓     | ✓   | 
| `i686-apple-darwin`   |  1.13.0   | 6.4 - 9.4 | ✓     | ✓     | ✓   |
| `i386-apple-ios`      |  1.13.0   | 6.4 - 9.2 | ✓     | ✓ [0] | -   |
| `x86_64-apple-ios`    |  1.13.0   | 6.4 - 9.2 | ✓     | ✓ [0] | -   |
| `armv7-apple-ios`     |  nightly  | 6.4 - 9.4 | ✓     | -     | -   |
| `aarch64-apple-ios`   |  nightly  | 6.4 - 9.4 | ✓     | -     | -   |

[0] `ctest` is only run on iOS for XCode 8.3 version and newer.

[travis_ci]: https://travis-ci.org/fitzgen/mach
[travis_ci_badge]: https://travis-ci.org/fitzgen/mach.png?branch=master
[crates.io]: https://crates.io/crates/mach
[Latest Version]: https://img.shields.io/crates/v/mach.svg
[docs]: https://docs.rs/mach/badge.svg
[docs.rs]: https://docs.rs/mach/


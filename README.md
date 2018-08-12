[![Build Status](https://travis-ci.org/fitzgen/mach.png?branch=master)](https://travis-ci.org/fitzgen/mach)

A rust interface to the Mach 3.0 kernel that underlies macOS.

# Platform support

The following table describes the current CI set-up:

| Target                | Min. Rust |   XCode   | build | ctest | run |
|-----------------------|-----------|-----------|-------|-------|-----|
| `x86_64-apple-darwin` |  1.13.0   | 6.4 - 9.4 | ✓     | ✓     | ✓   | 
| `i686-apple-darwin`   |  1.13.0   | 6.4 - 9.4 | ✓     | ✓     | ✓   |
| `i386-apple-ios`      |  1.13.0   | 6.4 - 9.4 | ✓     | ✓ [0] | -   |
| `x86_64-apple-ios`    |  1.13.0   | 6.4 - 9.4 | ✓     | ✓ [0] | -   |
| `armv7-apple-ios`     |  nightly  | 6.4 - 9.4 | ✓     | -     | -   |
| `aarch64-apple-ios`   |  nightly  | 6.4 - 9.4 | ✓     | -     | -   |

[0] `ctest` is only run on iOS for XCode 8.3 version and newer.

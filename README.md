[![Build Status](https://travis-ci.org/fitzgen/mach.png?branch=master)](https://travis-ci.org/fitzgen/mach)

A rust interface to the Mach 3.0 kernel that underlies OSX.

# Platform support

The following table describes the current CI set-up:

| Target                | Min. Rust |   XCode   | build | ctest | run |
|-----------------------|-----------|-----------|-------|-------|-----|
| `x86_64-apple-darwin` |  1.11.0   | 6.4 - 9.2 | ✓     | ✓     | ✓   | 
| `i686-apple-darwin`   |  1.11.0   | 6.4 - 9.2 | ✓     | ✓     | ✓   |
| `i386-apple-ios`      |  1.11.0   | 6.4 - 9.2 | ✓     | ✓ [0] | -   |
| `x86_64-apple-ios`    |  1.11.0   | 6.4 - 9.2 | ✓     | ✓ [0] | -   |
| `armv7-apple-ios`     |  nightly  | 6.4 - 9.2 | ✓     | -     | -   |
| `aarch64-apple-ios`   |  nigthly  | 6.4 - 9.2 | ✓     | -     | -   |

[0] `ctest` is only run on iOS for XCode 8.3 version and newer.

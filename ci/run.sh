#!/usr/bin/env bash

set -ex

: ${TARGET?"The TARGET environment variable must be set."}
: ${TRAVIS_RUST_VERSION?"The TRAVIS_RUST_VERSION environment variable must be set."}

echo "Running tests for target: ${TARGET}"
export RUST_BACKTRACE=1
export RUST_TEST_THREADS=1
export RUST_TEST_NOCAPTURE=1

if [[ $TARGET == *"ios"* ]]; then
    export RUSTFLAGS='-C link-args=-mios-simulator-version-min=7.0'
    rustc ./ci/deploy_and_run_on_ios_simulator.rs -o ios_cargo_runner --verbose
    if [[ $TARGET == "x86_64-apple-ios" ]]; then
        export CARGO_TARGET_X86_64_APPLE_IOS_RUNNER=$(pwd)/ios_cargo_runner
    fi
    if [[ $TARGET == "i386-apple-ios" ]]; then
        export CARGO_TARGET_I386_APPLE_IOS_RUNNER=$(pwd)/ios_cargo_runner
    fi
fi

rustup target add $TARGET || true

# Build w/o std
cargo clean
cargo build --target $TARGET --verbose 2>&1 | tee build_std.txt
cargo build --no-default-features --target $TARGET --verbose 2>&1 | tee build_no_std.txt

# Check that the no-std builds are not linked against a libc with default
# features or the use_std feature enabled:
cat build_std.txt | grep -q "default"
cat build_std.txt | grep -q "use_std"
! cat build_no_std.txt | grep -q "default"
! cat build_no_std.txt | grep -q "use_std"
# Make sure that the resulting build contains no std symbols
! find target/ -name *.rlib -exec nm {} \; | grep "std"

# Runs mach's run-time tests:
if [[ -z "$NORUN" ]]; then
    cargo test --target $TARGET --verbose
    cargo test --no-default-features --target $TARGET --verbose
fi

# Runs ctest to verify mach's ABI against the system libraries:
if [[ -z "$NOCTEST" ]]; then
    if [[ $TRAVIS_RUST_VERSION == "nightly" ]]; then
        cargo test --manifest-path mach-test/Cargo.toml --target $TARGET --verbose
        cargo test --no-default-features --manifest-path mach-test/Cargo.toml --target $TARGET --verbose
    fi
fi

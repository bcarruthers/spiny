# Spiny

# Building

## Cross-compiling

Issues with Tracy:
https://github.com/nagisa/rust_tracy_client/issues/31

Prerequisites:
``rustup target add x86_64-pc-windows-gnu``
``rustup toolchain install stable-x86_64-pc-windows-gnu``
``sudo port install x86_64-w64-mingw32-gcc``

Building for Windows:
``cargo build --target x86_64-pc-windows-gnu --release --bin tv``

## Bundling

Prerequisite:
``cargo install cargo-bundle``

# Running

## Desktop

Run (debug) with logging on MacOS:
``RUST_BACKTRACE=1 RUST_LOG="info,wgpu_core=warn,wgpu_hal=warn,naga=warn" cargo run``

Run (debug) with logging and Tracy on MacOS:
``RUST_TRACY=1 RUST_BACKTRACE=1 RUST_LOG="info,wgpu_core=warn,wgpu_hal=warn,naga=warn" cargo run``

Set logging on Windows:
``$env:RUST_LOG='info'``

Set nightly:
``rustup override set nightly``

Pack assets:
``cargo run --bin util -- pack assets scratch/assets.zip``

## WebAssembly

Run these from crates/tv_wasm folder. To change log level, change this line: console_log::init_with_level(log::Level::Info)

Note that startup can fail and logging not shown (failure happens after log line, but log lines
before failure don't show up, so it looks like the program didn't go past a certain point when it
did). Async-related?

Install build target:
``rustup target add wasm32-unknown-unknown``

Install wasm-pack (OpenSSL install needed for Windows):
``cargo install wasm-pack``

Install http server:
``cargo install basic-http-server``

Run (if '--target web' excluded, then wasm will be generated and may be blocked by the browser as it isn't standardized yet, if --debug excluded, then no debug symbols will be present and stack traces will be indecipherable, if unstable flag not included, then clipboard won't be available):
``RUSTFLAGS=--cfg=web_sys_unstable_apis wasm-pack build --target web --debug``
``basic-http-server``

Navigate to http://localhost:4000/

## Profiling

Make sure to include symbols in Cargo.toml when profiling release.

Profile release with flamegraph:
``sudo CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph --bin=tv --output scratch/flame-release.svg``

Profile release with flamegraph (no extra debug info):
``sudo cargo flamegraph --bin=tv --output scratch/flame-release.svg``

Profile dev with flamegraph:
``sudo cargo flamegraph --dev --bin=tv --output scratch/flame-dev.svg``

Profile dev stress test with flamegraph:
``sudo cargo flamegraph --dev --bin=tv_util --output scratch/flame-dev.svg -- test stress``

# Measuring

Check bloat:
``cargo bloat --crates --release --bin tv -n 50``

Check dependencies:
``cargo tree``

Count lines of code:
``loc --exclude fyrox``

Check build times:
``cargo build --timings``

# Git

## Checklist

- cargo test
- cargo fmt
- Test release build on Windows, MacOS, Firefox

## Tagging

Examples:
git tag
git tag -a v0.3.0 -m "v0.3.0"
git tag -a v0.3.0 9fceb02
git status
git push origin v0.3.0
git push --delete origin v0.3.0
git tag --delete v0.3.0

# Assets

## Asset loading

Options for loading at runtime:
- Embedded zip file
- Separate zip file
- Folder

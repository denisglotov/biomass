# Biomass - Rust & WebAssembly Task Runner

default:
    @just --list

# Build native desktop debug binary
build:
    cargo build

# Run the native app
run:
    cargo run

# Build release WebAssembly target and copy WASM binary to workspace root
build-wasm:
    cargo build --target wasm32-unknown-unknown --release
    cp target/wasm32-unknown-unknown/release/biomass.wasm ./biomass.wasm

# Build android image
build-android:
    cargo quad-apk build --release

# Check for compilation errors
check:
    cargo check

# Run Clippy linter with strict warning checks
clippy:
    cargo clippy -- -D warnings

# Format code using rustfmt
fmt:
    cargo fmt

# Check formatting without making changes
fmt-check:
    cargo fmt --check

# Run tests
test:
    cargo test

# Serve the WASM game locally on port 8080
serve: build-wasm
    python3 -m http.server 8080

# Run complete CI test suite (formatting, clippy, tests, WASM build)
ci: fmt-check clippy test build-wasm

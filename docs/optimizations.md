# Build Optimizations

This document outlines the optimizations applied to the release builds of this project to reduce the WebAssembly (`.wasm`) file size.

## Release Build Configuration

Release builds are configured in `Cargo.toml` to optimize for size. The following settings are used under `[profile.release]`:

- `lto = true`: Enables Link Time Optimization, which allows the compiler to perform optimizations across the entire crate.
- `opt-level = "z"`: Optimizes for size, even if it means sacrificing some performance.

## wasm-opt Integration

The `trunk` build tool is configured to use `wasm-opt` to further optimize the `.wasm` file. This is enabled in `index.html`:

```html
<link data-trunk rel="rust" data-wasm-opt="z" />
```

The `data-wasm-opt="z"` attribute tells `trunk` to run `wasm-opt` with the most aggressive size optimization settings.

## Build Commands

A `Makefile` target is provided to create a release build:

```bash
make build-release
```

This command runs `trunk build --release`, which compiles the Rust code in release mode and applies the `wasm-opt` optimizations.

## Expected File Size Reduction

Running a release build significantly reduces the size of the `.wasm` file compared to a debug build.

- **Debug build `*.wasm` size:** ~11.5 MB
- **Release build `*.wasm` size:** ~1.8 MB

This represents a reduction of over 84%.

# rust-learning-project

[![Crates.io](https://img.shields.io/crates/v/rust-learning-project.svg)](https://crates.io/crates/rust-learning-project)
[![Docs.rs](https://docs.rs/rust-learning-project/badge.svg)](https://docs.rs/rust-learning-project)
[![CI](https://github.com/fderuiter/rust-learning-project/workflows/CI/badge.svg)](https://github.com/fderuiter/rust-learning-project/actions)

## Installation

### Development Environment Setup

For a quick setup of your development environment, run the provided script. This will ensure you have the necessary WebAssembly build target installed.

```bash
./setup_dev.sh
```

If you prefer a manual setup, follow the instructions below.

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install rust-learning-project`

### WebAssembly Target

* Install the WebAssembly build target, which is required for compiling to Wasm:
  ```bash
  rustup target add wasm32-unknown-unknown
  ```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).

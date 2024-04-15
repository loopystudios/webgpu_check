<div align="center">

# WebGPU Check

**A simple, minimal crate to probe WebGPU support prior to running your application.**

[![Discord](https://img.shields.io/discord/913957940560531456.svg?label=Loopy&logo=discord&logoColor=ffffff&color=ffffff&labelColor=000000)](https://discord.gg/zrjnQzdjCB)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#license)
[![Build status](https://github.com/loopystudios/webgpu_check/workflows/CI/badge.svg)](https://github.com/loopystudios/webgpu_check/actions)
[![dependency status](https://deps.rs/repo/github/loopystudios/webgpu_check/status.svg)](https://deps.rs/repo/github/loopystudios/webgpu_check)
[![Crates.io](https://img.shields.io/crates/v/webgpu_check.svg)](https://crates.io/crates/webgpu_check)
[![Docs](https://img.shields.io/docsrs/webgpu_check)](https://docs.rs/webgpu_check)

</div>

Quickstart to run demo:

- **Web**

```shell
# Make sure the Rust toolchain supports the wasm32 target
rustup target add wasm32-unknown-unknown

# Install `wasm-server-runner` for the example
cargo install wasm-server-runner

cargo run --target wasm32-unknown-unknown --example simple
```

- **Native**

```shell
cargo run --example simple
```

There is also a web demo [available here](https://loopystudios.github.io/webgpu_check).

## Usage

One can simply check, at the beginning of their application, whether WebGPU is supported, and react accordingly.

```rust
fn main() {
    if !webgpu_check::is_webgpu_available() {
        panic!("This platform doesn't support WebGPU!");
    }
    // Proceed to run your WebGPU application...
}
```

## Community

All Loopy projects and development happens in the [Loopy Discord](https://discord.gg/zrjnQzdjCB). The discord is open to the public.

Contributions are welcome by pull request. The [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct) applies.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

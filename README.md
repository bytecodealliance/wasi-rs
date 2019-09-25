# WASI API Bindings for Rust

This package contains experimental [WASI](https://github.com/WebAssembly/WASI)
API bindings in Rust.

There are two top-level modules, `old` and `unstable`, which correspond to
the `old` and `unstable` directories in
[the phases directory](https://github.com/WebAssembly/WASI/tree/master/phases)
in the official WASI repository.

Within each are submodules corresponding to the specified API modules.

The top level of each of these submodules contains idiomatic Rust types and
interfaces exposing the low-level WASI functionality. These submodules each
contain a `raw` submodule which exposes the raw API functions.

This crate is quite low-level and provides conceptually a "system call"
interface. In most settings, it's better to use the Rust standard library,
which has WASI support.

To compile Rust projects to wasm using WASI, use the `wasm32-wasi` target,
like this:

```
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
```

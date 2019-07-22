# WASI API Bindings for Rust

This package contains experimental [WASI](https://github.com/WebAssembly/WASI)
API bindings in Rust.

There are two modules:

 - `wasi_unstable::raw`, which provides raw access to the literal binding to
   the API. These functions are unsafe and use raw pointers.

 - `wasi_unstable`, which provides thin wrappers around the raw functions
   which use idiomatic Rust types rather than raw pointers, and are safe.

This crate is quite low-level and provides conceptually a "system call"
interface. In most settings, it's better to use the Rust standard library,
which has WASI support.

To compile Rust projects to wasm using WASI, use the `wasm32-wasi` target,
like this:

```
rustup target add wasm32-wasi
cargo build --target wasm32-wasi
```

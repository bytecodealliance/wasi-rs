# wasi-core

This package contains experimental [WASI](https://github.com/WebAssembly/WASI)
API bindings in Rust.

There are two modules:

 - `wasi_unstable::raw`, which provides raw access to the literal binding to
   the API. These functions are unsafe and use raw pointers.

 - `wasi_unstable`, which provides thin wrappers around the raw functions
   which use idiomatic Rust types rather than raw pointers, and are safe.

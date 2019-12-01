This crate contains API bindings for the [ephemeral
phase](https://github.com/WebAssembly/WASI/blob/master/phases/README.md) of
[WASI](https://github.com/WebAssembly/WASI). The ephemeral phase exists for
experimentation prior to snapshotting.

This crate will never be released, and should only be used by path dependency
or git dependency. By default, this crate provides bindings for the ephemeral
phase from the included git submodule of the [WASI
repository](https://github.com/WebAssembly/WASI), but you can set the
`WASI_EPHEMERAL_WITX` environment variable to the full path to a witx file to
build bindings for that witx file instead.

This crate makes it easier to test bindings to arbitrary witx interfaces. Note
that WebAssembly runtimes typically do not implement the ephemeral phase of
WASI, so a WebAssembly program built against the default wasi-ephemeral may not
load. Instead, for development, you may want to build with
`WASI_EPHEMERAL_WITX` pointing to an edited version of the latest snapshot, and
a correspondingly modified version of a WebAssembly runtime.

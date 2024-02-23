#!/bin/sh

set -ex

wit-bindgen rust wit --out-dir src --std-feature --type-section-suffix rust-wasi-from-crates-io

# rustfmt chokes on the raw output of wit-bindgen right now due to trailling
# whitespace (unsure as to why), so format it with some options first to get it
# into a better state before applying the final format with default options
# which gets this to succeed.
#
# NB: this should be considered a bug in wit-bindgen that this is required to do
# twice. Passing `--rustfmt` to `wit-bindgen` should work.
rustfmt src/bindings.rs --edition 2021 --config-path ./ci/rustfmt-bindings.toml
rustfmt src/bindings.rs --edition 2021

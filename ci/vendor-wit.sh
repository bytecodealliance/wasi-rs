#!/usr/bin/env bash

# Script to re-vendor the WIT files that wasi-rs uses as defined by a
# particular tag in upstream repositories.
#
# This script is executed on CI to ensure that everything is up-to-date.
set -ex


rm -rf crates/wasip2/wit/deps
mkdir -p crates/wasip2/wit/deps

p2=0.2.9
wkg get --overwrite --format wit "wasi:cli@${p2}" -o crates/wasip2/wit/deps/cli.wit
wkg get --overwrite --format wit "wasi:clocks@${p2}" -o crates/wasip2/wit/deps/clocks.wit
wkg get --overwrite --format wit "wasi:filesystem@${p2}" -o crates/wasip2/wit/deps/filesystem.wit
wkg get --overwrite --format wit "wasi:http@${p2}" -o crates/wasip2/wit/deps/http.wit
wkg get --overwrite --format wit "wasi:io@${p2}" -o crates/wasip2/wit/deps/io.wit
wkg get --overwrite --format wit "wasi:random@${p2}" -o crates/wasip2/wit/deps/random.wit
wkg get --overwrite --format wit "wasi:sockets@${p2}" -o crates/wasip2/wit/deps/sockets.wit

rm -rf crates/wasip3/wit/deps
mkdir -p crates/wasip3/wit/deps

p3=0.3.0-rc-2026-01-06
wkg get --overwrite --format wit "wasi:cli@${p3}" -o crates/wasip3/wit/deps/cli.wit
wkg get --overwrite --format wit "wasi:clocks@${p3}" -o crates/wasip3/wit/deps/clocks.wit
wkg get --overwrite --format wit "wasi:filesystem@${p3}" -o crates/wasip3/wit/deps/filesystem.wit
wkg get --overwrite --format wit "wasi:http@${p3}" -o crates/wasip3/wit/deps/http.wit
wkg get --overwrite --format wit "wasi:random@${p3}" -o crates/wasip3/wit/deps/random.wit
wkg get --overwrite --format wit "wasi:sockets@${p3}" -o crates/wasip3/wit/deps/sockets.wit

# WASIp1 vendoring logic
wasip1_rev="0ba0c5e2"
curl -o crates/wasip1/typenames.witx -L \
  https://raw.githubusercontent.com/WebAssembly/WASI/$wasip1_rev/phases/snapshot/witx/typenames.witx
curl -o crates/wasip1/wasi_snapshot_preview1.witx -L \
  https://raw.githubusercontent.com/WebAssembly/WASI/$wasip1_rev/phases/snapshot/witx/wasi_snapshot_preview1.witx

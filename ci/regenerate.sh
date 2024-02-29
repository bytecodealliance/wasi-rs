#!/bin/sh

set -ex

generate() {
  file="$1"
  shift
  wit-bindgen rust wit --out-dir src --std-feature "$@"

  # rustfmt chokes on the raw output of wit-bindgen right now due to trailling
  # whitespace (unsure as to why), so format it with some options first to get it
  # into a better state before applying the final format with default options
  # which gets this to succeed.
  #
  # NB: this should be considered a bug in wit-bindgen that this is required to do
  # twice. Passing `--rustfmt` to `wit-bindgen` should work.
  rustfmt $file --edition 2021 --config-path ./ci/rustfmt-bindings.toml
  rustfmt $file --edition 2021
}

# Generate the main body of the bindings which includes all imports from the two
# worlds below.
generate src/bindings.rs --type-section-suffix rust-wasi-from-crates-io

# Generate bindings for the `wasi:cli/command` world specifically, namely the
# macro `export_command`.
#
# Note that `--with` is used to point at the previously generated bindings.
with="wasi:cli/environment=crate::cli::environment"
with="$with,wasi:cli/exit=crate::cli::exit"
with="$with,wasi:cli/stdin=crate::cli::stdin"
with="$with,wasi:cli/stdout=crate::cli::stdout"
with="$with,wasi:cli/stderr=crate::cli::stderr"
with="$with,wasi:cli/terminal-input=crate::cli::terminal_input"
with="$with,wasi:cli/terminal-output=crate::cli::terminal_output"
with="$with,wasi:cli/terminal-stdin=crate::cli::terminal_stdin"
with="$with,wasi:cli/terminal-stdout=crate::cli::terminal_stdout"
with="$with,wasi:cli/terminal-stderr=crate::cli::terminal_stderr"
with="$with,wasi:clocks/monotonic-clock=crate::clocks::monotonic_clock"
with="$with,wasi:clocks/wall-clock=crate::clocks::wall_clock"
with="$with,wasi:filesystem/types=crate::filesystem::types"
with="$with,wasi:filesystem/preopens=crate::filesystem::preopens"
with="$with,wasi:io/error=crate::io::error"
with="$with,wasi:io/poll=crate::io::poll"
with="$with,wasi:io/streams=crate::io::streams"
with="$with,wasi:random/random=crate::random::random"
with="$with,wasi:random/insecure=crate::random::insecure"
with="$with,wasi:random/insecure-seed=crate::random::insecure_seed"
with="$with,wasi:sockets/network=crate::sockets::network"
with="$with,wasi:sockets/instance-network=crate::sockets::instance_network"
with="$with,wasi:sockets/tcp=crate::sockets::tcp"
with="$with,wasi:sockets/tcp-create-socket=crate::sockets::tcp_create_socket"
with="$with,wasi:sockets/udp=crate::sockets::udp"
with="$with,wasi:sockets/udp-create-socket=crate::sockets::udp_create_socket"
with="$with,wasi:sockets/ip-name-lookup=crate::sockets::ip_name_lookup"
generate src/command.rs \
  --world wasi:cli/command \
  --with "$with" \
  --type-section-suffix rust-wasi-from-crates-io-command-world \
  --default-bindings-module wasi \
  --pub-export-macro \
  --export-macro-name export_command

# Same as the `command` world, but for the proxy world.
with="wasi:cli/stdin=crate::cli::stdin"
with="$with,wasi:cli/stdout=crate::cli::stdout"
with="$with,wasi:cli/stderr=crate::cli::stderr"
with="$with,wasi:clocks/monotonic-clock=crate::clocks::monotonic_clock"
with="$with,wasi:clocks/wall-clock=crate::clocks::wall_clock"
with="$with,wasi:io/error=crate::io::error"
with="$with,wasi:io/poll=crate::io::poll"
with="$with,wasi:io/streams=crate::io::streams"
with="$with,wasi:random/random=crate::random::random"
with="$with,wasi:http/types=crate::http::types"
with="$with,wasi:http/outgoing-handler=crate::http::outgoing_handler"
generate src/proxy.rs \
  --world wasi:http/proxy \
  --with "$with" \
  --type-section-suffix rust-wasi-from-crates-io-proxy-world \
  --default-bindings-module wasi \
  --pub-export-macro \
  --export-macro-name export_proxy

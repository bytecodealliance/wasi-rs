#!/bin/sh

set -ex

suffix() {
    name="$1"
    extra="$2"
    cargo metadata | jq -r \
        ".packages.[] | select(.name==\"${name}\") | (\"rust-\" + .name + \"-\" + .version + \"-from-crates-io${extra}\")"
}

generate() {
  file="$1"
  shift
  wit-bindgen rust "$@" --format
}

# ==============================================================================
# WASIp2 bindings

generate_p2() {
  file="$1"

  generate "$@" --std-feature ./crates/wasip2/wit --out-dir crates/wasip2/src

  sed -z -i 's/#\[unsafe(\n    link_section = "\(.*\)"\n)\]/\
#[cfg_attr(feature = "rustc-dep-of-std", unsafe(link_section = "\1-in-libstd"))]\
#[cfg_attr(not(feature = "rustc-dep-of-std"), unsafe(link_section = "\1"))]\
/' $file
}

# Generate the main body of the bindings which includes all imports from the two
# worlds below.
generate_p2 crates/wasip2/src/imports.rs \
  --type-section-suffix $(suffix "wasip2") \
  --generate-all \
  --world wasi:cli/imports

p2=0.2.9
p3=0.3.0-rc-2026-01-06

# Generate bindings for the `wasi:cli/command` world specifically, namely the
# macro `export_command`.
#
# Note that `--with` is used to point at the previously generated bindings.
with="wasi:cli/environment@$p2=crate::cli::environment"
with="$with,wasi:cli/exit@$p2=crate::cli::exit"
with="$with,wasi:cli/stdin@$p2=crate::cli::stdin"
with="$with,wasi:cli/stdout@$p2=crate::cli::stdout"
with="$with,wasi:cli/stderr@$p2=crate::cli::stderr"
with="$with,wasi:cli/terminal-input@$p2=crate::cli::terminal_input"
with="$with,wasi:cli/terminal-output@$p2=crate::cli::terminal_output"
with="$with,wasi:cli/terminal-stdin@$p2=crate::cli::terminal_stdin"
with="$with,wasi:cli/terminal-stdout@$p2=crate::cli::terminal_stdout"
with="$with,wasi:cli/terminal-stderr@$p2=crate::cli::terminal_stderr"
with="$with,wasi:clocks/monotonic-clock@$p2=crate::clocks::monotonic_clock"
with="$with,wasi:clocks/wall-clock@$p2=crate::clocks::wall_clock"
with="$with,wasi:filesystem/types@$p2=crate::filesystem::types"
with="$with,wasi:filesystem/preopens@$p2=crate::filesystem::preopens"
with="$with,wasi:io/error@$p2=crate::io::error"
with="$with,wasi:io/poll@$p2=crate::io::poll"
with="$with,wasi:io/streams@$p2=crate::io::streams"
with="$with,wasi:random/random@$p2=crate::random::random"
with="$with,wasi:random/insecure@$p2=crate::random::insecure"
with="$with,wasi:random/insecure-seed@$p2=crate::random::insecure_seed"
with="$with,wasi:sockets/network@$p2=crate::sockets::network"
with="$with,wasi:sockets/instance-network@$p2=crate::sockets::instance_network"
with="$with,wasi:sockets/tcp@$p2=crate::sockets::tcp"
with="$with,wasi:sockets/tcp-create-socket@$p2=crate::sockets::tcp_create_socket"
with="$with,wasi:sockets/udp@$p2=crate::sockets::udp"
with="$with,wasi:sockets/udp-create-socket@$p2=crate::sockets::udp_create_socket"
with="$with,wasi:sockets/ip-name-lookup@$p2=crate::sockets::ip_name_lookup"
generate_p2 crates/wasip2/src/command.rs \
  --world wasi:cli/command \
  --with "$with" \
  --type-section-suffix $(suffix "wasip2" "-command-world") \
  --default-bindings-module '$crate' \
  --pub-export-macro \
  --export-macro-name _export_command

# Same as the `command` world, but for the proxy world.
with="wasi:cli/stdin@$p2=crate::cli::stdin"
with="$with,wasi:cli/stdout@$p2=crate::cli::stdout"
with="$with,wasi:cli/stderr@$p2=crate::cli::stderr"
with="$with,wasi:clocks/monotonic-clock@$p2=crate::clocks::monotonic_clock"
with="$with,wasi:clocks/wall-clock@$p2=crate::clocks::wall_clock"
with="$with,wasi:io/error@$p2=crate::io::error"
with="$with,wasi:io/poll@$p2=crate::io::poll"
with="$with,wasi:io/streams@$p2=crate::io::streams"
with="$with,wasi:random/random@$p2=crate::random::random"
generate_p2 crates/wasip2/src/proxy.rs \
  --world wasi:http/proxy \
  --with "$with" \
  --type-section-suffix $(suffix "wasip2" "-proxy-world") \
  --default-bindings-module '$crate' \
  --pub-export-macro \
  --export-macro-name _export_proxy

# ==============================================================================
# WASIp3 bindings

generate_p3() {
  generate "$@" ./crates/wasip3/wit --out-dir crates/wasip3/src
}

generate_p3 crates/wasip3/src/imports.rs \
  --type-section-suffix $(suffix "wasip3") \
  --generate-all \
  --world wasi:cli/imports

with="wasi:cli/environment@$p3=crate::cli::environment"
with="$with,wasi:cli/exit@$p3=crate::cli::exit"
with="$with,wasi:cli/stdin@$p3=crate::cli::stdin"
with="$with,wasi:cli/stdout@$p3=crate::cli::stdout"
with="$with,wasi:cli/stderr@$p3=crate::cli::stderr"
with="$with,wasi:cli/types@$p3=crate::cli::types"
with="$with,wasi:cli/terminal-input@$p3=crate::cli::terminal_input"
with="$with,wasi:cli/terminal-output@$p3=crate::cli::terminal_output"
with="$with,wasi:cli/terminal-stdin@$p3=crate::cli::terminal_stdin"
with="$with,wasi:cli/terminal-stdout@$p3=crate::cli::terminal_stdout"
with="$with,wasi:cli/terminal-stderr@$p3=crate::cli::terminal_stderr"
with="$with,wasi:clocks/monotonic-clock@$p3=crate::clocks::monotonic_clock"
with="$with,wasi:clocks/system-clock@$p3=crate::clocks::system_clock"
with="$with,wasi:clocks/types@$p3=crate::clocks::types"
with="$with,wasi:filesystem/types@$p3=crate::filesystem::types"
with="$with,wasi:filesystem/preopens@$p3=crate::filesystem::preopens"
with="$with,wasi:random/random@$p3=crate::random::random"
with="$with,wasi:random/insecure@$p3=crate::random::insecure"
with="$with,wasi:random/insecure-seed@$p3=crate::random::insecure_seed"
with="$with,wasi:sockets/types@$p3=crate::sockets::types"
with="$with,wasi:sockets/ip-name-lookup@$p3=crate::sockets::ip_name_lookup"
generate_p3 crates/wasip3/src/command.rs \
  --world wasi:cli/command \
  --with "$with" \
  --type-section-suffix $(suffix "wasip3" "-command-world") \
  --default-bindings-module '$crate' \
  --pub-export-macro \
  --export-macro-name _export_command

with="wasi:cli/stdin@$p3=crate::cli::stdin"
with="$with,wasi:cli/stdout@$p3=crate::cli::stdout"
with="$with,wasi:cli/stderr@$p3=crate::cli::stderr"
with="$with,wasi:cli/types@$p3=crate::cli::types"
with="$with,wasi:clocks/monotonic-clock@$p3=crate::clocks::monotonic_clock"
with="$with,wasi:clocks/system-clock@$p3=crate::clocks::system_clock"
with="$with,wasi:clocks/types@$p3=crate::clocks::types"
with="$with,wasi:random/random@$p3=crate::random::random"
with="$with,wasi:random/insecure@$p3=crate::random::insecure"
with="$with,wasi:random/insecure-seed@$p3=crate::random::insecure_seed"
generate_p3 crates/wasip3/src/service.rs \
  --world wasi:http/service \
  --with "$with" \
  --type-section-suffix $(suffix "wasip3" "-service-world") \
  --default-bindings-module '$crate' \
  --pub-export-macro \
  --export-macro-name _export_service

fn main() {
    #[cfg(feature = "macros")]
    generate_macros();
}

#[cfg(feature = "macros")]
fn generate_macros() {
    use std::path::Path;

    let crate_root = std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let wit_path = Path::new(&crate_root)
        .join("wit")
        .to_str()
        .expect("project path must be valid UTF-8")
        .to_string();

    fn write_macro(filename: &str, contents: impl ToString) {
        let out_dir = std::env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join(filename);
        std::fs::write(dest_path, contents.to_string()).unwrap();
    }

    write_macro(
        "cli_run_export.rs",
        quote::quote! {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! cli_run_export {
                ($export_impl:path) => {
                    ::wasi::macros::wit_bindgen::generate!({
                        path: #wit_path,
                        world: "wasi:cli/command",
                        exports: {
                            "wasi:cli/run": $export_impl,
                        },
                        with: {
                            "wasi:cli/environment@0.2.0": ::wasi::cli::environment,
                            "wasi:cli/exit@0.2.0": ::wasi::cli::exit,
                            "wasi:cli/stderr@0.2.0": ::wasi::cli::stderr,
                            "wasi:cli/stdin@0.2.0": ::wasi::cli::stdin,
                            "wasi:cli/stdout@0.2.0": ::wasi::cli::stdout,
                            "wasi:cli/terminal-input@0.2.0": ::wasi::cli::terminal_input,
                            "wasi:cli/terminal-output@0.2.0": ::wasi::cli::terminal_output,
                            "wasi:cli/terminal-stderr@0.2.0": ::wasi::cli::terminal_stderr,
                            "wasi:cli/terminal-stdin@0.2.0": ::wasi::cli::terminal_stdin,
                            "wasi:cli/terminal-stdout@0.2.0": ::wasi::cli::terminal_stdout,
                            "wasi:clocks/monotonic-clock@0.2.0": ::wasi::clocks::monotonic_clock,
                            "wasi:clocks/wall-clock@0.2.0": ::wasi::clocks::wall_clock,
                            "wasi:filesystem/preopens@0.2.0": ::wasi::filesystem::preopens,
                            "wasi:filesystem/types@0.2.0": ::wasi::filesystem::types,
                            "wasi:io/error@0.2.0": ::wasi::io::error,
                            "wasi:io/poll@0.2.0": ::wasi::io::poll,
                            "wasi:io/streams@0.2.0": ::wasi::io::streams,
                            "wasi:random/insecure-seed@0.2.0": ::wasi::random::insecure_seed,
                            "wasi:random/insecure@0.2.0": ::wasi::random::insecure,
                            "wasi:random/random@0.2.0": ::wasi::random::random,
                            "wasi:sockets/instance-network@0.2.0": ::wasi::sockets::instance_network,
                            "wasi:sockets/ip-name-lookup@0.2.0": ::wasi::sockets::ip_name_lookup,
                            "wasi:sockets/network@0.2.0": ::wasi::sockets::network,
                            "wasi:sockets/tcp-create-socket@0.2.0": ::wasi::sockets::tcp_create_socket,
                            "wasi:sockets/tcp@0.2.0": ::wasi::sockets::tcp,
                            "wasi:sockets/udp-create-socket@0.2.0": ::wasi::sockets::udp_create_socket,
                            "wasi:sockets/udp@0.2.0": ::wasi::sockets::udp,
                        },
                        runtime_path: "::wasi::macros::wit_bindgen::rt",
                    });
                }
            }
        },
    );

    write_macro(
        "http_incoming_handler_export.rs",
        quote::quote! {
            #[doc(hidden)]
            #[macro_export]
            macro_rules! http_incoming_handler_export {
                ($export_impl:path) => {
                    ::wasi::macros::wit_bindgen::generate!({
                        path: #wit_path,
                        world: "wasi:http/proxy",
                        exports: {
                            "wasi:http/incoming-handler": $export_impl,
                        },
                        with: {
                            "wasi:cli/stderr@0.2.0": ::wasi::cli::stderr,
                            "wasi:cli/stdin@0.2.0": ::wasi::cli::stdin,
                            "wasi:cli/stdout@0.2.0": ::wasi::cli::stdout,
                            "wasi:clocks/monotonic-clock@0.2.0": ::wasi::clocks::monotonic_clock,
                            "wasi:clocks/wall-clock@0.2.0": ::wasi::clocks::wall_clock,
                            "wasi:http/outgoing-handler@0.2.0": ::wasi::http::outgoing_handler,
                            "wasi:http/types@0.2.0": ::wasi::http::types,
                            "wasi:io/error@0.2.0": ::wasi::io::error,
                            "wasi:io/poll@0.2.0": ::wasi::io::poll,
                            "wasi:io/streams@0.2.0": ::wasi::io::streams,
                            "wasi:random/random@0.2.0": ::wasi::random::random,
                        },
                        runtime_path: "::wasi::macros::wit_bindgen::rt",
                    });
                }
            }
        },
    );
    println!("cargo:rerun-if-changed=build.rs");
}

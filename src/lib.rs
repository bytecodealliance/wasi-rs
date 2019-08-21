#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::float_arithmetic,
        clippy::mut_mut,
        clippy::nonminimal_bool,
        clippy::option_map_unwrap_or,
        clippy::option_map_unwrap_or_else,
        clippy::print_stdout,
        clippy::unicode_not_nfc,
        clippy::use_self
    )
)]
#![no_std]
#[cfg(all(feature = "alloc", not(feature = "rustc-std-workspace-alloc")))]
extern crate alloc;
#[cfg(all(feature = "alloc", feature = "rustc-std-workspace-alloc"))]
extern crate rustc_std_workspace_alloc as alloc;

pub mod wasi_unstable;

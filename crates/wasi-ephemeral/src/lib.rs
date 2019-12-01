//! Raw API bindings to the WebAssembly System Interface (WASI)
//!
//! This crate provides Rust API bindings to WASI APIs. All WASI APIs are
//! exported from this crate and provided with the appropriate type signatures.
//! This crate is entirely procedurally generated from the `*.witx` files that
//! describe the WASI API.
//!
//! # WASI API Version
//!
//! The WASI API is evolving over time. It is both gaining new features as well
//! as tweaking the ABI of existing features. As a result it's important to
//! understand what version of this crate you're using and how it relates to
//! the WASI version of the spec.
//!
//! The WASI specification is organized into phases where there is a snapshot
//! at any one point in time describing the current state of the specification.
//! This crate implements the latest "ephemeral" version, not yet snapshotted.
//!
//! This crate will never be released, and should only be used by path
//! dependency or git dependency.
//!
//! # Crate Features
//!
//! This crate supports one feature, `std`, which implements the standard
//! `Error` trait for the exported [`Error`] type in this crate. This is
//! enabled by default but can be disabled to make the library `no_std`
//! compatible.

#![no_std]

mod error;

include!(concat!(env!("OUT_DIR"), "/lib_generated.rs"));

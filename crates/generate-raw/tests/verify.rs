#[test]
fn assert_same_as_src() {
    let actual = include_str!("../../../src/lib_generated.rs");
    let expected =
        generate_raw::generate(&["WASI/phases/snapshot/witx/wasi_snapshot_preview1.witx"]);
    if actual == expected {
        return;
    }
    panic!(
        "

the generate `raw.rs` does not match the actual source `raw.rs`, it's
recommended to run this command from the root of the repository:

    cargo run -p generate-raw crates/generate-raw/WASI > src/lib_generated.rs

"
    );
}

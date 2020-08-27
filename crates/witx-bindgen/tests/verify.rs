#[test]
fn assert_same_as_src() {
    let actual = include_str!("../../../src/lib_generated.rs");
    let expected =
        witx_bindgen::generate(&["WASI/phases/snapshot/witx/wasi_snapshot_preview1.witx"]);
    if actual == expected {
        return;
    }
    panic!(
        "

the generate `raw.rs` does not match the actual source `raw.rs`, it's
recommended to run this command from the root of the repository:

    cargo run -p witx-bindgen crates/witx-bindgen/WASI > src/lib_generated.rs

"
    );
}

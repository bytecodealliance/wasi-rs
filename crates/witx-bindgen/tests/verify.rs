#[test]
fn assert_same_as_src() {
    let actual = include_str!("../../wasip1/src/lib_generated.rs");
    let witx_path = "../wasip1/wasi_snapshot_preview1.witx";
    let expected = witx_bindgen::generate(&[witx_path]);
    if actual == expected {
        return;
    }
    panic!(
        "

the generated `raw.rs` does not match the actual source `raw.rs`, it's
recommended to run this command from the root of the repository:

    cargo run -p witx-bindgen crates/witx-bindgen/{} > src/lib_generated.rs

",
        witx_path
    );
}

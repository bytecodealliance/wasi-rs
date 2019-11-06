#[test]
fn assert_same_as_src() {
    let actual = include_str!("../../../src/wasi_unstable/raw.rs");
    let expected = generate_raw::generate("WASI".as_ref());
    if actual == expected {
        return;
    }
    panic!(
        "

the generate `raw.rs` does not match the actual source `raw.rs`, it's
recommended to run this command from the root of the repository:

    cargo run -p generate-raw crates/generate-raw/WASI > src/wasi_unstable/raw.rs

"
    );
}

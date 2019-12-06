use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let out_path: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let mut f = File::create(out_path.join("lib_generated.rs")).unwrap();
    const WASI_EPHEMERAL_WITX: &str =
        "../generate-raw/WASI/phases/ephemeral/witx/wasi_ephemeral_preview.witx";
    let witx_path: PathBuf = env::var_os("WASI_EPHEMERAL_WITX")
        .unwrap_or_else(|| WASI_EPHEMERAL_WITX.into())
        .into();
    let out = generate_raw::generate(&witx_path);
    write!(f, "{}", out).unwrap();
    println!("cargo:rerun-if-env-changed=WASI_EPHEMERAL_WITX");
    println!("cargo:rerun-if-changed={}", witx_path.display());
    println!(
        "cargo:rerun-if-changed={}",
        witx_path.with_file_name("typenames.witx").display(),
    );
    // TODO: Account for changes in use directives.
}

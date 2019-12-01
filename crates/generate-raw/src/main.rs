use std::env;
use std::path::PathBuf;

fn main() {
    let wasi_dir: PathBuf = env::args_os().nth(1).unwrap().into();
    let witx_path = wasi_dir.join("phases/snapshot/witx/wasi_snapshot_preview1.witx");
    print!("{}", generate_raw::generate(&witx_path));
}

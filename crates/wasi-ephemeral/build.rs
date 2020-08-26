use std::{
    env,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

const WITX_ROOT: &str = "../witx-bindgen/WASI/phases/ephemeral/witx";
const WITX_MODULES: &[&str] = &[
    "args", "clock", "environ", "fd", "path", "poll", "proc", "random", "sched", "sock",
];

fn main() {
    let out_path: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let mut f = File::create(out_path.join("lib_generated.rs")).unwrap();
    let root = Path::new(WITX_ROOT);
    let witx_paths: Vec<_> = WITX_MODULES
        .iter()
        .map(|x| {
            root.join(&["wasi_ephemeral_", x, ".witx"].join(""))
                .to_owned()
        })
        .collect();
    let out = witx_bindgen::generate(&witx_paths);
    write!(f, "{}", out).unwrap();
    for p in &witx_paths {
        println!("cargo:rerun-if-changed={}", p.display());
    }
    println!(
        "cargo:rerun-if-changed={}",
        root.join("wasi_ephemeral_typenames.witx").display()
    );
    // TODO: Account for changes in use directives.
}

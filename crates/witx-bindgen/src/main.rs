use std::env;
use std::path::PathBuf;

fn main() {
    let witx_paths = match env::args_os().nth(1) {
        Some(path) => vec![PathBuf::from(path)],
        None => witx::phases::snapshot().unwrap(),
    };
    print!("{}", witx_bindgen::generate(&witx_paths));
}

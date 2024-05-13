use std::io::Write as _;

use wasi::ext::rand::rand::Rng as _;
use wasi::ext::rand::{HostInsecureRng, HostRng};

fn main() {
    let mut stdout = wasi::cli::stdout::get_stdout();

    let r: u64 = HostRng.gen();
    stdout
        .write_all(format!("Cryptographically-secure random u64 is {r}\n").as_bytes())
        .unwrap();

    let r: u64 = HostInsecureRng.gen();
    stdout
        .write_all(format!("Pseudo-random u64 is {r}\n").as_bytes())
        .unwrap();

    stdout.flush().unwrap();
}

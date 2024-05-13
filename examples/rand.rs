use std::io::Write as _;

use wasi::ext::rand::rand::Rng as _;
use wasi::ext::rand::{HostInsecureRng, HostRng};

fn main() {
    let mut stdout = wasi::cli::stdout::get_stdout();

    let r: u64 = HostRng.gen();
    writeln!(stdout, "Cryptographically-secure random u64 is {r}").unwrap();

    let r: u64 = HostInsecureRng.gen();
    writeln!(stdout, "Pseudo-random u64 is {r}").unwrap();

    stdout.flush().unwrap();
}

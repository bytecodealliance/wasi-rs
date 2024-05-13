use wasi::ext::rand::rand::Rng as _;
use wasi::ext::rand::{HostInsecureRng, HostRng};

fn main() {
    let stdout = wasi::cli::stdout::get_stdout();

    let r: u64 = HostRng.gen();

    stdout
        .blocking_write_and_flush(
            format!("Cryptographically-secure random u64 is {r}\n").as_bytes(),
        )
        .unwrap();

    let r: u64 = HostInsecureRng.gen();
    stdout
        .blocking_write_and_flush(format!("Pseudo-random u64 is {r}\n").as_bytes())
        .unwrap();
}

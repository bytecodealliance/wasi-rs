use std::env;

fn main() {
    print!(
        "{}",
        generate_raw::generate(env::args_os().nth(1).unwrap().as_ref())
    );
}

wasi::export_command!(Example);

struct Example;

impl wasi::exports::cli::run::Guest for Example {
    fn run() -> Result<(), ()> {
        let stdout = wasi::cli::stdout::get_stdout();
        stdout.blocking_write_and_flush(b"Hello, WASI!").unwrap();
        Ok(())
    }
}

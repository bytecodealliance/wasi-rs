wasip3::cli::command::export!(Example);

struct Example;

impl wasip3::exports::cli::run::Guest for Example {
    async fn run() -> Result<(), ()> {
        let (mut tx, rx) = wasip3::wit_stream::new();
        wasip3::cli::stdout::set_stdout(rx);

        let remaining = tx.write_all(b"Hello, WASI!".to_vec()).await;
        assert!(remaining.is_empty());
        Ok(())
    }
}

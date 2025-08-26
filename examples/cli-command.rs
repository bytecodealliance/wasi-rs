wasi::cli::command::export!(Example);

struct Example;

impl wasi::exports::cli::run::Guest for Example {
    async fn run() -> Result<(), ()> {
        let (mut tx, rx) = wasi::wit_stream::new::<u8>();
        wasi::cli::stdout::set_stdout(rx);
        let result = tx.write_all(b"Hello, WASI!\n".to_vec()).await;
        assert!(result.is_empty());
        Ok(())
    }
}

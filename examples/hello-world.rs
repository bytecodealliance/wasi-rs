fn main() {
    wasi::async_support::block_on(async {
        let (mut tx, rx) = wasi::wit_stream::new::<u8>();
        wasi::cli::stdout::set_stdout(rx);
        let result = tx.write_all(b"Hello, WASI!\n".to_vec()).await;
        assert!(result.is_empty());
    })
}

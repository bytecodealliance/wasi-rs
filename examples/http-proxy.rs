use wasi::http::types::{ErrorCode, Fields, Request, Response};

wasi::http::proxy::export!(Example);

struct Example;

impl wasi::exports::http::handler::Guest for Example {
    async fn handle(_request: Request) -> Result<Response, ErrorCode> {
        let (trailers_tx, trailers_rx) = wasi::wit_future::new(|| Ok(None));
        drop(trailers_tx);
        let (mut body_tx, body_rx) = wasi::wit_stream::new();
        let (response, io_rx) = Response::new(Fields::new(), Some(body_rx), trailers_rx);

        wasi::async_support::spawn(async move {
            let remaining = body_tx.write_all(b"Hello, WASI!".to_vec()).await;
            assert!(remaining.is_empty());
            drop(body_tx);
            io_rx.await.expect("sending the response failed");
        });

        Ok(response)
    }
}

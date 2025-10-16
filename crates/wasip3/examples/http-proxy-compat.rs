use wasip3::http::types::{ErrorCode, Fields, Request, Response};
use wasip3::http_compat::BodyWriter;
use wasip3::{wit_bindgen, wit_future, wit_stream};

wasip3::http::proxy::export!(Example);

struct Example;

impl wasip3::exports::http::handler::Guest for Example {
    async fn handle(_request: Request) -> Result<Response, ErrorCode> {
        let (writer, body_rx, body_result_rx) = BodyWriter::new();

        let (response, _future_result) =
            Response::new(Fields::new(), Some(body_rx), body_result_rx);

        wit_bindgen::spawn(async move {
            writer
                .send_http_body(&mut "Hello, WASI!".to_string())
                .await
                .unwrap();
        });
        Ok(response)
    }
}

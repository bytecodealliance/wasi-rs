use wasip3::http_compat::{IncomingRequestBody, http_from_wasi_request, http_into_wasi_response};
use wasip3::http::types::{self, ErrorCode};

wasip3::http::proxy::export!(Example);

struct Example;

impl wasip3::exports::http::handler::Guest for Example {
    async fn handle(request: types::Request) -> Result<types::Response, ErrorCode> {
        let request = http_from_wasi_request(request)?;
        let response = serve(request).await?;
        http_into_wasi_response(response)
    }
}

async fn serve(_request: http::Request<IncomingRequestBody>) -> Result<http::Response<String>, ErrorCode> {
    Ok(http::Response::new("Hello, WASI!".to_string()))
}
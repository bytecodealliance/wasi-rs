use wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

wasi::http::incoming_handler::export!(Example);

struct Example;

impl exports::wasi::http::incoming_handler::Guest for Example {
    fn handle(_request: IncomingRequest, response_out: ResponseOutparam) {
        let resp = OutgoingResponse::new(Fields::new());
        let body = resp.body().unwrap();

        ResponseOutparam::set(response_out, Ok(resp));

        let out = body.write().unwrap();
        out.blocking_write_and_flush(b"Hello, WASI!").unwrap();
        drop(out);

        OutgoingBody::finish(body, None).unwrap();
    }
}

use std::io::Write as _;

use wasi::http::types::{IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam};

wasi::http::proxy::export!(Example);

struct Example;

impl wasi::exports::http::incoming_handler::Guest for Example {
    fn handle(request: IncomingRequest, response_out: ResponseOutparam) {
        let req_headers =
            http::HeaderMap::try_from(request.headers()).expect("failed to parse headers");
        let mut resp_headers = http::HeaderMap::new();
        for (name, value) in req_headers.iter() {
            // Append `-orig` to all request headers and send them back to the client
            resp_headers.append(
                http::HeaderName::try_from(format!("{name}-orig")).unwrap(),
                value.clone(),
            );
        }
        let resp = OutgoingResponse::new(resp_headers.into());
        let body = resp.body().unwrap();

        ResponseOutparam::set(response_out, Ok(resp));

        let mut out = body.write().unwrap();

        let method = http::Method::try_from(request.method()).unwrap();
        writeln!(out, "method: {method}").unwrap();

        if let Some(scheme) = request.scheme() {
            let scheme = http::uri::Scheme::try_from(scheme).unwrap();
            writeln!(out, "scheme: {scheme}").unwrap();
        }

        out.write_all(b"Hello, WASI!").unwrap();
        out.flush().unwrap();
        drop(out);

        OutgoingBody::finish(body, None).unwrap();
    }
}

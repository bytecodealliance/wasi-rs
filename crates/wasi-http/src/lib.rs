use anyhow::{anyhow, Context};
use bytes::{BufMut, Bytes, BytesMut};
use http::header::{HeaderName, HeaderValue};
#[cfg(feature = "std")]
use std::ops::Deref;

// Re-export HTTP related bindings
pub use wasi::snapshots::preview_2::{default_outgoing_http, streams, types as http_types};

pub struct DefaultClient {
    options: Option<default_outgoing_http::RequestOptions>,
}

impl DefaultClient {
    pub fn new(options: Option<default_outgoing_http::RequestOptions>) -> Self {
        Self { options }
    }

    pub fn handle(&self, req: http::Request<Bytes>) -> anyhow::Result<http::Response<Bytes>> {
        let req = Request::try_from(req)
            .context("converting http request")?
            .to_owned();

        let res = default_outgoing_http::handle(req, self.options);
        http_types::drop_outgoing_request(req);

        let response =
            http::Response::try_from(Response(res)).context("converting http response")?;
        http_types::drop_incoming_response(res);

        Ok(response)
    }
}

pub struct Request(default_outgoing_http::OutgoingRequest);

#[cfg(feature = "std")]
impl Deref for Request {
    type Target = default_outgoing_http::OutgoingRequest;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<http::Request<Bytes>> for Request {
    type Error = anyhow::Error;

    fn try_from(value: http::Request<Bytes>) -> Result<Self, Self::Error> {
        let (parts, body) = value.into_parts();
        let path = parts.uri.path();
        let query = parts.uri.query();
        let method = Method::from(parts.method);
        let headers = Headers::from(&parts.headers);
        let scheme = match parts.uri.scheme_str().unwrap_or("") {
            "http" => Some(http_types::SchemeParam::Http),
            "https" => Some(http_types::SchemeParam::Https),
            _ => None,
        };
        let request = http_types::new_outgoing_request(
            method.to_owned(),
            path,
            query.unwrap_or(""),
            scheme,
            parts
                .uri
                .authority()
                .map(|a| a.as_str())
                .ok_or_else(|| anyhow!("unable to extract authority"))?,
            headers.to_owned(),
        );

        let request_body = http_types::outgoing_request_write(request)
            .map_err(|_| anyhow!("outgoing request write failed"))?;

        let mut body_cursor = 0;
        if body.is_empty() {
            streams::write(request_body, &[]).map_err(|_| anyhow!("writing request body"))?;
        } else {
            while body_cursor < body.len() {
                let written = streams::write(request_body, &body[body_cursor..])
                    .map_err(|_| anyhow!("writing request body"))?;
                body_cursor += written as usize;
            }
        }

        Ok(Request(request))
    }
}

pub struct Method<'a>(http_types::MethodParam<'a>);

#[cfg(feature = "std")]
impl<'a> Deref for Method<'a> {
    type Target = http_types::MethodParam<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<http::Method> for Method<'a> {
    fn from(method: http::Method) -> Self {
        Self(match method {
             http::Method::GET => http_types::MethodParam::Get,
             http::Method::POST => http_types::MethodParam::Post,
             http::Method::PUT => http_types::MethodParam::Put,
             http::Method::DELETE => http_types::MethodParam::Delete,
             http::Method::PATCH => http_types::MethodParam::Patch,
             http::Method::CONNECT => http_types::MethodParam::Connect,
             http::Method::TRACE => http_types::MethodParam::Trace,
             http::Method::HEAD => http_types::MethodParam::Head,
             http::Method::OPTIONS => http_types::MethodParam::Options,
             _ => panic!("failed due to unsupported method, currently supported methods are: GET, POST, PUT, DELETE, PATCH, CONNECT, TRACE, HEAD, and OPTIONS"),
         })
    }
}

pub struct Response(http_types::IncomingResponse);

#[cfg(feature = "std")]
impl Deref for Response {
    type Target = http_types::IncomingResponse;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl TryFrom<Response> for http::Response<Bytes> {
    type Error = anyhow::Error;

    fn try_from(value: Response) -> Result<Self, Self::Error> {
        let future_response = value.to_owned();
        // TODO: we could create a pollable from the future_response and
        // poll on it here to test that its available immediately
        // poll::drop_pollable(future_response);

        let incoming_response = http_types::future_incoming_response_get(future_response)
            .ok_or_else(|| anyhow!("incoming response is available immediately"))?
            .map_err(|e| anyhow!("incoming response error: {e:?}"))?;
        http_types::drop_future_incoming_response(future_response);

        let status = http_types::incoming_response_status(incoming_response);
        let headers_handle = http_types::incoming_response_headers(incoming_response);

        let body_stream = http_types::incoming_response_consume(incoming_response)
            .map_err(|_| anyhow!("consuming incoming response"))?;

        let mut body = BytesMut::new();
        let mut eof = false;
        while !eof {
            let (body_chunk, stream_ended) = streams::read(body_stream, u64::MAX)
                .map_err(|_| anyhow!("reading response body"))?;
            eof = stream_ended;
            body.put(body_chunk.as_slice());
        }
        let mut res = http::Response::builder()
            .status(status)
            .body(body.freeze())
            .map_err(|_| anyhow!("building http response"))?;

        if headers_handle > 0 {
            let headers_map = res.headers_mut();
            for (name, value) in http_types::fields_entries(headers_handle) {
                headers_map.insert(
                    HeaderName::from_bytes(name.as_bytes())
                        .map_err(|_| anyhow!("converting response header name"))?,
                    HeaderValue::from_str(value.as_str())
                        .map_err(|_| anyhow!("converting response header value"))?,
                );
            }
        }

        Ok(res)
    }
}

pub struct Headers(http_types::Fields);

#[cfg(feature = "std")]
impl Deref for Headers {
    type Target = http_types::Fields;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<&'a http::HeaderMap> for Headers {
    fn from(headers: &'a http::HeaderMap) -> Self {
        Self(http_types::new_fields(
            &headers
                .iter()
                .map(|(name, value)| (name.as_str(), value.to_str().unwrap()))
                .collect::<Vec<(&'a str, &'a str)>>(),
        ))
    }
}

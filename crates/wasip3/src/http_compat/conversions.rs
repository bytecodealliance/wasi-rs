use super::{
    to_internal_error_code, 
    RequestOptionsExtension, 
    IncomingRequestBody, 
    IncomingResponseBody,
    Request as HttpRequest, 
    Response as HttpResponse,
    body_writer::BodyWriter,
};
use crate::http::types::{
    ErrorCode, 
    Fields, 
    HeaderError, 
    Headers, 
    Method, 
    Scheme, 
    Request as WasiHttpRequest,
    Response as WasiHttpResponse,
};
use std::{
    any::Any,
    convert::TryFrom,
};

/// Converts a host-side HTTP response (`HttpResponse<T>`) into a WASI HTTP response (`WasiHttpResponse`).
///
/// This function bridges standard Rust `http` responses with the WASI HTTP model,
/// serializing status codes, headers, and body data into their WebAssembly-compatible
/// representations. It supports generic response body types and streams the response
/// asynchronously into the WASI environment.
///
/// # Type Parameters
///
/// - `T`: The response body type.  
///   Must implement [`http_body::Body`] and [`Any`], with:
///   - `T::Data: Into<Vec<u8>>` — allowing conversion of body chunks into raw bytes.
///   - `T::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>` — allowing
///     conversion of body errors into boxed, thread-safe errors.
///
/// # Behavior
///
/// - If the response can be downcast to an [`IncomingResponseBody`] that has not yet started,
///   the unstarted response is immediately extracted and returned as a `WasiHttpResponse`.
/// - Otherwise:
///   - The response headers are cloned and converted into the WASI-compatible format.
///   - A new [`BodyWriter`] is created to stream the body data into the WASI environment.
///   - A [`WasiHttpResponse`] is constructed with the converted headers, body stream, and
///     result channel.
///   - The HTTP status code is set on the WASI response.
///   - An asynchronous task is spawned to forward the body data from the host response
///     into the WASI-side body writer.
/// 
/// # See Also
///
/// - [`http_from_wasi_response`] — converts a WASI response back into a host-side HTTP response.
/// - [`BodyWriter`] — for streaming body data into WASI.
/// - [`IncomingResponseBody`] — for handling pending or unstarted response states.
pub fn http_into_wasi_response<T>(mut resp: HttpResponse<T>) -> Result<WasiHttpResponse, ErrorCode>
where
    T: http_body::Body + Any,
    T::Data: Into<Vec<u8>>,
    T::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>
{
    if let Some(incoming_body) = (&mut resp as &mut dyn Any).downcast_mut::<IncomingResponseBody>()
    {
        if let Some(response) = incoming_body.take_unstarted() {
            return Ok(response);
        }
    }

    let headers = resp
        .headers()
        .clone()
        .try_into()
        .map_err(to_internal_error_code)?;

    let (body_writer, body_rx, body_result_rx) = BodyWriter::new();

    let (response, _future_result) =
        WasiHttpResponse::new(headers, Some(body_rx), body_result_rx);

    _ = response.set_status_code(resp.status().as_u16());

    wit_bindgen::spawn(async move {
        let mut body = std::pin::pin!(resp.into_body());
        _ = body_writer.send_http_body(&mut body).await;
    });

    Ok(response)
}

/// Converts a WASI HTTP response (`WasiHttpResponse`) into a standard host-side
/// [`http::Response`] suitable for use with Rust’s `http` ecosystem.
///
/// This function performs the reverse operation of [`http_into_wasi_response`], translating
/// the fields and body of a response from the WASI HTTP model into the conventional Rust
/// `http` crate representation.
///
/// # Behavior
///
/// - Creates an HTTP response builder and sets its status code from the WASI response.
/// - Iterates through all headers from the WASI response and adds them to the HTTP response builder.
/// - Wraps the WASI response’s body in an [`IncomingResponseBody`] to provide a streaming interface
///   compatible with host-side consumers.
/// - Finalizes and returns the constructed [`HttpResponse`].
///
/// # See Also
///
/// - [`http_into_wasi_response`] — the inverse conversion.
/// - [`IncomingResponseBody`] — for handling WASI-to-host body streams.
/// - [`ErrorCode`] — for standardized error reporting.
pub fn http_from_wasi_response(resp: WasiHttpResponse) -> Result<HttpResponse, ErrorCode> {
    let mut builder = http::Response::builder().status(resp.get_status_code());

    for (k, v) in resp.get_headers().copy_all() {
        builder = builder.header(k, v);
    }

    let body = IncomingResponseBody::new(resp)?;
    builder.body(body).map_err(to_internal_error_code) // TODO: downcast to more specific http error codes
}

/// Converts a host-side HTTP request (`HttpRequest<T>`) into a WASI HTTP request (`WasiHttpRequest`).
///
/// This function bridges between the standard Rust `http` crate request types and the WASI HTTP
/// request model used in WebAssembly components. It serializes headers, method, URI components,
/// and body streams into their WASI equivalents while preserving request metadata.
///
/// # Type Parameters
///
/// - `T`: The body type of the HTTP request. It must implement [`http_body::Body`] and [`Any`],
///   and its data must be convertible into a `Vec<u8>`. The body’s error type must also be convertible
///   into a boxed error implementing [`std::error::Error`].
///
/// # Behavior
///
/// - If the input request can be downcast to an [`IncomingRequestBody`] that has not yet started,
///   the unstarted request is immediately extracted and returned as a `WasiHttpRequest`.
/// - Otherwise, the request is decomposed into its parts and body using [`HttpRequest::into_parts()`].
/// - The headers are cloned and converted into the WASI format using `try_into()`.
/// - Optional per-request configuration (such as timeouts or redirects) is extracted from
///   [`RequestOptionsExtension`] if present in the request’s extensions.
/// - A [`BodyWriter`] is created to stream the request body into the WASI component.
/// - The method, scheme, authority, and path+query components are set on the WASI request,
///   returning standardized [`ErrorCode`]s if invalid.
/// - An asynchronous task is spawned to send the body data into the WASI-side body stream.
///
/// # See Also
///
/// - [`http_from_wasi_response`] — for converting WASI responses back into standard HTTP responses.
/// - [`http_into_wasi_response`] — for converting HTTP responses into WASI format.
/// - [`BodyWriter`] — for streaming request bodies to WASI.
/// - [`IncomingRequestBody`] — for managing unstarted or in-progress request states.
pub fn http_into_wasi_request<T>(mut req: HttpRequest<T>) -> Result<WasiHttpRequest, ErrorCode>
where
    T: http_body::Body + Any,
    T::Data: Into<Vec<u8>>,
    T::Error: Into<Box<dyn std::error::Error + Send + Sync + 'static>>
{
    if let Some(incoming_body) = (&mut req as &mut dyn Any).downcast_mut::<IncomingRequestBody>()
    {
        if let Some(request) = incoming_body.take_unstarted() {
            return Ok(request);
        }
    }

    let (parts, body) = req.into_parts();

    let options = parts
        .extensions
        .get::<RequestOptionsExtension>()
        .cloned()
        .map(|o| o.0);

    let headers = parts
        .headers
        .try_into()
        .map_err(to_internal_error_code)?;

    let (body_writer, contents_rx, trailers_rx) = BodyWriter::new();

    let (req, _result) = WasiHttpRequest::new(headers, Some(contents_rx), trailers_rx, options);

    req.set_method(&parts.method.into())
        .map_err(|()| ErrorCode::HttpRequestMethodInvalid)?;

    let scheme = parts.uri.scheme().map(Into::into);
    req.set_scheme(scheme.as_ref())
        .map_err(|()| ErrorCode::HttpProtocolError)?;

    req.set_authority(parts.uri.authority().map(|a| a.as_str()))
        .map_err(|()| ErrorCode::HttpRequestUriInvalid)?;

    req.set_path_with_query(parts.uri.path_and_query().map(|pq| pq.as_str()))
        .map_err(|()| ErrorCode::HttpRequestUriInvalid)?;

    wit_bindgen::spawn(async move {
        let mut body = std::pin::pin!(body);
        _ = body_writer.send_http_body(&mut body).await;
    });

    Ok(req)
}

/// Converts a WASI HTTP request (`WasiHttpRequest`) into a standard host-side
/// [`http::Request`].
///
/// This function performs the reverse of [`http_into_wasi_request`], translating a request
/// from the WASI HTTP model into a conventional Rust `http` request type. It reconstructs
/// the URI, method, headers, extensions, and body so that the request can be used directly
/// by host HTTP clients, servers, or middleware.
///
/// # Behavior
///
/// - Builds a [`Uri`] using the scheme, authority, and path/query values retrieved from
///   the WASI request.
/// - Initializes an [`http::Request`] builder with the reconstructed URI and the request method.
/// - Copies all headers from the WASI request into the new host request.
/// - If the WASI request includes configuration options, wraps them in a
///   [`RequestOptionsExtension`] and attaches them as an extension to the request.
/// - Wraps the WASI request body in an [`IncomingRequestBody`] to provide an asynchronous
///   stream interface for the host environment.
/// - Returns the completed [`HttpRequest`] once construction succeeds.
///
/// # See Also
///
/// - [`http_into_wasi_request`] — converts from host HTTP requests into WASI requests.
/// - [`http_from_wasi_request`] — converts from WASI responses into host responses.
/// - [`IncomingRequestBody`] — for streaming WASI request bodies into host code.
/// - [`RequestOptionsExtension`] — for carrying optional request metadata.
pub fn http_from_wasi_request(req: WasiHttpRequest) -> Result<HttpRequest, ErrorCode> {
    let uri = {
        let mut builder = http::Uri::builder();
        if let Some(scheme) = req.get_scheme() {
            builder = builder.scheme(scheme);
        }
        if let Some(authority) = req.get_authority() {
            builder = builder.authority(authority);
        }
        if let Some(path_and_query) = req.get_path_with_query() {
            builder = builder.path_and_query(path_and_query);
        }
        builder
            .build()
            .map_err(|_| ErrorCode::HttpRequestUriInvalid)?
    };

    let mut builder = http::Request::builder()
        .method(req.get_method())
        .uri(uri);

    if let Some(options) = req.get_options().map(RequestOptionsExtension) {
        builder = builder.extension(options);
    }

    for (k, v) in req.get_headers().copy_all() {
        builder = builder.header(k, v);
    }

    let body = IncomingRequestBody::new(req)?;

    builder.body(body).map_err(to_internal_error_code) // TODO: downcast to more specific http error codes
}

impl TryFrom<Scheme> for http::uri::Scheme {
    type Error = http::uri::InvalidUri;

    fn try_from(scheme: Scheme) -> Result<Self, Self::Error> {
        match scheme {
            Scheme::Http => Ok(http::uri::Scheme::HTTP),
            Scheme::Https => Ok(http::uri::Scheme::HTTPS),
            Scheme::Other(s) => s.parse(),
        }
    }
}

impl From<&http::uri::Scheme> for Scheme {
    fn from(scheme: &http::uri::Scheme) -> Self {
        match scheme {
            s if s == &http::uri::Scheme::HTTP => Scheme::Http,
            s if s == &http::uri::Scheme::HTTPS => Scheme::Https,
            other => Scheme::Other(other.to_string()),
        }
    }
}

impl TryFrom<Method> for http::Method {
    type Error = http::method::InvalidMethod;

    fn try_from(method: Method) -> Result<Self, Self::Error> {
        match method {
            Method::Get => Ok(http::Method::GET),
            Method::Post => Ok(http::Method::POST),
            Method::Put => Ok(http::Method::PUT),
            Method::Delete => Ok(http::Method::DELETE),
            Method::Patch => Ok(http::Method::PATCH),
            Method::Head => Ok(http::Method::HEAD),
            Method::Options => Ok(http::Method::OPTIONS),
            Method::Connect => Ok(http::Method::CONNECT),
            Method::Trace => Ok(http::Method::TRACE),
            Method::Other(o) => http::Method::from_bytes(o.as_bytes()),
        }
    }
}

impl From<&http::Method> for Method {
    fn from(method: &http::Method) -> Self {
        match method {
            &http::Method::GET => Method::Get,
            &http::Method::POST => Method::Post,
            &http::Method::PUT => Method::Put,
            &http::Method::DELETE => Method::Delete,
            &http::Method::PATCH => Method::Patch,
            &http::Method::HEAD => Method::Head,
            &http::Method::OPTIONS => Method::Options,
            &http::Method::CONNECT => Method::Connect,
            &http::Method::TRACE => Method::Trace,
            other => Method::Other(other.to_string()),
        }
    }
}

impl From<http::Method> for Method {
    fn from(method: http::Method) -> Self {
        (&method).into()
    }
}

impl TryFrom<Headers> for http::HeaderMap {
    type Error = ErrorCode;

    fn try_from(headers: Headers) -> Result<Self, Self::Error> {
        headers
            .copy_all()
            .into_iter()
            .try_fold(http::HeaderMap::new(), |mut map, (k, v)| {
                let v = http::HeaderValue::from_bytes(&v).map_err(to_internal_error_code)?;
                let k: http::HeaderName = k.parse().map_err(to_internal_error_code)?;
                map.append(k, v);
                Ok(map)
            })
    }
}

impl TryFrom<http::HeaderMap> for Fields {
    type Error = HeaderError;

    fn try_from(map: http::HeaderMap) -> Result<Self, Self::Error> {
        // https://docs.rs/http/1.3.1/http/header/struct.HeaderMap.html#method.into_iter-2
        // For each yielded item that has None provided for the HeaderName, then
        // the associated header name is the same as that of the previously
        // yielded item. The first yielded item will have HeaderName set.
        let mut last_name = None;
        let iter = map.into_iter().map(move |(name, value)| {
            if name.is_some() {
                last_name = name;
            }
            let name = last_name
                .as_ref()
                .expect("HeaderMap::into_iter always returns Some(name) before None");
            let value = bytes::Bytes::from_owner(value).to_vec();
            (name.as_str().into(), value)
        });
        let entries = Vec::from_iter(iter);
        Fields::from_list(&entries)
    }
}

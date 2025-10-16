use super::to_internal_error_code;
use crate::http::types::{ErrorCode, Fields, HeaderError, Headers, Method, Scheme};
use std::convert::TryFrom;

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

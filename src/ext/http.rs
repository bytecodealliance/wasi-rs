pub use http;

use core::fmt::Display;

impl From<http::Method> for crate::http::types::Method {
    fn from(method: http::Method) -> Self {
        use std::string::ToString;

        match method.as_str() {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
            "PATCH" => Self::Patch,
            _ => Self::Other(method.to_string()),
        }
    }
}

impl TryFrom<crate::http::types::Method> for http::Method {
    type Error = http::method::InvalidMethod;

    fn try_from(method: crate::http::types::Method) -> Result<Self, Self::Error> {
        match method {
            crate::http::types::Method::Get => Ok(Self::GET),
            crate::http::types::Method::Head => Ok(Self::HEAD),
            crate::http::types::Method::Post => Ok(Self::POST),
            crate::http::types::Method::Put => Ok(Self::PUT),
            crate::http::types::Method::Delete => Ok(Self::DELETE),
            crate::http::types::Method::Connect => Ok(Self::CONNECT),
            crate::http::types::Method::Options => Ok(Self::OPTIONS),
            crate::http::types::Method::Trace => Ok(Self::TRACE),
            crate::http::types::Method::Patch => Ok(Self::PATCH),
            crate::http::types::Method::Other(method) => method.parse(),
        }
    }
}

impl From<http::uri::Scheme> for crate::http::types::Scheme {
    fn from(scheme: http::uri::Scheme) -> Self {
        use std::string::ToString;

        match scheme.as_str() {
            "http" => Self::Http,
            "https" => Self::Https,
            _ => Self::Other(scheme.to_string()),
        }
    }
}

impl TryFrom<crate::http::types::Scheme> for http::uri::Scheme {
    type Error = http::uri::InvalidUri;

    fn try_from(scheme: crate::http::types::Scheme) -> Result<Self, Self::Error> {
        match scheme {
            crate::http::types::Scheme::Http => Ok(Self::HTTP),
            crate::http::types::Scheme::Https => Ok(Self::HTTPS),
            crate::http::types::Scheme::Other(scheme) => scheme.parse(),
        }
    }
}

#[derive(Debug)]
pub enum FieldsToHeaderMapError {
    InvalidHeaderName(http::header::InvalidHeaderName),
    InvalidHeaderValue(http::header::InvalidHeaderValue),
}

impl Display for FieldsToHeaderMapError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FieldsToHeaderMapError::InvalidHeaderName(e) => write!(f, "invalid header name: {e}"),
            FieldsToHeaderMapError::InvalidHeaderValue(e) => write!(f, "invalid header value: {e}"),
        }
    }
}

impl std::error::Error for FieldsToHeaderMapError {}

impl TryFrom<crate::http::types::Fields> for http::HeaderMap {
    type Error = FieldsToHeaderMapError;

    fn try_from(fields: crate::http::types::Fields) -> Result<Self, Self::Error> {
        let mut headers = http::HeaderMap::new();
        for (name, value) in fields.entries() {
            let name = http::HeaderName::try_from(name)
                .map_err(FieldsToHeaderMapError::InvalidHeaderName)?;
            let value = http::HeaderValue::try_from(value)
                .map_err(FieldsToHeaderMapError::InvalidHeaderValue)?;
            match headers.entry(name) {
                http::header::Entry::Vacant(entry) => {
                    entry.insert(value);
                }
                http::header::Entry::Occupied(mut entry) => {
                    entry.append(value);
                }
            };
        }
        Ok(headers)
    }
}

impl From<http::HeaderMap> for crate::http::types::Fields {
    fn from(headers: http::HeaderMap) -> Self {
        use std::string::ToString;

        let fields = crate::http::types::Fields::new();
        for (name, value) in headers.iter() {
            fields
                .append(&name.to_string(), &value.as_bytes().to_vec())
                .expect("failed to append header")
        }
        fields
    }
}

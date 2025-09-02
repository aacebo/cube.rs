use std::fmt;

/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Method {
    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/GET
    #[cfg_attr(feature = "serde", serde(rename = "GET"))]
    Get,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/HEAD
    #[cfg_attr(feature = "serde", serde(rename = "HEAD"))]
    Head,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/POST
    #[cfg_attr(feature = "serde", serde(rename = "POST"))]
    Post,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/PUT
    #[cfg_attr(feature = "serde", serde(rename = "PUT"))]
    Put,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/PATCH
    #[cfg_attr(feature = "serde", serde(rename = "PATCH"))]
    Patch,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/DELETE
    #[cfg_attr(feature = "serde", serde(rename = "DELETE"))]
    Delete,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/CONNECT
    #[cfg_attr(feature = "serde", serde(rename = "CONNECT"))]
    Connect,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/OPTIONS
    #[cfg_attr(feature = "serde", serde(rename = "OPTIONS"))]
    Options,

    /// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods/TRACE
    #[cfg_attr(feature = "serde", serde(rename = "TRACE"))]
    Trace,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Get => write!(f, "GET"),
            Self::Head => write!(f, "HEAD"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Patch => write!(f, "PATCH"),
            Self::Delete => write!(f, "DELETE"),
            Self::Connect => write!(f, "CONNECT"),
            Self::Options => write!(f, "OPTIONS"),
            Self::Trace => write!(f, "TRACE"),
        };
    }
}

impl TryFrom<&str> for Method {
    type Error = cube_core::error::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        return match value {
            "GET" => Ok(Self::Get),
            "HEAD" => Ok(Self::Head),
            "POST" => Ok(Self::Post),
            "PUT" => Ok(Self::Put),
            "PATCH" => Ok(Self::Patch),
            "DELETE" => Ok(Self::Delete),
            "CONNECT" => Ok(Self::Connect),
            "OPTIONS" => Ok(Self::Options),
            "TRACE" => Ok(Self::Trace),
            _ => Err(Self::Error::from(format!(
                "{} is not an http method",
                value
            ))),
        };
    }
}

impl TryFrom<String> for Method {
    type Error = cube_core::error::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        return Self::try_from(value.as_str());
    }
}

impl Into<String> for Method {
    fn into(self) -> String {
        return self.to_string();
    }
}

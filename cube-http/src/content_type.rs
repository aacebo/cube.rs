use std::fmt;

/// https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Type
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ContentType {
    /// application/json
    #[cfg_attr(feature = "serde", serde(rename = "application/json"))]
    Json,

    /// text/plain; charset=utf-8
    #[cfg_attr(feature = "serde", serde(rename = "text/plain; charset=utf-8"))]
    PlainText,

    /// text/html; charset=utf-8
    #[cfg_attr(feature = "serde", serde(rename = "text/html; charset=utf-8"))]
    Html,

    /// text/xml
    #[cfg_attr(feature = "serde", serde(rename = "text/xml"))]
    Xml,

    /// application/www-form-url-encoded
    #[cfg_attr(feature = "serde", serde(rename = "application/www-form-url-encoded"))]
    FormUrlEncoded,

    /// image/jpeg
    #[cfg_attr(feature = "serde", serde(rename = "image/jpeg"))]
    Jpg,

    /// image/png
    #[cfg_attr(feature = "serde", serde(rename = "image/png"))]
    Png,

    /// application/octet-stream
    #[cfg_attr(feature = "serde", serde(rename = "application/octet-stream"))]
    OctetStream,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Json => write!(f, "application/json"),
            Self::PlainText => write!(f, "text/plain; charset=utf-8"),
            Self::Html => write!(f, "text/html; charset=utf-8"),
            Self::Xml => write!(f, "text/xml"),
            Self::FormUrlEncoded => write!(f, "application/www-form-url-encoded"),
            Self::Jpg => write!(f, "image/jpeg"),
            Self::Png => write!(f, "image/png"),
            Self::OctetStream => write!(f, "application/octet-stream"),
        };
    }
}

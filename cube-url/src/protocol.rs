use std::fmt;

/// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Protocol {
    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/blob
    #[cfg_attr(feature = "serde", serde(rename = "blob"))]
    Blob,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/data
    #[cfg_attr(feature = "serde", serde(rename = "data"))]
    Data,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#file
    #[cfg_attr(feature = "serde", serde(rename = "file"))]
    File,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#ftp
    #[cfg_attr(feature = "serde", serde(rename = "ftp"))]
    Ftp,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#http
    #[cfg_attr(feature = "serde", serde(rename = "http"))]
    Http,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#http
    #[cfg_attr(feature = "serde", serde(rename = "https"))]
    Https,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/javascript
    #[cfg_attr(feature = "serde", serde(rename = "javascript"))]
    Javascript,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#mailto
    #[cfg_attr(feature = "serde", serde(rename = "mailto"))]
    MailTo,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#ssh
    #[cfg_attr(feature = "serde", serde(rename = "ssh"))]
    Ssh,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#tel
    #[cfg_attr(feature = "serde", serde(rename = "tel"))]
    Tel,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes/urn
    #[cfg_attr(feature = "serde", serde(rename = "urn"))]
    Urn,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#view-source
    #[cfg_attr(feature = "serde", serde(rename = "view-source"))]
    ViewSource,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#ws
    #[cfg_attr(feature = "serde", serde(rename = "ws"))]
    Ws,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes#wss
    #[cfg_attr(feature = "serde", serde(rename = "wss"))]
    Wss,

    #[cfg_attr(feature = "serde", serde(untagged))]
    Other(String),
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Blob => write!(f, "blob"),
            Self::Data => write!(f, "data"),
            Self::File => write!(f, "file"),
            Self::Ftp => write!(f, "ftp"),
            Self::Http => write!(f, "http"),
            Self::Https => write!(f, "https"),
            Self::Javascript => write!(f, "javascript"),
            Self::MailTo => write!(f, "mailto"),
            Self::Ssh => write!(f, "ssh"),
            Self::Tel => write!(f, "tel"),
            Self::Urn => write!(f, "urn"),
            Self::ViewSource => write!(f, "view-source"),
            Self::Ws => write!(f, "ws"),
            Self::Wss => write!(f, "wss"),
            Self::Other(v) => write!(f, "{}", v),
        };
    }
}

impl From<&str> for Protocol {
    fn from(value: &str) -> Self {
        return match value.to_lowercase().as_str() {
            "blob" => Self::Blob,
            "data" => Self::Data,
            "file" => Self::File,
            "ftp" => Self::Ftp,
            "http" => Self::Http,
            "https" => Self::Https,
            "javascript" => Self::Javascript,
            "mailto" => Self::MailTo,
            "ssh" => Self::Ssh,
            "tel" => Self::Tel,
            "urn" => Self::Urn,
            "view-source" => Self::ViewSource,
            "ws" => Self::Ws,
            "wss" => Self::Wss,
            other => Self::Other(other.to_string()),
        };
    }
}

impl From<String> for Protocol {
    fn from(value: String) -> Self {
        return From::<&str>::from(&value);
    }
}

use std::fmt;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Header {
    #[cfg_attr(feature = "serde", serde(untagged))]
    Raw(String),
}

impl From<&str> for Header {
    fn from(value: &str) -> Self {
        return Self::Raw(value.to_string());
    }
}

impl From<String> for Header {
    fn from(value: String) -> Self {
        return Self::Raw(value);
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Raw(v) => write!(f, "{}", v),
        };
    }
}

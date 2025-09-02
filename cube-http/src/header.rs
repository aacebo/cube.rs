#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Header {
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

use cube_url::Url;

use crate::Headers;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Request {
    pub url: Url,
    pub headers: Headers,
}

#[cfg(feature = "serde")]
impl std::fmt::Display for Request {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", serde_json::to_string_pretty(self).unwrap());
    }
}

use cube_core::error::Error;
use cube_url::Url;

use crate::{Headers, Method, RequestMessage};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Request<T> {
    pub method: Method,
    pub url: Url,
    pub headers: Headers,
    pub body: Option<T>,
}

impl<T> TryFrom<&RequestMessage> for Request<T> {
    type Error = Error;

    fn try_from(request: &RequestMessage) -> Result<Self, Self::Error> {
        return Ok(Self {
            method: request.method,
            headers: Headers::from(&request.headers),
            url: Url::parse(&format!(
                "{}://{}{}",
                request.protocol,
                &request.headers.get("Host").unwrap(),
                &request.path,
            ))?,
            body: None,
        });
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> std::fmt::Display for Request<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", serde_json::to_string_pretty(self).unwrap());
    }
}

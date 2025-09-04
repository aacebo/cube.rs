mod protocol;
pub use protocol::*;

mod params;
pub use params::*;

mod query;
pub use query::*;

pub mod template;

use cube_core::{bytes::Scanner, error::Error};

/// https://developer.mozilla.org/en-US/docs/Web/URI
/// https://developer.mozilla.org/en-US/docs/Web/API/URL
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Url {
    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Schemes
    protocol: Protocol,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Authority#host
    host: String,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Authority#port
    port: Option<u16>,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Path
    path: String,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Path
    params: Params,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Query
    query: Query,

    /// https://developer.mozilla.org/en-US/docs/Web/URI/Reference/Authority#user
    user: Option<String>,
}

impl Url {
    pub fn protocol(&self) -> &Protocol {
        return &self.protocol;
    }

    pub fn host(&self) -> &str {
        return &self.host;
    }

    pub fn base(&self) -> String {
        return format!("{}://{}", self.protocol, self.host);
    }

    pub fn port(&self) -> Option<u16> {
        return match self.port {
            None => None,
            Some(v) => Some(v),
        };
    }

    pub fn path(&self) -> &str {
        return &self.path;
    }

    pub fn params(&self) -> &Params {
        return &self.params;
    }

    pub fn query(&self) -> &Query {
        return &self.query;
    }

    pub fn user(&self) -> Option<&str> {
        return match &self.user {
            None => None,
            Some(v) => Some(v),
        };
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl std::fmt::Display for Url {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}://{}{}", &self.protocol, &self.host, &self.path);
    }
}

impl Url {
    pub fn parse(url: &str) -> Result<Self, Error> {
        let bytes = &mut Scanner::from(url.as_bytes());
        let protocol = Protocol::from(bytes.next_until_bytes(b"://").commit());
        let host = bytes.fshift(3).next_until(b'/').commit().to_string();
        let path = bytes.next_until(b'?').commit().to_string();
        let query = Query::parse(bytes.into_inner())?;
        let port = match host.find(":") {
            None => None,
            Some(i) => Some(host[i + 1..].parse()?),
        };

        return Ok(Self {
            protocol,
            host,
            port,
            path,
            params: Params::new(),
            query,
            user: None,
        });
    }
}

#[cfg(test)]
mod test {
    #[test]
    pub fn should_parse_query() {
        let url = super::Url::parse("http://localhost/users?hello=world&a=bcd").unwrap();
        assert_eq!(url.query.get("hello").unwrap(), "world");
        assert_eq!(url.query.get("a").unwrap(), "bcd");
    }
}

use std::io::Write;
use std::{collections::HashMap, net};

use cube_core::bytes::ByteReader;
use cube_core::error::Error;

use crate::Method;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestMessage {
    pub method: Method,
    pub path: String,
    pub protocol: String,
    pub protocol_v: String,
    pub headers: HashMap<String, String>,
}

impl RequestMessage {
    pub fn read(stream: &net::TcpStream) -> Result<Self, Error> {
        let mut reader = ByteReader::new(stream);
        let mut message = Self {
            method: Method::try_from(reader.read_utf8_until_exclusive(b" ")?)?,
            path: reader.read_utf8_until_exclusive(b" ")?,
            protocol: reader.read_utf8_until_exclusive(b"/")?.to_lowercase(),
            protocol_v: reader.read_utf8_until_exclusive(b"\r\n")?,
            headers: HashMap::new(),
        };

        loop {
            let key = reader.read_utf8_until_exclusive(b": ")?;
            let mut value = reader.read_utf8_until_exclusive(b"\r\n")?;

            if value.starts_with("\"") {
                value = value.strip_prefix("\"").unwrap().to_string();
            }

            if value.ends_with("\"") {
                value = value.strip_suffix("\"").unwrap().to_string();
            }

            message.headers.insert(key, value);

            if reader.next_if(b"\r\n") {
                break;
            }
        }

        return Ok(message);
    }

    pub fn write(&self, stream: &mut net::TcpStream) -> Result<usize, Error> {
        let mut count = 0;
        let mut line = format!(
            "{} {} {}/{}\r\n",
            self.method, self.path, self.protocol, self.protocol_v
        );

        stream.write_all(line.as_bytes())?;
        count += line.len();

        for (key, value) in self.headers.iter() {
            line = format!("{}: {}\r\n", key, value);
            stream.write_all(line.as_bytes())?;
            count += line.len();
        }

        stream.write_all(b"\r\n")?;
        count += 2;
        return Ok(count);
    }
}

#[cfg(feature = "serde")]
impl std::fmt::Display for RequestMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", serde_json::to_string_pretty(self).unwrap());
    }
}

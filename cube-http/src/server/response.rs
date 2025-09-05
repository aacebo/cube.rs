use cube_core::error::Error;
use cube_url::Protocol;

use crate::{Header, Headers, ResponseMessage, Status};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Response<T> {
    pub protocol: Protocol,
    pub protocol_v: String,
    pub status: Status,
    pub headers: Headers,
    pub body: Option<T>,

    #[serde(skip)]
    stream: std::net::TcpStream,
}

impl<T> Response<T> {
    pub fn new(stream: std::net::TcpStream) -> Self {
        return Self {
            protocol: Protocol::Http,
            protocol_v: String::from("1.1"),
            status: Status::Ok,
            headers: Headers::new(),
            body: None,
            stream,
        };
    }

    pub fn protocol(&mut self, protocol: &Protocol, version: &str) -> &mut Self {
        self.protocol = protocol.clone();
        self.protocol_v = version.to_string();
        return self;
    }

    pub fn status(&mut self, status: Status) -> &mut Self {
        self.status = status;
        return self;
    }

    pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
        self.headers.set(name, &Header::Raw(value.to_string()));
        return self;
    }

    pub fn headers(&mut self, headers: &[(&str, &str)]) -> &mut Self {
        for (name, value) in headers {
            self.headers.set(name, &Header::Raw(value.to_string()));
        }

        return self;
    }

    pub fn send(&mut self) -> Result<usize, Error> {
        let count = self.to_message().write(&mut self.stream)?;
        self.stream.shutdown(std::net::Shutdown::Both)?;
        return Ok(count);
    }

    pub fn to_message(&self) -> ResponseMessage {
        return ResponseMessage {
            protocol: self.protocol.to_string(),
            protocol_v: self.protocol_v.clone(),
            status: self.status,
            headers: self.headers.clone().into(),
        };
    }
}

impl<T> Into<ResponseMessage> for Response<T> {
    fn into(self) -> ResponseMessage {
        return ResponseMessage {
            protocol: self.protocol.to_string(),
            protocol_v: self.protocol_v.clone(),
            status: self.status,
            headers: self.headers.into(),
        };
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> std::fmt::Display for Response<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", serde_json::to_string_pretty(self).unwrap());
    }
}

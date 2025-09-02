use std::{error, fmt, io, num, string};

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.message);
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        return Some(self);
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        return Self {
            message: value.to_string(),
        };
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        return Self { message: value };
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        return Self {
            message: value.to_string(),
        };
    }
}

impl From<fmt::Error> for Error {
    fn from(value: fmt::Error) -> Self {
        return Self {
            message: value.to_string(),
        };
    }
}

impl From<string::ParseError> for Error {
    fn from(value: string::ParseError) -> Self {
        return Self {
            message: value.to_string(),
        };
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(value: string::FromUtf8Error) -> Self {
        return Self {
            message: value.to_string(),
        };
    }
}

impl From<string::FromUtf16Error> for Error {
    fn from(value: string::FromUtf16Error) -> Self {
        return Self {
            message: value.to_string(),
        };
    }
}

impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Self {
        return Self {
            message: value.to_string(),
        };
    }
}

use std::{error, fmt, io, num, string};

#[derive(Debug, Clone)]
pub struct Error {
    pub message: Option<String>,
    pub errors: Vec<Error>,
}

impl Error {
    pub fn new() -> Self {
        return Self {
            message: None,
            errors: vec![],
        };
    }

    pub fn push(&mut self, error: &Error) -> &mut Self {
        self.errors.push(error.clone());
        return self;
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = &self.message {
            write!(f, "{}", v)?;
        }

        for error in &self.errors {
            write!(f, "{}", error)?;
        }

        return Ok(());
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
            message: Some(value.to_string()),
            errors: vec![],
        };
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        return Self {
            message: Some(value),
            errors: vec![],
        };
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        return Self {
            message: Some(value.to_string()),
            errors: vec![],
        };
    }
}

impl From<fmt::Error> for Error {
    fn from(value: fmt::Error) -> Self {
        return Self {
            message: Some(value.to_string()),
            errors: vec![],
        };
    }
}

impl From<string::ParseError> for Error {
    fn from(value: string::ParseError) -> Self {
        return Self {
            message: Some(value.to_string()),
            errors: vec![],
        };
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(value: string::FromUtf8Error) -> Self {
        return Self {
            message: Some(value.to_string()),
            errors: vec![],
        };
    }
}

impl From<string::FromUtf16Error> for Error {
    fn from(value: string::FromUtf16Error) -> Self {
        return Self {
            message: Some(value.to_string()),
            errors: vec![],
        };
    }
}

impl From<num::ParseIntError> for Error {
    fn from(value: num::ParseIntError) -> Self {
        return Self {
            message: Some(value.to_string()),
            errors: vec![],
        };
    }
}

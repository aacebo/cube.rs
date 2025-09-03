use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::template::Token;

/// Template Identifier
///
/// Example
/// -------
/// `test`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ident {
    pub start: usize,
    pub end: usize,
    pub name: String,
}

impl Ident {
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Ident {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Self, Error> {
        while !scan.is_eof() && scan.curr() != b'}' {
            scan.next();
        }

        if !scan.is_eof() && scan.curr() != b'}' {
            return Err(Error::from("[cube::url::template] => expected '}'"));
        }

        return Ok(Self {
            start: scan.left(),
            end: scan.right(),
            name: scan.commit().to_string(),
        });
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.name);
    }
}

impl PartialEq<Token> for Ident {
    fn eq(&self, other: &Token) -> bool {
        return match other {
            Token::Ident(v) => v.name == self.name,
            _ => false,
        };
    }
}

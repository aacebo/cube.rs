use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::template::Token;

/// Template Hash
///
/// Example
/// -------
/// `#`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hash {
    pub start: usize,
    pub end: usize,
}

impl Hash {
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Hash {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Self, Error> {
        if scan.curr() != b'#' {
            return Err(Error::from("[cube::url::template] => expected '#'"));
        }

        let start = scan.left();
        let end = scan.right();
        scan.commit();

        return Ok(Self { start, end });
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "#");
    }
}

impl PartialEq<Token> for Hash {
    fn eq(&self, other: &Token) -> bool {
        return match other {
            Token::Hash(_) => true,
            _ => false,
        };
    }
}

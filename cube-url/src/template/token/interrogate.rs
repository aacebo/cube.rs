use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

/// Template Interrogate
///
/// Example
/// -------
/// `?`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Interrogate {
    pub start: usize,
    pub end: usize,
}

impl Interrogate {
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Interrogate {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Self, Error> {
        if scan.curr() != b'?' {
            return Err(Error::from("[cube::url::template] => expected '?'"));
        }

        let start = scan.left();
        let end = scan.right();
        scan.commit();

        return Ok(Self { start, end });
    }
}

impl fmt::Display for Interrogate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "?");
    }
}

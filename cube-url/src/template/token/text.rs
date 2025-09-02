use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text {
    pub start: usize,
    pub end: usize,
    pub value: String,
}

impl Text {
    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl TryFrom<&mut Scanner<'_>> for Text {
    type Error = Error;

    fn try_from(scan: &mut Scanner<'_>) -> Result<Self, Self::Error> {
        while !scan.is_eof() && scan.peek().unwrap_or(0).is_ascii_alphanumeric() {
            scan.next();
        }

        return Ok(Self {
            start: scan.left(),
            end: scan.right(),
            value: scan.commit().to_string(),
        });
    }
}

impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.value);
    }
}

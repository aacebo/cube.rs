use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{
    Url,
    template::{Expression, Token},
};

/// Template Literal
///
/// Example
/// -------
/// `http://localhost/api/users/1`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Literal(Token);

impl Literal {
    pub fn len(&self) -> usize {
        return self.0.len();
    }

    pub fn start(&self) -> usize {
        return self.0.start();
    }

    pub fn end(&self) -> usize {
        return self.0.end();
    }

    pub fn eval(&self, scan: &mut Scanner<'_>, url: &mut Url) -> Result<(), Error> {
        while !scan.is_eof() && scan.count() < self.0.len() {
            scan.next();
        }

        if scan.to_string() != self.0.to_string() {
            return Err(Error::from(format!(
                "[cube::url::template] => expected '{}', found '{}' [{}, {}]",
                self.0.to_string(),
                scan.to_string(),
                self.0.start(),
                self.0.end(),
            )));
        }

        scan.commit();
        return Ok(());
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Literal {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Expression, Error> {
        return Ok(Expression::Literal(Self(Token::parse(scan)?)));
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

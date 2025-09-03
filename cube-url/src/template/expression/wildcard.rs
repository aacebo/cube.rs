use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{
    Url,
    template::{Asterisk, Expression, Ident},
};

/// Template Wildcard
///
/// Example
/// -------
/// `http://*/api/users`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wildcard {
    left: Option<Box<Expression>>,
    token: Asterisk,
    right: Option<Box<Expression>>,
}

impl Wildcard {
    pub fn eval(&self, text: &str, url: &mut Url) -> Result<(), Error> {
        return Ok(());
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Wildcard {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Expression, Error> {
        let mut expr = Expression::parse(&mut *scan)?;

        while scan.curr() == b'*' {
            let op = Asterisk::parse(&mut *scan)?;
            let mut right: Option<Box<Expression>> = None;

            if !scan.is_eof() {
                right = Some(Box::new(Expression::parse(&mut *scan)?));
            }

            expr = Expression::Wildcard(Self {
                left: Some(Box::new(expr)),
                token: Asterisk::parse(scan)?,
                right,
            });
        }

        return Ok(expr);
    }
}

impl fmt::Display for Wildcard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = &self.left {
            write!(f, "{}", v)?;
        }

        write!(f, "{}", self.token);

        if let Some(v) = &self.right {
            write!(f, "{}", v)?;
        }

        return Ok(());
    }
}

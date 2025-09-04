use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{
    Url,
    template::{Asterisk, Expression, Ident, Logical},
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
    pub fn len(&self) -> usize {
        return (self.end() - self.start()) + 1;
    }

    pub fn start(&self) -> usize {
        return match &self.left {
            None => self.token.start,
            Some(v) => v.start(),
        };
    }

    pub fn end(&self) -> usize {
        return match &self.right {
            None => self.token.end,
            Some(v) => v.end(),
        };
    }

    pub fn eval(&self, scan: &mut Scanner<'_>, url: &mut Url) -> Result<(), Error> {
        if let Some(left) = &self.left {
            left.eval(scan, url)?;
        }

        if let Some(right) = &self.right {
            scan.next_until_bytes(right.to_string().as_bytes());
        } else {
            scan.next_until_end();
        }

        scan.commit();

        if let Some(right) = &self.right {
            right.eval(scan, url)?;
        }

        return Ok(());
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Wildcard {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Expression, Error> {
        let mut expr = Logical::parse(&mut *scan)?;

        while scan.curr() == b'*' {
            let token = Asterisk::parse(&mut *scan)?;
            let mut right: Option<Box<Expression>> = None;

            if !scan.is_eof() {
                right = Some(Box::new(Logical::parse(&mut *scan)?));
            }

            expr = Expression::Wildcard(Self {
                left: Some(Box::new(expr)),
                token,
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

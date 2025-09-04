use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{
    Url,
    template::{Expression, Ident, Wildcard},
};

/// Template Variable
///
/// Example
/// -------
/// `http://localhost/api/users/{user}/posts`
/// vs
/// `http://localhost/api/users/1234567/posts`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Var {
    left: Option<Box<Expression>>,
    name: Ident,
    right: Option<Box<Expression>>,
}

impl Var {
    pub fn len(&self) -> usize {
        return (self.end() - self.start()) + 1;
    }

    pub fn start(&self) -> usize {
        return match &self.left {
            None => self.name.start,
            Some(v) => v.start(),
        };
    }

    pub fn end(&self) -> usize {
        return match &self.right {
            None => self.name.end,
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

        url.params.set(&self.name.name, scan.commit());

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

impl Var {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Expression, Error> {
        let mut expr = Wildcard::parse(&mut *scan)?;

        while scan.curr() == b'{' {
            let name = Ident::parse(&mut *scan)?;
            let mut right: Option<Box<Expression>> = None;

            if !scan.is_eof() {
                right = Some(Box::new(Wildcard::parse(&mut *scan)?));
            }

            expr = Expression::Var(Self {
                left: Some(Box::new(expr)),
                name,
                right,
            });
        }

        return Ok(expr);
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(left) = &self.left {
            write!(f, "{}", left)?;
        }

        write!(f, "{}", self.name)?;

        if let Some(right) = &self.right {
            write!(f, "{}", right)?;
        }

        return Ok(());
    }
}

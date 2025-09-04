use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{
    Url,
    template::{Expression, Literal, Pipe},
};

/// Template Logical
///
/// Example
/// -------
/// `http://localhost/api/users/(a|b)`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Logical {
    left: Box<Expression>,
    op: Pipe,
    right: Box<Expression>,
}

impl Logical {
    pub fn len(&self) -> usize {
        return self.left.len() + 1 + self.right.len();
    }

    pub fn start(&self) -> usize {
        return self.left.start();
    }

    pub fn end(&self) -> usize {
        return self.right.end();
    }

    pub fn eval(&self, scan: &mut Scanner<'_>, url: &mut Url) -> Result<(), Error> {
        let mut first = &self.left;
        let mut second = &self.right;

        if first.len() < self.right.len() {
            first = &self.right;
            second = &self.left;
        }

        let left = first.eval(scan, url);

        if left.is_ok() {
            return Ok(());
        }

        scan.reset();
        let right = second.eval(scan, url);

        if right.is_ok() {
            return Ok(());
        }

        scan.reset();
        return Err(Error::from(format!(
            "[cube::url::template] => expected '{}'",
            Error::new()
                .push(&left.unwrap_err())
                .push(&right.unwrap_err()),
        )));
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Logical {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Expression, Error> {
        let mut expr = Literal::parse(&mut *scan)?;

        while scan.curr() == b'|' {
            let op = Pipe::parse(&mut *scan)?;
            let right = Literal::parse(&mut *scan)?;

            expr = Expression::Logical(Self {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            });
        }

        return Ok(expr);
    }
}

impl fmt::Display for Logical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}{}{}", self.left, self.op, self.right);
    }
}

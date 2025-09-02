use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{
    Url,
    template::{Expression, Literal, Or},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Logical {
    left: Box<Expression>,
    op: Or,
    right: Box<Expression>,
}

impl Logical {
    pub fn eval(&self, text: &str, url: &mut Url) -> Result<(), Error> {
        return Ok(());
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
            let op = Or::try_from(&mut *scan)?;
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

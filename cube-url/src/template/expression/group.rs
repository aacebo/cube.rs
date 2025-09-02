use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{Url, template::Expression};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Group(Vec<Expression>);

impl Group {
    pub fn eval(&self, text: &str, url: &mut Url) -> Result<(), Error> {
        for expr in self.0.iter() {
            expr.eval(text, url)?;
        }

        return Ok(());
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Group {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Expression, Error> {
        let mut expressions = Vec::<Expression>::new();
        scan.fshift(1);

        while !scan.is_eof() && scan.curr() != b')' {
            expressions.push(Expression::parse(&mut *scan)?);
        }

        scan.fshift(1);
        return Ok(Expression::Group(Self(expressions)));
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;

        for expr in self.0.iter() {
            write!(f, "{}", expr)?;
        }

        write!(f, ")")?;
        return Ok(());
    }
}

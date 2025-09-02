#![allow(unused)]

mod token;
pub(crate) use token::*;

mod expression;
pub(crate) use expression::*;

use cube_core::error::Error;

use crate::Url;

/// https://datatracker.ietf.org/doc/html/rfc6570
#[derive(Debug, Clone)]
pub struct Template(Vec<Expression>);

impl Template {
    pub fn new(pattern: &str) -> Result<Self, Error> {
        let mut expressions = Vec::<Expression>::new();
        let parts = pattern.split("/");

        for part in parts {
            expressions.push(Expression::new(part)?);
        }

        return Ok(Self(expressions));
    }

    pub fn parse(&self, url: &str) -> Result<Url, Error> {
        let mut uri = Url::parse(url)?;
        let path = uri.path.clone();
        let parts = path.split("/").filter(|v| !v.is_empty());

        for (i, text) in parts.enumerate() {
            self.0[i].eval(text, &mut uri)?;
        }

        return Ok(uri);
    }
}

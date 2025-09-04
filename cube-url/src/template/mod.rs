#![allow(unused)]

use std::fmt;

mod token;
pub(crate) use token::*;

mod expression;
pub(crate) use expression::*;

use cube_core::{bytes::Scanner, error::Error};

use crate::Url;

/// https://datatracker.ietf.org/doc/html/rfc6570
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Template(Vec<Expression>);

impl Template {
    pub fn parse(pattern: &str) -> Result<Self, Error> {
        let mut scan = Scanner::from(pattern);
        let mut expressions = Vec::<Expression>::new();

        while !scan.is_eof() {
            expressions.push(Expression::parse(&mut scan)?);
        }

        return Ok(Self(expressions));
    }

    pub fn eval(&self, url: &str) -> Result<Url, Error> {
        let mut uri = Url::parse(url)?;
        let mut scan = Scanner::from(url);

        for expr in self.0.iter() {
            expr.eval(&mut scan, &mut uri)?;
        }

        return Ok(uri);
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for expr in self.0.iter() {
            write!(f, "{}", expr);
        }

        return Ok(());
    }
}

#[cfg(test)]
mod test {
    use cube_core::bytes::Scanner;

    #[test]
    pub fn should_parse() {
        let mut template = super::Template::parse("http://*lh*:3000/(a|b)?hello={world}").unwrap();
        template.0.reverse();
        let mut expr = template.0.pop().unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "http");

        expr = template.0.pop().unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), ":");

        expr = template.0.pop().unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "/");

        expr = template.0.pop().unwrap();

        assert!(expr.is_wildcard());
        assert_eq!(expr.to_string(), "/*lh*:");

        expr = template.0.pop().unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "3000");

        expr = template.0.pop().unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "/");

        expr = template.0.pop().unwrap();

        assert!(expr.is_group());
        assert_eq!(expr.to_string(), "(a|b)");

        expr = template.0.pop().unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "?");

        expr = template.0.pop().unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "hello");

        expr = template.0.pop().unwrap();

        assert!(expr.is_var());
        assert_eq!(expr.to_string(), "={world}");
        assert!(template.0.is_empty());
    }

    #[test]
    pub fn should_stringify() {
        let template = super::Template::parse("http://localhost:3000/(a|b)?hello={world}").unwrap();

        assert_eq!(
            template.to_string(),
            "http://localhost:3000/(a|b)?hello={world}"
        );
    }

    #[test]
    pub fn should_evaluate() {
        let mut template =
            super::Template::parse("http://*:3000/(user|users)/{user}/orgs/{org_id}").unwrap();

        let mut url = template
            .eval("http://localhost:3000/user/1/orgs/test")
            .unwrap();

        assert_eq!(url.params.get("user").unwrap(), "1");
        assert_eq!(url.params.get("org_id").unwrap(), "test");

        url = template
            .eval("http://localhost:3000/users/1234/orgs/myorgid")
            .unwrap();

        assert_eq!(url.params.get("user").unwrap(), "1234");
        assert_eq!(url.params.get("org_id").unwrap(), "myorgid");
    }
}

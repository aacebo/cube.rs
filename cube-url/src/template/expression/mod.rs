mod var;
pub(crate) use var::*;

mod group;
pub(crate) use group::*;

mod logical;
pub(crate) use logical::*;

mod literal;
pub(crate) use literal::*;

mod wildcard;
pub(crate) use wildcard::*;

use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::Url;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Expression {
    Var(Var),
    Group(Group),
    Logical(Logical),
    Literal(Literal),
    Wildcard(Wildcard),
}

impl Expression {
    pub fn new(pattern: &str) -> Result<Self, Error> {
        let mut scan = Scanner::from(pattern);
        return Self::parse(&mut scan);
    }

    pub fn is_var(&self) -> bool {
        return match self {
            Self::Var(_) => true,
            _ => false,
        };
    }

    pub fn is_group(&self) -> bool {
        return match self {
            Self::Group(_) => true,
            _ => false,
        };
    }

    pub fn is_logical(&self) -> bool {
        return match self {
            Self::Logical(_) => true,
            _ => false,
        };
    }

    pub fn is_literal(&self) -> bool {
        return match self {
            Self::Literal(_) => true,
            _ => false,
        };
    }

    pub fn is_wildcard(&self) -> bool {
        return match self {
            Self::Wildcard(_) => true,
            _ => false,
        };
    }

    pub fn eval(&self, text: &str, url: &mut Url) -> Result<(), Error> {
        return match self {
            Self::Group(v) => v.eval(text, url),
            Self::Literal(v) => v.eval(text, url),
            Self::Logical(v) => v.eval(text, url),
            Self::Var(v) => v.eval(text, url),
            Self::Wildcard(v) => v.eval(text, url),
        };
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Expression {
    fn parse(scan: &mut Scanner<'_>) -> Result<Self, Error> {
        return match scan.curr() {
            b'{' => Ok(Var::parse(scan)?),
            b'(' => Ok(Group::parse(scan)?),
            b'*' => Ok(Wildcard::parse(scan)?),
            _ => Ok(Logical::parse(scan)?),
        };
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Var(v) => write!(f, "{}", v),
            Self::Group(v) => write!(f, "{}", v),
            Self::Logical(v) => write!(f, "{}", v),
            Self::Literal(v) => write!(f, "{}", v),
            Self::Wildcard(v) => write!(f, "{}", v),
        };
    }
}

#[cfg(test)]
mod test {
    use cube_core::bytes::Scanner;

    #[test]
    pub fn should_parse() {
        let mut scan = Scanner::from("http://localhost:3000/(a|b)?hello={world}");
        let mut expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "http");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), ":");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "/");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "/");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "localhost");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), ":");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "3000");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "/");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_group());
        assert_eq!(expr.to_string(), "(a|b)");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "?");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "hello");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_literal());
        assert_eq!(expr.to_string(), "=");

        expr = super::Expression::parse(&mut scan).unwrap();

        assert!(expr.is_var());
        assert_eq!(expr.to_string(), "{world}");
        assert!(scan.is_eof());
    }
}

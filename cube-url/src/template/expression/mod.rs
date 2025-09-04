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

    pub fn len(&self) -> usize {
        return (self.end() - self.start()) + 1;
    }

    pub fn start(&self) -> usize {
        return match self {
            Self::Group(v) => v.start(),
            Self::Literal(v) => v.start(),
            Self::Logical(v) => v.start(),
            Self::Var(v) => v.start(),
            Self::Wildcard(v) => v.start(),
        };
    }

    pub fn end(&self) -> usize {
        return match self {
            Self::Group(v) => v.end(),
            Self::Literal(v) => v.end(),
            Self::Logical(v) => v.end(),
            Self::Var(v) => v.end(),
            Self::Wildcard(v) => v.end(),
        };
    }

    pub fn eval(&self, scan: &mut Scanner<'_>, url: &mut Url) -> Result<(), Error> {
        return match self {
            Self::Group(v) => v.eval(scan, url),
            Self::Literal(v) => v.eval(scan, url),
            Self::Logical(v) => v.eval(scan, url),
            Self::Var(v) => v.eval(scan, url),
            Self::Wildcard(v) => v.eval(scan, url),
        };
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Expression {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Self, Error> {
        return match scan.curr() {
            b'(' => Ok(Group::parse(scan)?),
            _ => Ok(Var::parse(scan)?),
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
        let mut scan = Scanner::from("http://localhost:*/(a|b)?hello={world}");
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

        assert!(expr.is_wildcard());
        assert_eq!(expr.to_string(), ":*/");

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

        assert!(expr.is_var());
        assert_eq!(expr.to_string(), "={world}");
        assert!(scan.is_eof());
    }
}

use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{
    Url,
    template::{Expression, Ident},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Var {
    name: Ident,
}

impl Var {
    pub fn eval(&self, text: &str, url: &mut Url) -> Result<(), Error> {
        return Ok(());
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Var {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Expression, Error> {
        return Ok(Expression::Var(Self {
            name: Ident::try_from(scan)?,
        }));
    }
}

impl fmt::Display for Var {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.name);
    }
}

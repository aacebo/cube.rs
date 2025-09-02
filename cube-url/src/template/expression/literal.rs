use std::fmt;

use cube_core::{bytes::Scanner, error::Error};

use crate::{
    Url,
    template::{Expression, Token},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Literal(Token);

impl Literal {
    pub fn eval(&self, text: &str, url: &mut Url) -> Result<(), Error> {
        if text != self.0.to_string() {
            return Err(Error::from(format!(
                "[cube::url::template] => expected '{}'",
                self.0.to_string()
            )));
        }

        return Ok(());
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Literal {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Expression, Error> {
        return Ok(Expression::Literal(Self(Token::try_from(scan)?)));
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}

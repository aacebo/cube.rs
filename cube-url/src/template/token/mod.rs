use std::fmt;

mod text;
pub(crate) use text::*;

mod ident;
pub(crate) use ident::*;

mod interrogate;
pub(crate) use interrogate::*;

mod asterisk;
pub(crate) use asterisk::*;

mod ampersand;
pub(crate) use ampersand::*;

mod colon;
pub(crate) use colon::*;

mod slash;
pub(crate) use slash::*;

mod hash;
pub(crate) use hash::*;

mod equals;
pub(crate) use equals::*;

mod exclamation;
pub(crate) use exclamation::*;

mod pipe;
pub(crate) use pipe::*;

use cube_core::{bytes::Scanner, error::Error};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Token {
    Ampersand(Ampersand),
    Asterisk(Asterisk),
    Colon(Colon),
    Equals(Equals),
    Exclamation(Exclamation),
    Hash(Hash),
    Ident(Ident),
    Interrogate(Interrogate),
    Pipe(Pipe),
    Slash(Slash),
    Text(Text),
}

impl Token {
    pub fn is_ampersand(&self) -> bool {
        return match self {
            Self::Ampersand(_) => true,
            _ => false,
        };
    }

    pub fn is_asterisk(&self) -> bool {
        return match self {
            Self::Asterisk(_) => true,
            _ => false,
        };
    }

    pub fn is_colon(&self) -> bool {
        return match self {
            Self::Colon(_) => true,
            _ => false,
        };
    }

    pub fn is_equals(&self) -> bool {
        return match self {
            Self::Equals(_) => true,
            _ => false,
        };
    }

    pub fn is_exclamation(&self) -> bool {
        return match self {
            Self::Exclamation(_) => true,
            _ => false,
        };
    }

    pub fn is_hash(&self) -> bool {
        return match self {
            Self::Hash(_) => true,
            _ => false,
        };
    }

    pub fn is_ident(&self) -> bool {
        return match self {
            Self::Ident(_) => true,
            _ => false,
        };
    }

    pub fn is_interrogate(&self) -> bool {
        return match self {
            Self::Interrogate(_) => true,
            _ => false,
        };
    }

    pub fn is_pipe(&self) -> bool {
        return match self {
            Self::Pipe(_) => true,
            _ => false,
        };
    }

    pub fn is_slash(&self) -> bool {
        return match self {
            Self::Slash(_) => true,
            _ => false,
        };
    }

    pub fn is_text(&self) -> bool {
        return match self {
            Self::Text(_) => true,
            _ => false,
        };
    }

    #[cfg(feature = "serde")]
    pub fn to_json(&self) -> String {
        return serde_json::to_string_pretty(self).unwrap();
    }
}

impl Token {
    pub fn parse(scan: &mut Scanner<'_>) -> Result<Self, Error> {
        return match scan.curr() {
            b'&' => Ok(Self::Ampersand(Ampersand::parse(scan)?)),
            b'*' => Ok(Self::Asterisk(Asterisk::parse(scan)?)),
            b':' => Ok(Self::Colon(Colon::parse(scan)?)),
            b'=' => Ok(Self::Equals(Equals::parse(scan)?)),
            b'!' => Ok(Self::Exclamation(Exclamation::parse(scan)?)),
            b'#' => Ok(Self::Hash(Hash::parse(scan)?)),
            b'{' => Ok(Self::Ident(Ident::parse(scan)?)),
            b'?' => Ok(Self::Interrogate(Interrogate::parse(scan)?)),
            b'|' => Ok(Self::Pipe(Pipe::parse(scan)?)),
            b'/' => Ok(Self::Slash(Slash::parse(scan)?)),
            _ => Ok(Self::Text(Text::parse(scan)?)),
        };
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Ampersand(v) => write!(f, "{}", v),
            Self::Asterisk(v) => write!(f, "{}", v),
            Self::Colon(v) => write!(f, "{}", v),
            Self::Equals(v) => write!(f, "{}", v),
            Self::Exclamation(v) => write!(f, "{}", v),
            Self::Hash(v) => write!(f, "{}", v),
            Self::Ident(v) => write!(f, "{}", v),
            Self::Interrogate(v) => write!(f, "{}", v),
            Self::Pipe(v) => write!(f, "{}", v),
            Self::Slash(v) => write!(f, "{}", v),
            Self::Text(v) => write!(f, "{}", v),
        };
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        return match self {
            Self::Ampersand(v) => v.eq(other),
            Self::Asterisk(v) => v.eq(other),
            Self::Colon(v) => v.eq(other),
            Self::Equals(v) => v.eq(other),
            Self::Exclamation(v) => v.eq(other),
            Self::Hash(v) => v.eq(other),
            Self::Ident(v) => v.eq(other),
            Self::Interrogate(v) => v.eq(other),
            Self::Pipe(v) => v.eq(other),
            Self::Slash(v) => v.eq(other),
            Self::Text(v) => v.eq(other),
        };
    }
}

#[cfg(test)]
mod test {
    use cube_core::bytes::Scanner;

    #[test]
    pub fn should_parse() {
        let mut scan = Scanner::from("http://localhost:3000/test?hello={world}");
        let mut token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_text());
        assert_eq!(token.to_string(), "http");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_colon());
        assert_eq!(token.to_string(), ":");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_slash());
        assert_eq!(token.to_string(), "/");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_slash());
        assert_eq!(token.to_string(), "/");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_text());
        assert_eq!(token.to_string(), "localhost");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_colon());
        assert_eq!(token.to_string(), ":");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_text());
        assert_eq!(token.to_string(), "3000");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_slash());
        assert_eq!(token.to_string(), "/");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_text());
        assert_eq!(token.to_string(), "test");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_interrogate());
        assert_eq!(token.to_string(), "?");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_text());
        assert_eq!(token.to_string(), "hello");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_equals());
        assert_eq!(token.to_string(), "=");

        token = super::Token::parse(&mut scan).unwrap();

        assert!(token.is_ident());
        assert_eq!(token.to_string(), "{world}");
        assert!(scan.is_eof());
    }
}

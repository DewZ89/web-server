use super::method::Method;
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
};

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
impl Request {}

impl TryFrom<&[u8]> for Request {
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }

    type Error = String;
}

pub enum ParseError {
    InvalidRequest,
    InvalidMethod,
    InvalidEncoding,
    InvalidProtocol,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "InvalidRequest",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidMethod => "InvalidMethod",
            Self::InvalidProtocol => "InvalidProtocol",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

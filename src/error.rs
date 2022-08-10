//! Error and Result types.
use std::error::Error as StdError;
use std::fmt::{self, Debug, Display};
use std::io;

use serde::de::Visitor;
use serde::ser::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

/// in service::sys_user, this would be uiseful
pub type Result<T> = std::result::Result<T, Error>;

/// A generic error that reprsents all
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Default Error
    E(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::E(error) => write!(f, "{}", error),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    #[inline]
    fn from(err: io::Error) -> Self {
        Error::from(err.to_string())
    }
}

impl From<&str> for Error {
    fn from(arg: &str) -> Self {
        return Error::E(arg.to_string());
    }
}

impl From<std::string::String> for Error {
    fn from(arg: String) -> Self {
        return Error::E(arg);
    }
}

impl From<&dyn std::error::Error> for Error {
    fn from(arg: &dyn std::error::Error) -> Self {
        return Error::E(arg.to_string());
    }
}

/// ERROR 实现 CLONE trait
/// 在vo 中 RespVO 会用到这一 trait
impl Clone for Error {
    fn clone(&self) -> Self {
        Error::from(self.to_string())
    }

    fn clone_from(&mut self, source: &Self) {
        *self = Self::from(source.to_string());
    }
}

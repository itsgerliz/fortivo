use std::fmt::Display;
use serde::{de, ser};
use thiserror::Error;

/// Type alias for errors in this crate
pub(crate) type FortivoResult<T> = Result<T, FortivoError>;

/// Main error type used by libfortivo
#[derive(Error, Debug)]
pub enum FortivoError {
    #[error("Encountered an error trying to create a new Arca header")]
    ArcaHeader(#[from] ArcaHeaderError),
    #[error("Encountered an error trying to process a timestamp")]
    Time(#[from] std::time::SystemTimeError),
    #[error("Encountered an input/output error")]
    IO(#[from] std::io::Error),
    #[error("Encountered an error trying to serialize")]
    Serialize(String),
    #[error("Encountered an error trying to deserialize")]
    Deserialize(String),
}

#[derive(Error, Debug)]
pub enum ArcaHeaderError {
    #[error("Magic byte is invalid")]
    MagicByteInvalid,
    #[error("Arca name is too long")]
    NameTooLong,
    #[error("Name length field does not match actual name field length")]
    NameLengthsDoNotMatch,
    #[error("Timestamp/s is/are above current system time")]
    TimestampAboveCurrentSystemTime,
    #[error("Provided flags are not allowed in this engine version")]
    FlagsNotAllowed,
    #[error("Incompatible engine version")]
    IncompatibleEngineVersion
}

impl ser::Error for FortivoError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Serialize(msg.to_string())
    }
}

impl de::Error for FortivoError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Deserialize(msg.to_string())
    }
}
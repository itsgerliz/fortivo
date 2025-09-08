use std::fmt::Display;
use serde::{de, ser};
use thiserror::Error;

/// Type alias for errors in this crate
pub(crate) type FortivoResult<T> = Result<T, FortivoError>;

/// Main error type used by libfortivo
#[derive(Error, Debug)]
pub enum FortivoError {
    #[error("Encountered an IO error")]
    IO(#[from] std::io::Error),
    #[error("Encountered an error trying to serialize")]
    Serialize(String),
    #[error("Encountered an error trying to deserialize")]
    Deserialize(String),
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
use thiserror::Error;
use serde::ser;
use std::fmt::Display;

/// Type alias for errors in this crate
pub type FortivoResult<T> = Result<T, FortivoError>;

/// The main error type used by this crate, it implements the [`std::error::Error`] trait
#[derive(Error, Debug)]
pub enum FortivoError {
	#[error("Encountered an IO error")]
	IO(#[from] std::io::Error),
	#[error("Is clock correctly set?")]
	Time(#[from] std::time::SystemTimeError),
	#[error("Encountered an error trying to serialize")]
	Serialize(String),
	#[error("Arca name is too long, maximum allowed is 512 bytes")]
	ArcaNameTooLong
}

// Implement serde's Error trait so this error type can be used with it
impl ser::Error for FortivoError {
	fn custom<T: Display>(msg: T) -> Self {
		Self::Serialize(msg.to_string())
	}
}
use std::error::Error;
use thiserror::Error;

/// Type alias for errors in this crate
type Result<T> = std::result::Result<T, FortivoError>;

/// The main error type used by this crate, it implements the [`std::error::Error`] trait
#[derive(Error, Debug)]
pub enum FortivoError {
	#[error("An IO error has ocurred")]
	IOError(#[from] std::io::Error)
}

impl FortivoError {
	/// Print this error, and its cause if a source exists
	pub fn print_error(&self) {
		eprintln!("{}", self);

		if let Some(source) = self.source() {
			eprintln!("Caused by: {}", source);
		}
	}
}
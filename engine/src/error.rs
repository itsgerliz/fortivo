use std::error::Error;
use thiserror::Error;

/// The main error type used by this crate, it implements the [`std::error::Error`] trait
#[derive(Error, Debug)]
pub enum FortivoError {
	#[error("An IO error has ocurred")]
	IOError(#[from] std::io::Error)
}

pub(crate) fn print_fortivo_error(err: &FortivoError) {
	eprintln!("{}", err);

	if let Some(source) = err.source() {
		eprintln!("Caused by: {}", source);
	}
}
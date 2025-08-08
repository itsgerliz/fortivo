use thiserror::Error;

/// Type alias for errors in this crate
pub type FortivoResult<T> = Result<T, FortivoError>;

/// The main error type used by this crate, it implements the [`std::error::Error`] trait
#[derive(Error, Debug)]
pub enum FortivoError {
	#[error("An IO error has ocurred")]
	IOError(#[from] std::io::Error)
}
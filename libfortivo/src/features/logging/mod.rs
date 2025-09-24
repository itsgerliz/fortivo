#[cfg(feature = "logging")]
macro_rules! conditional_debug {
	($($arg:tt)*) => { log::debug!($($arg)*) }
}

#[cfg(not(feature = "logging"))]
macro_rules! conditional_debug {
	($($arg:tt)*) => {}
}

#[cfg(feature = "logging")]
macro_rules! conditional_trace {
	($($arg:tt)*) => { log::trace!($($arg)*) }
}

#[cfg(not(feature = "logging"))]
macro_rules! conditional_trace {
	($($arg:tt)*) => {}
}

pub(crate) use {conditional_debug, conditional_trace};
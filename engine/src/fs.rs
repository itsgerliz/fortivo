use std::{fs::{self, File}, path::Path};
use crate::error::FortivoResult;

pub struct Arca {
	handle: File,
	is_dirty: bool
}

pub fn new_arca<P: AsRef<Path>>(pathname: P) -> FortivoResult<Arca> {
	Ok(
		Arca {
			handle: File::create_new(pathname)?,
			is_dirty: false
		}
	)
}

pub fn delete_arca<P: AsRef<Path>>(pathname: P) -> FortivoResult<()> {
	Ok(
		fs::remove_file(pathname)?
	)
}
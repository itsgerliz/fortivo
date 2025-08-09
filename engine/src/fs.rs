use std::{fs::{self, File}, path::{Path, PathBuf}};
use crate::error::FortivoResult;

pub struct Arca {
	handle: File,
	path: PathBuf,
	is_dirty: bool
}

pub fn new_arca<P: AsRef<Path>>(pathname: P) -> FortivoResult<Arca> {
	Ok(
		Arca {
			handle: File::create_new(&pathname)?,
			path: pathname.as_ref().to_path_buf(),
			is_dirty: false
		}
	)
}

pub fn delete_arca(arca: Arca) -> FortivoResult<()> {
	Ok(
		fs::remove_file(arca.path)?
	)
}
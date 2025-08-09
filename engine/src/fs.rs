use std::{fs::{self, File}, path::{Path, PathBuf}};
use crate::error::FortivoResult;

pub struct Arca {
	handle: File,
	path: PathBuf,
	is_dirty: bool
}

impl Arca {
	pub fn new<P: AsRef<Path>>(pathname: P) -> FortivoResult<Self> {
		Ok(
			Arca {
				handle: File::create_new(&pathname)?,
				path: pathname.as_ref().to_path_buf(),
				is_dirty: false
			}
		)
	}

	pub fn delete(self) -> FortivoResult<()> {
		Ok(fs::remove_file(self.path)?)
	}

	pub fn check_dirty(&self) -> bool {
		self.is_dirty
	}
}
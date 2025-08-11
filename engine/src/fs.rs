use std::{fs::{self, File, OpenOptions}, path::{Path, PathBuf}};
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

	pub fn open<P: AsRef<Path>>(pathname: P, read_write: bool) -> FortivoResult<Self> {
		if read_write {
			Ok(
				Arca {
					handle: OpenOptions::new().read(true).write(true).open(&pathname)?,
					path: pathname.as_ref().to_path_buf(),
					is_dirty: false
				}
			)
		} else {
			Ok(
				Arca {
					handle: File::open(&pathname)?,
					path: pathname.as_ref().to_path_buf(),
					is_dirty: false
				}
			)
		}
	}

	pub fn delete(self) -> FortivoResult<()> {
		Ok(fs::remove_file(self.path)?)
	}

	pub fn check_dirty(&self) -> bool {
		self.is_dirty
	}
}

pub struct ArcaHeader {

}

pub struct Arcanum {

}
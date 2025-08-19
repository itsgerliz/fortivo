use std::{fs::{self, File, OpenOptions}, io::{self, Read, Write}, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};
use crate::error::FortivoResult;

pub struct Arca {
	handle: File,
	path: PathBuf,
	is_dirty: bool,
	header: ArcaHeader
}

#[derive(Serialize, Deserialize)]
pub struct ArcaHeader {

}

#[derive(Serialize, Deserialize)]
pub struct ArcaObject {

}

impl Arca {
	pub fn new<P: AsRef<Path>>(pathname: P, new_header: ArcaHeader) -> FortivoResult<Self> {
		Ok(
			Arca {
				handle: File::create_new(&pathname)?,
				path: pathname.as_ref().to_path_buf(),
				is_dirty: false,
				header: new_header
			}
		)
	}

	pub fn open<P: AsRef<Path>>(pathname: P, read_write: bool) -> FortivoResult<Self> {
		if read_write {
			let file_handle = OpenOptions::new().read(true).write(true).open(&pathname)?;
			let new_header = ArcaHeader { }; //TODO

			Ok(
				Arca {
					handle: file_handle,
					path: pathname.as_ref().to_path_buf(),
					is_dirty: false,
					header: new_header
				}
			)
		} else {
			let file_handle = File::open(&pathname)?;
			let new_header = ArcaHeader { }; //TODO

			Ok(
				Arca {
					handle: file_handle,
					path: pathname.as_ref().to_path_buf(),
					is_dirty: false,
					header: new_header
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

	pub fn write_header(&self) -> FortivoResult<usize> {
		todo!()
	}
}

impl Read for Arca {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.handle.read(buf)
	}
}

impl Write for Arca {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.handle.write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.handle.flush()
	}
}
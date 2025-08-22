use std::{fs::File, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use serde::{Serialize, Deserialize};
use crate::error::{FortivoError, FortivoResult};

pub struct Arca {
	handle: File,
	path: PathBuf,
	is_dirty: bool,
	header: ArcaHeader
}

#[derive(Serialize, Deserialize)]
pub struct ArcaHeader {
	magic_byte: u8,
	name_length: u16,
	name: Vec<u8>, // This should not be bigger than 512 elements (bytes)
	creation_date: u64,
	modification_date: u64,
	object_count: u64,
	flags: u16
}

impl ArcaHeader {
	pub fn new<N: AsRef<str>>(name: N) -> FortivoResult<Self> {
		let name_str = name.as_ref();
		let name_len = name_str.len();
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)?
			.as_secs();

		if name_len > 512 {
			return Err(FortivoError::ArcaNameTooLong)
		}

		Ok(
			ArcaHeader {
				magic_byte: 0xCF,
				name_length: name_len as u16,
				name: name_str.as_bytes().to_vec(),
				creation_date: now,
				modification_date: now,
				object_count: 0,
				flags: 0
			}
		)
	}
}

pub struct ArcaObject {
	// TODO
}
use std::{fs::File, path::{Path, PathBuf}};
use crate::error::FortivoResult;

pub struct Arca {
    handle: File,
    path: PathBuf,
    header: ArcaHeader,
}

impl Arca {
    pub fn new<P: AsRef<Path>>(path: P) -> FortivoResult<Self> {
        todo!()
    }

    pub fn open<P: AsRef<Path>>(path: P) -> FortivoResult<Self> {
        todo!()
    }
}

pub struct ArcaHeader {
    pub(crate) magic_byte: u8,
    pub(crate) name_length: u16,
    pub(crate) name: [u8; 512],
    pub(crate) creation_date: u64,
    pub(crate) modification_date: u64,
    pub(crate) arcanum_count: u64,
    pub(crate) flags: u16,
}

impl ArcaHeader {
    pub fn new() -> Self {
        ArcaHeader {
            magic_byte: 0xCF,
            name_length: 0,
            name: [0; 512],
            creation_date: 0,
            modification_date: 0,
            arcanum_count: 0,
            flags: 0,
        }
    }
}

use std::{fs::File, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};
use serde::{Deserialize, Serialize};
use crate::error::{FortivoError, FortivoResult, NewArcaHeaderError};

pub struct Arca<'a> {
    handle: File,
    path: PathBuf,
    header: ArcaHeader<'a>,
}

impl<'a> Arca<'a> {
    pub fn new<P: AsRef<Path>>(path: P) -> FortivoResult<Self> {
        todo!()
    }

    pub fn open<P: AsRef<Path>>(path: P) -> FortivoResult<Self> {
        todo!()
    }
}

const ALLOWED_FLAGS: u16 = 0b0000_0000_0000_0000;

#[derive(Serialize, Deserialize)]
pub struct ArcaHeader<'a> {
    magic_byte: u8,
    name_length: u16, // Should not be > 512
    name: &'a [u8],
    creation_date: u64,
    modification_date: u64,
    arcanum_count: u64,
    flags: u16,
}

impl<'a> ArcaHeader<'a> {
    pub fn new(name: &'a [u8], timestamp: u64, flags: u16) -> FortivoResult<Self> {
        let name_length = name.len();

        if name_length > 512 {
            return Err(FortivoError::from(NewArcaHeaderError::NameTooLong))
        }

        if timestamp > SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() {
            return Err(FortivoError::from(NewArcaHeaderError::TimestampAboveCurrentSystemTime))
        }

        if (!flags & ) // TODO

        Ok(
            ArcaHeader {
                magic_byte: 0xCF,
                name_length: name_length as u16,
                name: name,
                creation_date: timestamp,
                modification_date: timestamp,
                arcanum_count: 0,
                flags: flags
            }
        )
    }
}

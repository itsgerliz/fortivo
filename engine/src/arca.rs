use std::{fs::File, path::Path, time::{SystemTime, UNIX_EPOCH}};
use serde::{Deserialize, Serialize};
use crate::{error::{ArcaHeaderError, FortivoError, FortivoResult}, serde::deserializer::ArcaHeaderDeserializer};

pub const ALLOWED_FLAGS: u16 = 0b0000_0000_0000_0000;
pub const ENGINE_VERSION: [u64; 3] = [0, 1, 0];

pub struct Arca {
    handle: File,
    header: ArcaHeader,
}

impl Arca {
    pub fn new<P: AsRef<Path>>(path: P, name: &[u8], flags: u16) -> FortivoResult<Self> {
        Ok(
            Self {
                handle: File::create_new(path.as_ref())?,
                header: ArcaHeader::new(name, flags)?
            }
        )
    }

    pub fn open<P: AsRef<Path>>(path: P) -> FortivoResult<Self> {
        let file_handle = File::options().read(true).write(true).open(path.as_ref())?;
        let arca_header = ArcaHeader::deserialize(ArcaHeaderDeserializer { reader: &file_handle })?;

        // TODO check valid header and construct
        Ok(
            Self {

            }
        )
    }
}

#[derive(Serialize, Deserialize)]
struct ArcaHeader {
    magic_byte: u8,
    name_length: u16,
    name: Vec<u8>,
    creation_date: u64,
    modification_date: u64,
    arcanum_count: u64,
    flags: u16,
    engine_version: [u64; 3]
}

impl ArcaHeader {
    fn new(name: &[u8], flags: u16) -> FortivoResult<Self> {
        let name_length = name.len();
        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

        if name_length > 512 {
            return Err(FortivoError::from(ArcaHeaderError::NameTooLong))
        }

        if (flags & !ALLOWED_FLAGS) != 0 {
            return Err(FortivoError::from(ArcaHeaderError::FlagsNotAllowed));
        }

        Ok(
            Self {
                magic_byte: 0xCF,
                name_length: name_length as u16,
                name: name.to_vec(),
                creation_date: current_time,
                modification_date: current_time,
                arcanum_count: 0,
                flags: flags,
                engine_version: ENGINE_VERSION
            }
        )
    }

    fn validate(&self) -> FortivoResult<()> {
        if self.magic_byte != 0xCF {
            return Err(FortivoError::from(ArcaHeaderError::MagicByteInvalid))
        }

        if self.name_length > 512 {
            return Err(FortivoError::from(ArcaHeaderError::NameTooLong))
        }

        if self.name_length as usize != self.name.len() {
            return Err(FortivoError::from(ArcaHeaderError::NameLengthsDoNotMatch))
        }

        let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        if self.creation_date > current_time || self.modification_date > current_time {
            return Err(FortivoError::from(ArcaHeaderError::TimestampAboveCurrentSystemTime))
        }

        // arcanum_count field has no validity check, it cannot be negative due to type
        // bounds and checking if the Arcanum count in this Arca matches the arcanum_count
        // would be a waste of time since one single Arca could theorethically contain
        // up to 2^64 Arcanums

        if (self.flags & !ALLOWED_FLAGS) != 0 {
            return Err(FortivoError::from(ArcaHeaderError::FlagsNotAllowed))
        }

        if ENGINE_VERSION < self.engine_version {
            return Err(FortivoError::from(ArcaHeaderError::IncompatibleEngineVersion))
        }

        Ok(())
    }
}

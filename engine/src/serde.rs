use crate::{error::FortivoResult, fs::{ArcaHeader, Arcanum}};

trait Serdeable: Sized {
	fn serialize(&self) -> FortivoResult<&[u8]>;
	fn deserialize(src: &[u8]) -> FortivoResult<Self>;
}

impl Serdeable for ArcaHeader {
	fn serialize(&self) -> FortivoResult<&[u8]> {
		todo!()
	}

	fn deserialize(src: &[u8]) -> FortivoResult<Self> {
		todo!()
	}
}

impl Serdeable for Arcanum {
	fn serialize(&self) -> FortivoResult<&[u8]> {
		todo!()
	}

	fn deserialize(src: &[u8]) -> FortivoResult<Self> {
		todo!()
	}
}
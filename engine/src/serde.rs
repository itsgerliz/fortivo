use std::io::Write;
use serde::ser::{self, Impossible};
use crate::error::FortivoError;

pub struct ArcaHeaderSerializer<W: Write> {
	writer: W
}

impl<W: Write> ser::Serializer for &mut ArcaHeaderSerializer<W> {
	type Ok = ();
	type Error = FortivoError;
	
	type SerializeSeq = Impossible<Self::Ok, Self::Error>;
	type SerializeTuple = Impossible<Self::Ok, Self::Error>;
	type SerializeTupleStruct = Impossible<Self::Ok, Self::Error>;
	type SerializeTupleVariant = Impossible<Self::Ok, Self::Error>;
	type SerializeMap = Impossible<Self::Ok, Self::Error>;
	type SerializeStruct = Self;
	type SerializeStructVariant = Impossible<Self::Ok, Self::Error>;

	// Implemented methods
	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		Ok(self.writer.write_all(&[v])?)
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		Ok(self.writer.write_all(&v.to_le_bytes())?)
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		Ok(self.writer.write_all(&v.to_le_bytes())?)
	}

	fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Self::Error> {
		Ok(self)
	}

	// Unimplemented methods
	fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_none(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + ser::Serialize { unimplemented!() }
	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> { unimplemented!() }
	fn serialize_newtype_struct<T>(self, _name: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + ser::Serialize { unimplemented!() }
	fn serialize_newtype_variant<T>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<Self::Ok, Self::Error> where T: ?Sized + ser::Serialize { unimplemented!() }
	fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> { unimplemented!() }
	fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> { unimplemented!() }
	fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> { unimplemented!() }
	fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> { unimplemented!() }
	fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> { unimplemented!() }
	fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> { unimplemented!() }
}

impl<W: Write> ser::SerializeStruct for &mut ArcaHeaderSerializer<W> {
	type Ok = ();
	type Error = FortivoError;

	fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
	where
		T: ?Sized + ser::Serialize {
		value.serialize(&mut **self)
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(())
	}
}
//! The basic serializer

use serde::ser::Serializer;
use std::io::{BufWriter, Write};
use crate::{error::{SerializerError, SerializerResult}, ser::complex::ComplexSerializer};

/// The basic serializer type
/// # Considerations
/// - This type is buffered, read [BasicSerializer::flush]
pub struct BasicSerializer<W: Write> {
    writer: BufWriter<W>,
}

impl<W: Write> BasicSerializer<W> {
    /// Construct a new basic serializer, which will write its output into `W`
    pub fn new(destination: W) -> Self {
        Self {
            writer: BufWriter::new(destination),
        }
    }

    /// The [BasicSerializer] is buffered, this means that while you might have finished
    /// serializing data, this inner buffer could have CBOR data pending to be written
    /// to the output, this method tries to flush this buffer, ensuring all pending
    /// data is written to its output
    /// # Considerations
    /// When the [BasicSerializer] is dropped, flush() will be called automatically by the
    /// [std::ops::Drop] trait, but any errors that might occur during this process
    /// will be ignored, therefore, its highly recommendable to call this method
    pub fn flush(&mut self) -> SerializerResult {
        Ok(self.writer.flush()?)
    }

    fn write_u8(&mut self, data: u8) -> SerializerResult {
        Ok(self.writer.write_all(&[data])?)
    }

    fn write_u16(&mut self, data: u16) -> SerializerResult {
        Ok(self.writer.write_all(&data.to_be_bytes())?)
    }

    fn write_u32(&mut self, data: u32) -> SerializerResult {
        Ok(self.writer.write_all(&data.to_be_bytes())?)
    }

    fn write_u64(&mut self, data: u64) -> SerializerResult {
        Ok(self.writer.write_all(&data.to_be_bytes())?)
    }

    fn write_bytes(&mut self, data: &[u8]) -> SerializerResult {
        Ok(self.writer.write_all(data)?)
    }
}

enum ArgumentPlacement {
    AdditionalInformation,
    NextByte,
    NextTwoBytes,
    NextFourBytes,
    NextEightBytes
}

fn calculate_argument_placement(length: usize) -> Result<ArgumentPlacement, SerializerError> {
    if length < 24 {
        Ok(ArgumentPlacement::AdditionalInformation)
    } else if length < u8::MAX as usize {
        Ok(ArgumentPlacement::NextByte)
    } else if length < u16::MAX as usize {
        Ok(ArgumentPlacement::NextTwoBytes)
    } else if length < u32::MAX as usize {
        Ok(ArgumentPlacement::NextFourBytes)
    } else if length < u64::MAX as usize {
        Ok(ArgumentPlacement::NextEightBytes)
    } else {
        Err(SerializerError::LengthOutOfBounds)
    }
}

impl<'serializer, W: Write> Serializer for &'serializer mut BasicSerializer<W> {
    type Ok = ();
    type Error = SerializerError;

    type SerializeSeq = ComplexSerializer<'serializer, W>;
    type SerializeTuple = ComplexSerializer<'serializer, W>;
    type SerializeTupleStruct = ComplexSerializer<'serializer, W>;
    type SerializeTupleVariant = ComplexSerializer<'serializer, W>;
    type SerializeMap = ComplexSerializer<'serializer, W>;
    type SerializeStruct = ComplexSerializer<'serializer, W>;
    type SerializeStructVariant = ComplexSerializer<'serializer, W>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        if v < 24 {
            // We don't OR with 0x00 (unsigned integer major type) since v can't be above 23
            self.write_u8(v)
        } else {
            // 0x18 = unsigned integer in the next byte
            self.write_u8(0x18)?;
            self.write_u8(v)
        }
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        if v <= u8::MAX as u16 {
            // Forward if v fits in u8
            self.serialize_u8(v as u8)
        } else {
            // 0x19 = unsigned integer in the next two bytes
            self.write_u8(0x19)?;
            self.write_u16(v)
        }
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        if v <= u16::MAX as u32 {
            // Forward if v fits in u16
            self.serialize_u16(v as u16)
        } else {
            // 0x1A = unsigned integer in the next four bytes
            self.write_u8(0x1A)?;
            self.write_u32(v)
        }
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        if v <= u32::MAX as u64 {
            // Forward if v fits in u32
            self.serialize_u32(v as u32)
        } else {
            // 0x1B = unsigned integer in the next eight bytes
            self.write_u8(0x1B)?;
            self.write_u64(v)
        }
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}
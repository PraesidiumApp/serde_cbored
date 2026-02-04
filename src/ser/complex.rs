//! The complex serializer

use std::io::Write;
use serde::ser::{SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant};
use crate::{error::SerializerError, ser::basic::BasicSerializer};

/// The complex serializer type
pub struct ComplexSerializer<'serializer, W: Write> {
    encoder: &'serializer mut BasicSerializer<W>,
    indefinite_length: bool,
}

impl<'serializer, W: Write> SerializeSeq for ComplexSerializer<'serializer, W> {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'serializer, W: Write> SerializeTuple for ComplexSerializer<'serializer, W> {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'serializer, W: Write> SerializeTupleStruct for ComplexSerializer<'serializer, W> {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'serializer, W: Write> SerializeTupleVariant for ComplexSerializer<'serializer, W> {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'serializer, W: Write> SerializeMap for ComplexSerializer<'serializer, W> {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'serializer, W: Write> SerializeStruct for ComplexSerializer<'serializer, W> {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'serializer, W: Write> SerializeStructVariant for ComplexSerializer<'serializer, W> {
    type Ok = ();
    type Error = SerializerError;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

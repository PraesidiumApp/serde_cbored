//! Error types
//!
//! For simplicity and clarity the specific error cases both for serializing and deserializing are directly  
//! declared in its respective enums ([SerializerError] and [DeserializerError]) so the [SerializerError::Serialization]  
//! and [DeserializerError::Deserialization] variants, while needed by [serde::ser::Error] and [serde::de::Error]  
//! trait contracts, are left unused

use serde::{de, ser};
use std::{fmt::Display, io};
use thiserror::Error;

#[cfg(feature = "ser")]
pub(crate) type SerializerResult = Result<(), SerializerError>;

#[cfg(feature = "de")]
pub(crate) type DeserializerResult = Result<(), DeserializerError>;

/// Represents an error while serializing a CBOR data sequence
#[cfg(feature = "ser")]
#[derive(Error, Debug)]
pub enum SerializerError {
    /// Unused variant, needed because of [serde::ser::Error] trait contract (read module docs)
    #[error("Error when serializing")]
    Serialization(String),
    /// Input/Output error while encoding, usually an error when writing to the [Encoder](crate::ser)'s output
    #[error("Input/Output error")]
    IO(#[from] io::Error),
    /// The CBOR RFC which this codec is based on does not allow data items with lengths above
    /// 2^64 bytes, this number is absurdly big so it should not be reached
    #[error("Cannot serialize lengths above 2^64 bytes")]
    LengthOutOfBounds,
}

/// Represents an error while deserializing a CBOR data sequence
#[cfg(feature = "de")]
#[derive(Error, Debug)]
pub enum DeserializerError {
    /// Unused variant, needed because of [serde::de::Error] trait contract (read module docs)
    #[error("Error when deserializing")]
    Deserialization(String),
    /// Input/Output error while decoding, usually an error when reading from [Decoder](crate::de)'s input
    #[error("Input/Output error")]
    IO(#[from] io::Error),
}

#[cfg(feature = "ser")]
impl ser::Error for SerializerError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Serialization(msg.to_string())
    }
}

#[cfg(feature = "de")]
impl de::Error for DeserializerError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Deserialization(msg.to_string())
    }
}

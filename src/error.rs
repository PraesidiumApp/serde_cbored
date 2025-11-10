//! Provides types representing possible errors when encoding/decoding  
//! 
//! For simplicity and clarity the specific error cases both for encoding and decoding are directly  
//! declared in its respective enums ([EncodeError] and [DecodeError]) so the [EncodeError::Serialization]  
//! and [DecodeError::Deserialization] variants, while needed by [serde::ser::Error] and [serde::de::Error]  
//! trait contracts, are left unused

use serde::{de, ser};
use std::{fmt::Display, io};
use thiserror::Error;

/// The main error type, holds either an [EncodeError] or a [DecodeError]
#[derive(Error, Debug)]
pub enum Error {
    /// Error while encoding a CBOR data sequence
    #[error("Error while encoding a CBOR data sequence")]
    Encode(#[from] EncodeError),
    /// Error while decoding a CBOR data sequence
    #[error("Error while decoding a CBOR data sequence")]
    Decode(#[from] DecodeError),
}

/// Represents an error while encoding a CBOR data sequence
#[derive(Error, Debug)]
pub enum EncodeError {
    /// Unused variant, needed because of [serde::ser::Error] trait contract (read module docs)
    #[error("Error when serializing")]
    Serialization(String),
    /// Represents an Input/Output error while encoding, usually an error when writing to the  
    /// [Encoder](crate::serde::ser::Encoder)'s inner writer (the encoding output)
    #[error("Input/Output error")]
    IO(#[from] io::Error),
    /// Byte slices longer than 2^64 cannot be encoded in the present CBOR RFC  
    /// Since this is absurdly big, this error should never be returned, its only here to comply with the RFC
    #[error("Cannot encode byte strings longer than 2^64 bytes")]
    ByteStringTooLong,
    /// Text strings longer than 2^64 (bytes) cannot be encoded in the present CBOR RFC  
    /// Since this is absurdly big, this error should never be returned, its only here to comply with the RFC
    #[error("Cannot encode text strings longer than 2^64 bytes")]
    TextStringTooLong,
}

/// Represents an error while decoding a CBOR data sequence
#[derive(Error, Debug)]
pub enum DecodeError {
    /// Unused variant, needed because of [serde::de::Error] trait contract (read module docs)
    #[error("Error when deserializing")]
    Deserialization(String),
}

impl ser::Error for EncodeError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Serialization(msg.to_string())
    }
}

impl de::Error for DecodeError {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Deserialization(msg.to_string())
    }
}

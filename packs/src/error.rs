use thiserror::Error;
use crate::ll::marker::Marker;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("IO error while reading: {0}")]
    ReadIOError(#[from] std::io::Error),
    #[error("Unexpected marker '{0}'")]
    UnexpectedMarker(Marker),
    #[error("Unknown marker byte '{0}'")]
    UnknownMarkerByte(u8),
    #[error("Cannot read size info as usize")]
    CannotReadSizeInfo,
    #[error("Unexpected tag byte '{0}'")]
    UnexpectedTagByte(u8),
    #[error("Expected {0} fields but got {1}")]
    UnexpectedNumberOfFields(usize, usize),
    #[error("Not allowed to decode NoStruct")]
    TryingToDecodeNoStruct,
}

#[derive(Error, Debug)]
pub enum EncodeError {
    #[error("IO error while writing: {0}")]
    WriteIOError(#[from] std::io::Error),
    #[error("Too many struct fields: {0}")]
    TooManyStructFields(usize)
}


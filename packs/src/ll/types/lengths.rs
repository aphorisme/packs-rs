use std::io::{Read, Write};
use crate::error::{DecodeError, EncodeError};
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
use std::convert::TryFrom;
use crate::ll::marker::{MarkerSizeInfo, Marker};

pub fn read_size_8<T: Read>(reader: &mut T) -> Result<usize, DecodeError> {
    let u = reader.read_u8()?;
    TryFrom::try_from(u).or(Err(DecodeError::CannotReadSizeInfo))
}

pub fn read_size_16<T: Read>(reader: &mut T) -> Result<usize, DecodeError> {
    let u = reader.read_u16::<BigEndian>()?;
    TryFrom::try_from(u).or(Err(DecodeError::CannotReadSizeInfo))
}

pub fn read_size_32<T: Read>(reader: &mut T) -> Result<usize, DecodeError> {
    let u = reader.read_i32::<BigEndian>()?;
    TryFrom::try_from(u).or(Err(DecodeError::CannotReadSizeInfo))
}

pub fn read_size<T: Read>(size_info: MarkerSizeInfo, reader: &mut T) -> Result<usize, DecodeError> {
    match size_info {
        MarkerSizeInfo::Tiny => Ok(0),
        MarkerSizeInfo::Bit8 => read_size_8(reader),
        MarkerSizeInfo::Bit16 => read_size_16(reader),
        MarkerSizeInfo::Bit32 => read_size_32(reader),
        MarkerSizeInfo::None => Ok(0),
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// The possible lengths for sized types like `String8`. The different sign extensions are according
/// to the PackStream specification.
pub enum Length {
    Tiny(u8),
    Bit8(u8),
    Bit16(u16),
    Bit32(i32),
}

impl Length {
    pub fn marker(self, tiny_marker: fn(usize) -> Marker, marker8: Marker, marker16: Marker, marker32: Marker) -> Marker {
        match self {
            Length::Tiny(u) => tiny_marker(u as usize),
            Length::Bit8(_) => marker8,
            Length::Bit16(_) => marker16,
            Length::Bit32(_) => marker32,
        }
    }

    pub fn from_usize(size: usize) -> Option<Length> {
        if size <= 0x0F {
            Some(Length::Tiny(size as u8))
        } else if let Ok(i) = TryFrom::try_from(size) {
            Some(Length::Bit8(i))
        } else if let Ok(i) = TryFrom::try_from(size) {
            Some(Length::Bit16(i))
        } else if let Ok(i) = TryFrom::try_from(size) {
            Some(Length::Bit32(i))
        } else {
            None
        }
    }

    pub fn into_usize(self) -> usize {
        match self {
            Length::Tiny(u) => u as usize,
            Length::Bit8(u) => u as usize,
            Length::Bit16(u) => u as usize,
            Length::Bit32(i) =>
                TryFrom::try_from(i).expect(&format!("Cannot read usize out of {}", i))
        }
    }

    pub fn encode<T: Write>(self, mut writer: T) -> Result<usize, EncodeError> {
        match self {
            Length::Tiny(_) => Ok(0),
            Length::Bit8(size) => {
                writer.write_u8(size)?;
                Ok(1)
            },
            Length::Bit16(size) => {
                writer.write_u16::<BigEndian>(size)?;
                Ok(2)
            },
            Length::Bit32(size) => {
                writer.write_i32::<BigEndian>(size)?;
                Ok(4)
            }
        }
    }
}
use std::io::{Read, Write};
use crate::error::{DecodeError, EncodeError};
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
use std::convert::TryFrom;
use crate::ll::marker::Marker;

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

/// Reads the size of a PackStream `Dictionary` as denoted by the marker. Reports `UnexpectedMarker`
/// on markers which denote no `Dictionary`.
/// ```
/// use packs::ll::marker::Marker;
/// use packs::ll::types::lengths::{read_dict_size, Length};
///
/// let mut buffer = Vec::with_capacity(2);
/// Length::Bit16(42042).encode(&mut buffer).unwrap();
///
/// let size = read_dict_size(Marker::Dictionary16, &mut buffer.as_slice()).unwrap();
///
/// assert_eq!(42042, size);
/// ```
pub fn read_dict_size<T: Read>(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
    match marker {
        Marker::TinyDictionary(u) => Ok(u),
        Marker::Dictionary8 => read_size_8(reader),
        Marker::Dictionary16 => read_size_16(reader),
        Marker::Dictionary32 => read_size_32(reader),
        _ => Err(DecodeError::UnexpectedMarker(marker))
    }
}

/// Reads the size of a PackStream `List` as denoted by the marker. Analogous to
/// [`read_dict_size`](crate::ll::types::lengths::read_dict_size).
pub fn read_list_size<T: Read>(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
    match marker {
        Marker::TinyList(u) => Ok(u),
        Marker::List8 => read_size_8(reader),
        Marker::List16 => read_size_16(reader),
        Marker::List32 => read_size_32(reader),
        _ => Err(DecodeError::UnexpectedMarker(marker))
    }
}

/// Reads the size of a PackStream `String` as denoted by the marker.
pub fn read_string_size<T: Read>(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
    match marker {
        Marker::TinyString(u) => Ok(u),
        Marker::String8 => read_size_8(reader),
        Marker::String16 => read_size_16(reader),
        Marker::String32 => read_size_32(reader),
        _ => Err(DecodeError::UnexpectedMarker(marker))
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
/// The possible lengths for sized types like `String8`. The different sign extensions are according
/// to the PackStream specification. This type should be used for encoding any size information, e.g.
/// the 32bit size information of a `String32` or `Dictionary32` can be encoded using `Length`:
/// ```
/// use packs::ll::types::lengths::Length;
/// let mut buffer = Vec::with_capacity(4);
/// Length::Bit32(420420).encode(&mut buffer);
///
/// assert_eq!(buffer, vec!(0x00, 0x06, 0x6A, 0x44), "Got: {:X?}", buffer);
/// ```
/// It does not shrink the size to any lower space (e.g. using `u8` instead of `i32`).
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
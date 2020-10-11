use std::io::{Write, Read};
use std::io;
use byteorder::{WriteBytesExt, ReadBytesExt};
use crate::ll::bitops::{combine, high_nibble_equals, get_tiny_size};
use std::fmt::{Display, Formatter};
use crate::ll::bounds::{is_in_plus_tiny_int_bound};
use crate::error::DecodeError;

#[derive(Copy, Clone, Debug, PartialEq)]
/// # Overview
/// A `Marker` is the first byte of any encoded value in PackStream. It denotes what type the encoded
/// value is and might carry some size information.
///
/// ## Size as part of the marker
/// Some marker have as their second nibble size information, which ranges from `0x00` to `0x0F`.
/// This information is part of `Marker`. The higher nibble information the just flags the type. It
/// can be checked against using [`MarkerHighNibble`](crate::ll::marker::MarkerHighNibble), which only
/// carries the type information.
/// ```
/// use packs::ll::marker::{Marker, MarkerHighNibble};
/// let m = Marker::from_u8(0x81).unwrap(); // TinyString of length 1.
/// assert_eq!(MarkerHighNibble::TinyString, m.high_nibble());
/// ```
///
/// ## Special types
/// There are two special marker, `PlusTinyInt` and `MinusTinyInt` which carry not only the type
/// information, but the value itself. The extra information is just the read byte, hence any
/// interpretation has to be done on the provided value, see e.g.
/// [`byte_to_minus_tiny_int()`](crate::ll::types::fixed::byte_to_minus_tiny_int).
pub enum Marker {
    // marker as first nibble
    TinyString(usize),
    TinyList(usize),
    TinyDictionary(usize),
    Structure(usize),

    // special:
    PlusTinyInt(u8),
    MinusTinyInt(u8),

    // fixed length
    Float64,

    Int8,
    Int16,
    Int32,
    Int64,

    Null,

    True,
    False,

    // length as section
    String8,
    String16,
    String32,

    Bytes8,
    Bytes16,
    Bytes32,

    List8,
    List16,
    List32,

    Dictionary8,
    Dictionary16,
    Dictionary32,
}

impl Marker {
    pub fn high_nibble(&self) -> MarkerHighNibble {
        match self {
            Marker::Null => MarkerHighNibble::Null,
            Marker::True => MarkerHighNibble::True,
            Marker::False => MarkerHighNibble::False,
            Marker::Float64 => MarkerHighNibble::Float64,
            Marker::PlusTinyInt(_) => MarkerHighNibble::PlusTinyInt,
            Marker::MinusTinyInt(_) => MarkerHighNibble::MinusTinyInt,
            Marker::Int8 => MarkerHighNibble::Int8,
            Marker::Int16 => MarkerHighNibble::Int16,
            Marker::Int32 => MarkerHighNibble::Int32,
            Marker::Int64 => MarkerHighNibble::Int64,
            Marker::TinyString(_) => MarkerHighNibble::TinyString,
            Marker::String8 => MarkerHighNibble::String8,
            Marker::String16 => MarkerHighNibble::String16,
            Marker::String32 => MarkerHighNibble::String32,
            Marker::TinyList(_) => MarkerHighNibble::TinyList,
            Marker::List8 => MarkerHighNibble::List8,
            Marker::List16 => MarkerHighNibble::List16,
            Marker::List32 => MarkerHighNibble::List32,
            Marker::TinyDictionary(_) => MarkerHighNibble::TinyDictionary,
            Marker::Dictionary8 => MarkerHighNibble::Dictionary8,
            Marker::Dictionary16 => MarkerHighNibble::Dictionary16,
            Marker::Dictionary32 => MarkerHighNibble::Dictionary32,
            Marker::Bytes8 => MarkerHighNibble::Bytes8,
            Marker::Bytes16 => MarkerHighNibble::Bytes16,
            Marker::Bytes32 => MarkerHighNibble::Bytes32,
            Marker::Structure(_) => MarkerHighNibble::Structure,
        }
    }

    pub fn encode<T: Write>(self, into: &mut T) -> io::Result<usize> {
        into.write_u8(self.into_u8())?;
        Ok(1)
    }

    pub fn into_u8(self) -> u8 {
        use Marker::*;
        match self {
            TinyString(size) => combine(MarkerHighNibble::TinyString as u8, size as u8),
            TinyList(size) => combine(MarkerHighNibble::TinyList as u8, size as u8),
            TinyDictionary(size) => combine(MarkerHighNibble::TinyDictionary as u8, size as u8),
            Structure(size) => combine(MarkerHighNibble::Structure as u8, size as u8),

            PlusTinyInt(value) => value,
            MinusTinyInt(value) => value,

            p => p.high_nibble() as u8,
        }
    }

    pub fn from_u8(from: u8) -> Option<Marker> {
        if is_in_plus_tiny_int_bound(from as i64) {
            Some(Marker::PlusTinyInt(from))
        } else if MarkerHighNibble::MinusTinyInt.is_contained_in(from) {
            Some(Marker::MinusTinyInt(from))
        } else if MarkerHighNibble::TinyString.is_contained_in(from) {
            Some(Marker::TinyString(get_tiny_size(from)))
        } else if MarkerHighNibble::TinyList.is_contained_in(from) {
            Some(Marker::TinyList(get_tiny_size(from)))
        } else if MarkerHighNibble::TinyDictionary.is_contained_in(from) {
            Some(Marker::TinyDictionary(get_tiny_size(from)))
        } else if MarkerHighNibble::Structure.is_contained_in(from) {
            Some(Marker::Structure(get_tiny_size(from)))
        } else {
            match from {
                0xC0 => Some(Marker::Null),
                0xC1 => Some(Marker::Float64),
                0xC2 => Some(Marker::True),
                0xC3 => Some(Marker::False),

                0xC8 => Some(Marker::Int8),
                0xC9 => Some(Marker::Int16),
                0xCA => Some(Marker::Int32),
                0xCB => Some(Marker::Int64),

                0xCC => Some(Marker::Bytes8),
                0xCD => Some(Marker::Bytes16),
                0xCE => Some(Marker::Bytes32),

                0xD0 => Some(Marker::String8),
                0xD1 => Some(Marker::String16),
                0xD2 => Some(Marker::String32),

                0xD4 => Some(Marker::List8),
                0xD5 => Some(Marker::List16),
                0xD6 => Some(Marker::List32),

                0xD8 => Some(Marker::Dictionary8),
                0xD9 => Some(Marker::Dictionary16),
                0xDA => Some(Marker::Dictionary32),

                _ => None
            }
        }
    }

    pub fn decode<T: Read>(reader: &mut T) -> Result<Marker, DecodeError> {
        let marker_byte = reader.read_u8()?;
        if let Some(marker) = Marker::from_u8(marker_byte) {
            Ok(marker)
        } else {
            Err(DecodeError::UnknownMarkerByte(marker_byte))
        }
    }

    pub fn size_info(self) -> MarkerSizeInfo {
        match self {
            Marker::TinyString(_)
            | Marker::TinyList(_)
            | Marker::TinyDictionary(_)
            | Marker::Structure(_) => MarkerSizeInfo::Tiny,

            Marker::String8
            | Marker::List8
            | Marker::Dictionary8
            | Marker::Bytes8 => MarkerSizeInfo::Bit8,

            Marker::String16
            | Marker::List16
            | Marker::Dictionary16
            | Marker::Bytes16 => MarkerSizeInfo::Bit16,

            Marker::String32
            | Marker::List32
            | Marker::Dictionary32
            | Marker::Bytes32 => MarkerSizeInfo::Bit32,

            _ => MarkerSizeInfo::None,
        }
    }
}

impl Display for Marker {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MarkerSizeInfo {
    Tiny,
    Bit8,
    Bit16,
    Bit32,
    None,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MarkerHighNibble {
    // marker as first nibble
    TinyString = 0x80,
    TinyList = 0x90,
    TinyDictionary = 0xA0,
    Structure = 0xB0,

    // special:
    PlusTinyInt = 0x00,
    MinusTinyInt = 0xF0,

    // fixed length
    Float64 = 0xC1,

    Int8 = 0xC8,
    Int16 = 0xC9,
    Int32 = 0xCA,
    Int64 = 0xCB,

    Null = 0xC0,

    True = 0xC2,
    False = 0xC3,

    // length as section
    String8 = 0xD0,
    String16 = 0xD1,
    String32 = 0xD2,

    Bytes8 = 0xCC,
    Bytes16 = 0xCD,
    Bytes32 = 0xCE,

    List8 = 0xD4,
    List16 = 0xD5,
    List32 = 0xD6,

    Dictionary8 = 0xD8,
    Dictionary16 = 0xD9,
    Dictionary32 = 0xDA,
}

impl MarkerHighNibble {
    pub fn is_contained_in(self, marker_byte: u8) -> bool {
        high_nibble_equals(self as u8, marker_byte & 0xF0)
    }
}

#[cfg(test)]
pub mod test {
    use crate::ll::marker::Marker;

    #[test]
    fn from_u8_plus_tiny_int_marker() {
        assert_eq!(Some(Marker::PlusTinyInt(0x0F)), Marker::from_u8(0x0F));
    }

    #[test]
    fn from_u8_minus_tiny_int_marker() {
        for i in 0u8..0x10 {
            assert_eq!(Some(Marker::MinusTinyInt(0xF0 | i)), Marker::from_u8(0xF0 | i));
        }
    }

    #[test]
    fn from_u8_tiny_sized_marker() {
        let r: Vec<fn(usize) -> Marker> = vec! {
            Marker::TinyString,
            Marker::TinyList,
            Marker::TinyDictionary,
            Marker::Structure,
        };

        for m in r {
            for i in 0u8..0x10 {
                assert_eq!(Some(m(i as usize)), Marker::from_u8(m(0).high_nibble() as u8 | i));
            }
        }

    }

    #[test]
    fn from_u8_inverses() {
        let r = vec! {
            Marker::Null,
            Marker::Float64,
            Marker::False,
            Marker::True,
            Marker::MinusTinyInt(0xF0),
            Marker::PlusTinyInt(0x00),
            Marker::Int8,
            Marker::Int16,
            Marker::Int32,
            Marker::Int64,
            Marker::TinyString(0),
            Marker::String8,
            Marker::String16,
            Marker::String32,
            Marker::TinyList(0),
            Marker::List8,
            Marker::List16,
            Marker::List32,
            Marker::TinyDictionary(0),
            Marker::Dictionary8,
            Marker::Dictionary16,
            Marker::Dictionary32,
            Marker::Bytes8,
            Marker::Bytes16,
            Marker::Bytes32,
            Marker::Structure(0),
        };

        for m in r {
            assert_eq!(Some(m), Marker::from_u8(m.high_nibble() as u8));
        }
    }
}
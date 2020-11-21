//! # Overview
//! The main API traits are [`Pack`](crate::packable::Pack) and [`Unpack`](crate::packable::Unpack).
//! Both are high-level traits for types which can be encoded and decoded
//! following the PackStream specification. This includes the basic types as well as
//! compound structures.
//!
//! ## Encoding and Decoding Strategy
//! The `encode` function for all basic types tries to be as space efficient as the specification
//! allows. E.g. an `0: i32` will get encoded just with `0x00` instead of some full `Int32`. The
//! `decode` function though reads any correct encoded value into its target type. This means,
//! that `decode : Bytes -> Value` is not injective; different byte sequences can be decoded into
//! the same value:
//! ```
//! use packs::{Pack, Unpack};
//! let mut bytes_i16 : &[u8] = &[0xC9, 0xFF, 0xFF]; // -1 as Int16
//! let mut bytes_i8 : &[u8] = &[0xC8, 0xFF]; // -1 as Int8
//!
//! let decoded_i16 = i64::decode(&mut bytes_i16).unwrap();
//! let decoded_i8 = i64::decode(&mut bytes_i8).unwrap();
//!
//! assert_eq!(decoded_i8, decoded_i16);
//! ```
//! This especially means that `encode` and `decode` do **not** need to be inverses:
//! ```
//! use packs::{Pack, Unpack};
//! let mut bytes : &[u8] = &[0xC9, 0x00, 0x01]; // 1 as Int16
//! let decoded = i64::decode(&mut bytes).unwrap();
//!
//! // but will be encoded just as `TINY_PLUS_INT` here: `0x01`.
//! let mut encoded_bytes = Vec::new();
//! decoded.encode(&mut encoded_bytes).unwrap();
//!
//! assert_eq!(encoded_bytes.as_slice(), &[0x01]);
//! ```
//! If the base value can only be encoded in a unique manner, then `encode` and `decode`
//! are inverses:
//! ```
//! use packs::{Pack, Unpack};
//! let mut buffer = Vec::new();
//! let value: i64 = 42334388282948;
//!
//! value.encode(&mut buffer).unwrap();
//! let res = i64::decode(&mut buffer.as_slice()).unwrap();
//!
//! assert_eq!(value, res);
//! ```
//! as well as in the other direction:
//! ```
//! use packs::{Pack, Unpack};
//! use std::io::Cursor;
//! let buffer: &[u8] = &[0xC9, 0x7F, 0x0C];
//! let mut cursor = Cursor::new(buffer);
//! let res = i64::decode(&mut cursor).unwrap();
//! let mut res_buffer = Vec::new();
//! res.encode(&mut res_buffer).unwrap();
//!
//! cursor.set_position(0);
//! assert_eq!(cursor.into_inner(), res_buffer.as_slice());
//! ```
//!
//! ## Implementation for user-defined types
//! An implementation for new base types is **not** foreseen. Base types are built into the library
//! itself and should be rather (pull) requested.
//!
//! An implementation for complex types (i.e. structures) on the other hand, is possible and intended.
//! Structures are packed with an extra tag byte to denote which structure is packed.


use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{Read, Write};

use crate::error::{DecodeError, EncodeError};
use crate::ll::bounds::{is_in_i16_bound, is_in_i32_bound, is_in_i8_bound, is_in_minus_tiny_int_bound, is_in_plus_tiny_int_bound};
use crate::ll::marker::Marker;
use crate::ll::types::fixed::{byte_to_minus_tiny_int, encode_i16, encode_i32, encode_i64, encode_i8, encode_minus_tiny_int, encode_plus_tiny_int, decode_body_i8, decode_body_i16, decode_body_i32, decode_body_i64, decode_body_f64, encode_f64};
use crate::ll::types::lengths::{Length, read_size_16, read_size_32, read_size_8, read_string_size, read_list_size, read_dict_size};
use crate::ll::types::sized::{write_body_by_iter};
use crate::value::Value;
use crate::value::bytes::Bytes;
use crate::value::dictionary::Dictionary;

/// Trait to encode values into any writer using PackStream; using a space efficient way
/// to pack.
pub trait Pack: Sized {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError>;
}

/// Trait to decode values from a stream using PackStream.
pub trait Unpack: Sized {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError>;
    fn decode<T: Read>(reader: &mut T) -> Result<Self, DecodeError> {
        let marker = Marker::decode(reader)?;
        Self::decode_body(marker, reader)
    }
}

impl Unpack for i64 {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        match marker {
            Marker::PlusTinyInt(value) => Ok(value as i64),
            Marker::MinusTinyInt(value) => {
                Ok(byte_to_minus_tiny_int(value) as i64)
            }
            Marker::Int8 => Ok(decode_body_i8(reader)? as i64),
            Marker::Int16 => Ok(decode_body_i16(reader)? as i64),
            Marker::Int32 => Ok(decode_body_i32(reader)? as i64),
            Marker::Int64 => Ok(decode_body_i64(reader)?),

            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

impl Pack for i64 {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        if is_in_plus_tiny_int_bound(*self) {
            Ok(encode_plus_tiny_int(*self as u8, writer)?)
        } else if is_in_minus_tiny_int_bound(*self) {
            Ok(encode_minus_tiny_int(*self as i8, writer)?)
        } else if is_in_i8_bound(*self) {
            Ok(encode_i8(*self as i8, writer)?)
        } else if is_in_i16_bound(*self) {
            Ok(encode_i16(*self as i16, writer)?)
        } else if is_in_i32_bound(*self) {
            Ok(encode_i32(*self as i32, writer)?)
        } else {
            Ok(encode_i64(*self, writer)?)
        }
    }
}

impl Unpack for i32 {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        match marker {
            Marker::PlusTinyInt(value) => Ok(value as i32),
            Marker::MinusTinyInt(value) => {
                Ok(byte_to_minus_tiny_int(value) as i32)
            }
            Marker::Int8 => Ok(decode_body_i8(reader)? as i32),
            Marker::Int16 => Ok(decode_body_i16(reader)? as i32),
            Marker::Int32 => Ok(decode_body_i32(reader)?),

            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

impl Pack for i32 {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        if is_in_plus_tiny_int_bound(*self as i64) {
            Ok(encode_plus_tiny_int(*self as u8, writer)?)
        } else if is_in_minus_tiny_int_bound(*self as i64) {
            Ok(encode_minus_tiny_int(*self as i8, writer)?)
        } else if is_in_i8_bound(*self as i64) {
            Ok(encode_i8(*self as i8, writer)?)
        } else if is_in_i16_bound(*self as i64) {
            Ok(encode_i16(*self as i16, writer)?)
        } else {
            Ok(encode_i32(*self, writer)?)
        }
    }
}

impl Unpack for String {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        let len = read_string_size(marker, reader)?;
        let mut result = String::new();
        reader.take(len as u64).read_to_string(&mut result)?;
        Ok(result)
    }
}

impl Pack for String {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let len = Length::from_usize(self.len()).expect("String has invalid length");
        let mut written =
            match len {
                Length::Tiny(t) => Marker::TinyString(t as usize).encode(writer)?,
                Length::Bit8(_) => Marker::String8.encode(writer)?,
                Length::Bit16(_) => Marker::String16.encode(writer)?,
                Length::Bit32(_) => Marker::String32.encode(writer)?,
            };
        written += len.encode(writer)?;
        written += writer.write(self.as_bytes())?;

        Ok(written)
    }
}

impl<P: Pack> Pack for Vec<P> {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let len = Length::from_usize(self.len()).expect("Vec has invalid size");
        let mut written = len.encode_as_list_size(writer)?;
        written += write_body_by_iter(&mut self.iter(), writer)?;
        Ok(written)
    }
}

impl<P: Unpack> Unpack for Vec<P> {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        let len = read_list_size(marker, reader)?;
        let mut result = Vec::with_capacity(len);
        for _ in 0..len {
            let p = P::decode(reader)?;
            result.push(p);
        }

        Ok(result)
    }
}


impl<P: Unpack> Unpack for HashMap<String, P> {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        let len = read_dict_size(marker, reader)?;
        let mut result = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = String::decode(reader)?;
            let val = P::decode(reader)?;
            result.insert(key, val);
        }

        Ok(result)
    }
}

impl<P: Pack> Pack for HashMap<String, P> {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let len = Length::from_usize(self.len()).expect("HashMap has invalid length");
        let mut written = len.encode_as_dict_size(writer)?;

        for (key, val) in self {
            written +=
                key.encode(writer)?
                    + val.encode(writer)?;
        }

        Ok(written)
    }
}

impl<P: Unpack> Unpack for Dictionary<P> {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        let inner =
            <HashMap<String, Value<P>>>::decode_body(marker, reader)?;
        Ok(Dictionary::from_inner(inner))
    }
}

impl<P: Pack> Pack for Dictionary<P> {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        self.inner().encode(writer)
    }
}

impl<P: Unpack + Hash + Eq> Unpack for HashSet<P> {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        let len = read_list_size(marker, reader)?;
        let mut result = HashSet::with_capacity(len);
        for _ in 0..len {
            let p = P::decode(reader)?;
            result.insert(p);
        }

        Ok(result)
    }
}

impl<P: Pack> Pack for HashSet<P> {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let len = Length::from_usize(self.len()).expect("HashSet has invalid length");
        let mut written = len.encode_as_list_size(writer)?;
        written += write_body_by_iter(&mut self.iter(), writer)?;

        Ok(written)
    }
}

impl Unpack for Bytes {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        let len = match marker {
            Marker::Bytes8 => read_size_8(reader)?,
            Marker::Bytes16 => read_size_16(reader)?,
            Marker::Bytes32 => read_size_32(reader)?,
            _ => Err(DecodeError::UnexpectedMarker(marker))?,
        };
        let mut res = vec![0; len];
        reader.read_exact(&mut res)?;
        Ok(Bytes(res))
    }
}

impl Pack for Bytes {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let len = Length::from_usize(self.0.len()).expect("Bytes has invalid size");
        let mut written = match len {
            Length::Tiny(u) =>
                Marker::Bytes8.encode(writer)? + Length::Bit8(u).encode(writer)?,
            Length::Bit8(_) =>
                Marker::Bytes8.encode(writer)? + len.encode(writer)?,
            Length::Bit16(_) =>
                Marker::Bytes16.encode(writer)? + len.encode(writer)?,
            Length::Bit32(_) =>
                Marker::Bytes32.encode(writer)? + len.encode(writer)?,
        };

        written += writer.write(self.0.as_slice())?;
        Ok(written)
    }
}

impl Unpack for f64 {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        if marker == Marker::Float64 {
            Ok(decode_body_f64(reader)?)
        } else {
            Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

impl Pack for f64 {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Ok(encode_f64(*self, writer)?)
    }
}

impl Pack for f32 {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        (*self as f64).encode(writer)
    }
}

impl Unpack for bool {
    fn decode_body<T: Read>(marker: Marker, _: &mut T) -> Result<Self, DecodeError> {
        match marker {
            Marker::True => Ok(true),
            Marker::False => Ok(false),
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

impl Pack for bool {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        if *self {
            Marker::True.encode(writer)?;
            Ok(1)
        } else {
            Marker::False.encode(writer)?;
            Ok(1)
        }
    }
}

impl<P: Pack> Pack for Option<P> {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        if let Some(p) = self {
            P::encode(p, writer)
        } else {
            Marker::Null.encode(writer)?;
            Ok(1)
        }
    }
}

impl<P: Unpack> Unpack for Option<P> {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        match marker {
            Marker::Null => Ok(None),
            _ => {
                P::decode_body(marker, reader).map(Some)
            }
        }
    }
}

impl<S: Unpack> Unpack for Value<S> {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        match marker {
            Marker::Null => Ok(Value::Null),
            Marker::True => Ok(Value::Boolean(true)),
            Marker::False => Ok(Value::Boolean(false)),

            Marker::Float64 => Ok(Value::Float(f64::decode_body(marker, reader)?)),

            Marker::PlusTinyInt(_) |
            Marker::MinusTinyInt(_) |
            Marker::Int8 |
            Marker::Int16 |
            Marker::Int32 |
            Marker::Int64 => Ok(Value::Integer(i64::decode_body(marker, reader)?)),

            Marker::TinyString(_) |
            Marker::String8 |
            Marker::String16 |
            Marker::String32 => Ok(Value::String(String::decode_body(marker, reader)?)),

            Marker::TinyList(_) |
            Marker::List8 |
            Marker::List16 |
            Marker::List32 => Ok(Value::List(Vec::decode_body(marker, reader)?)),

            Marker::TinyDictionary(_) |
            Marker::Dictionary8 |
            Marker::Dictionary16 |
            Marker::Dictionary32 => Ok(Value::Dictionary(Dictionary::decode_body(marker, reader)?)),

            Marker::Bytes8 |
            Marker::Bytes16 |
            Marker::Bytes32 => Ok(Value::Bytes(Bytes::decode_body(marker, reader)?)),

            Marker::Structure(_, _) => {
                Ok(Value::Structure(S::decode_body(marker, reader)?))
            }
        }
    }
}

impl<S: Pack> Pack for Value<S> {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        match self {
            Value::Null => Ok(Marker::Null.encode(writer)?),
            Value::Boolean(b) => bool::encode(b, writer),
            Value::Integer(i) => i64::encode(i, writer),
            Value::Float(f) => f64::encode(f, writer),
            Value::String(s) => String::encode(s, writer),
            Value::Bytes(bs) => Bytes::encode(bs, writer),
            Value::Dictionary(d) => <Dictionary<S>>::encode(d, writer),
            Value::List(l) => <Vec<Value<S>>>::encode(l, writer),
            Value::Structure(s) => {
                s.encode(writer)
            }
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::io::Cursor;

    use crate::ll::marker::MarkerHighNibble;
    use crate::packable::{Pack, Unpack};
    use crate::structure::NoStruct;
    use crate::value::Value;

    pub fn unpack_pack_test<T: Unpack + Pack>(mut buffer: &[u8]) {
        let compare = Vec::from(buffer);
        let res = T::decode(&mut buffer).unwrap();

        let mut res_buffer : Vec<u8> = Vec::new();
        res.encode(&mut res_buffer).unwrap();

        assert_eq!(compare,
                   res_buffer,
                   "expected '{:X?}' but got '{:X?}'",
                   compare,
                   res_buffer);
    }

    pub fn pack_unpack_test<T: Pack + Unpack + PartialEq + Debug>(values: &[T]) {
        for value in values {
            let mut buffer: Vec<u8> = Vec::new();
            value
                .encode(&mut buffer)
                .expect(&format!("cannot encode '{:?}'", value));


            let mut cursor = Cursor::new(buffer.clone());
            let res =
                T::decode(&mut cursor)
                    .expect(&format!("cannot decode back to '{:?}'", value));
            assert_eq!(value,
                       &res,
                       "'{:?}' got packed->unpacked into '{:?}'",
                       value, res);
        }
    }

    pub fn pack_to_test<T: Pack + Debug>(value: T, bytes: &[u8]) {
        let mut encoded : Vec<u8> = Vec::new();
        let used_bits = value.encode(&mut encoded).unwrap();

        assert_eq!(used_bits,
                   bytes.len(),
                   "{} written bits, but expected {} bits.",
                   used_bits, bytes.len());
        assert_eq!(encoded,
                   bytes,
                   "value '{:?}' encoded into '{:X?}' but expected '{:X?}'",
                   value, encoded, bytes);
    }

    pub fn unpack_to_test<T: Unpack + Debug + PartialEq>(bytes: &[u8], value: T) {
        assert!(bytes.len() > 0, "Input bytes cannot be empty.");

        let mut cursor : Cursor<&[u8]> = Cursor::new(bytes);
        let res = T::decode(&mut cursor).unwrap();

        assert_eq!(cursor.position(),
                   bytes.len() as u64,
                   "need to read all {} bytes, but read only {}",
                   bytes.len(), cursor.position());

        assert_eq!(res, value);
    }

    #[test]
    fn unpack_pack_m1_tiny_int() {
        unpack_pack_test::<i64>(&[0xFF]);
        unpack_pack_test::<i32>(&[0xFF]);
    }

    #[test]
    fn unpack_to_m1_minus_tiny_int() {
        unpack_to_test(&[0xFF], -1i64);
        unpack_to_test(&[0xFF], -1i32);
    }

    #[test]
    fn unpack_to_m1_int8() {
        unpack_to_test(&[0xC8, 0xFF], -1i32);
        unpack_to_test(&[0xC8, 0xFF], -1i64);
    }

    #[test]
    fn unpack_to_m1_int16() {
        unpack_to_test(&[0xC9, 0xFF, 0xFF], -1i64);
        unpack_to_test(&[0xC9, 0xFF, 0xFF], -1i32);
    }

    #[test]
    fn unpack_to_m1_int32() {
        unpack_to_test(&[0xCA, 0xFF, 0xFF, 0xFF, 0xFF], -1i32);
        unpack_to_test(&[0xCA, 0xFF, 0xFF, 0xFF, 0xFF], -1i64);
    }

    #[test]
    fn unpack_to_hello_string() {
        unpack_to_test(&[0x85, 0x68, 0x65, 0x6C, 0x6C, 0x6F], String::from("hello"));
        unpack_to_test(&[0xD0, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F], String::from("hello"));
        unpack_to_test(&[0xD1, 0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F], String::from("hello"));
        unpack_to_test(&[0xD2, 0x00, 0x00, 0x00, 0x05, 0x68, 0x65, 0x6C, 0x6C, 0x6F], String::from("hello"));
    }

    #[test]
    fn space_shrinking_packing_numbers() {
        pack_to_test(-1, &[0xFF]);
        pack_to_test(-1, &[0xFF]);
        pack_to_test(-17, &[0xC8, 0xEF]);
        pack_to_test(-17, &[0xC8, 0xEF]);
        pack_to_test(128, &[0xC9, 0x00, 0x80]);
        pack_to_test(128, &[0xC9, 0x00, 0x80]);
    }

    #[test]
    fn space_shrinking_packing_strings() {
        pack_to_test(String::from("hello"), &[0x85, 0x68, 0x65, 0x6C, 0x6C, 0x6F]);
        pack_to_test(String::from(""), &[0x80]);
    }

    #[test]
    fn pack_unpack_numbers() {
        pack_unpack_test(&[0, 1, -1, 127, 443928, 49448443, -2700392]);
        pack_unpack_test(&[0, 1, -1, 127, 443928, 49448443, -2700392]);
    }

    #[test]
    fn pack_unpack_strings() {
        let strings: Vec<String> =
            vec!("hello world",
                 "JErl .aA_E Ae1-233k 12ä##",
                 "",
                 "ß++°",
                 "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.")
                .into_iter()
                .map(|s| String::from(s))
                .collect();

        pack_unpack_test(&strings);
    }

    #[test]
    fn pack_unpack_f64() {
        pack_unpack_test(&[0.3, 0.42, -1.0, 0.33333, -455402.1]);
    }

    #[test]
    fn pack_to_bool() {
        pack_to_test(true, &[MarkerHighNibble::True as u8]);
        pack_to_test(false, &[MarkerHighNibble::False as u8]);
    }

    #[test]
    fn unpack_to_bool() {
        unpack_to_test(&[MarkerHighNibble::True as u8], true);
        unpack_to_test(&[MarkerHighNibble::False as u8], false);
    }

    #[test]
    fn pack_unpack_bool() {
        pack_unpack_test(&[true, false]);
    }

    #[test]
    fn pack_unpack_vec_int() {
        pack_unpack_test(
            &[
                vec!(1, 42, 0, 0),
                vec!(),
                vec!(3942379123i64, -1, 0, 813819289, -16, -17)
            ]
        );
    }

    #[test]
    fn pack_unpack_vec_bytes() {
        pack_unpack_test(
            &[
                vec!(
                    vec!(0x08, 0x7F),
                ),
                vec!(),
                vec!(
                    vec!(),
                    vec!(0xFF),
                    vec!(),
                    vec!(0x00)
                )
            ]
        )
    }

    #[test]
    fn pack_unpack_bytes() {
        pack_unpack_test(
            &[
                vec!(0x00, 0x01, 0x03, 0xFF),
                vec!(),
            ]);
    }

    #[test]
    fn pack_unpack_vec_option_int() {
        pack_unpack_test(
            &[
                vec!(0, 0, -1),
                vec!(42i64),
                vec!(),
                vec!(2371237164781i64, -3)
            ]
        )
    }

    #[test]
    fn pack_unpack_hashmap_int() {
        pack_unpack_test(
            &[
                [
                    (String::from("hello"), 42),
                    (String::from("foo"), -1),
                    (String::from("ßß$"), 0),
                ].iter().cloned().collect(),
                HashMap::new(),
            ]
        );
    }

    #[test]
    fn pack_unpack_values() {
        /*pack_unpack_test::<Value<()>>(
            &[
                42.into(), "hello".into()
            ]
        );*/
        let value: Value<NoStruct> = Value::Boolean(true);

        let mut buffer: Vec<u8> = Vec::new();
        value.encode(&mut buffer).unwrap();
    }
}
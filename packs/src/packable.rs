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
//! use packs::packable::{Pack, Unpack};
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
//! use packs::packable::{Pack, Unpack};
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
//! use packs::packable::{Pack, Unpack};
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
//! use packs::packable::{Pack, Unpack};
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


use std::io::{Read, Write};
use crate::ll::types::fixed::{encode_i32, encode_plus_tiny_int, encode_minus_tiny_int, encode_i8, encode_i16, encode_i64, byte_to_minus_tiny_int};
use crate::ll::bounds::{is_in_plus_tiny_int_bound, is_in_minus_tiny_int_bound, is_in_i8_bound, is_in_i16_bound, is_in_i32_bound};
use crate::ll::marker::Marker;
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
use crate::ll::types::sized::{decode_sized, encode_sized, SizedTypeUnpack};
use crate::error::{DecodeError, EncodeError};
use std::collections::{HashMap, HashSet};
use crate::value::{Value};
use crate::structure::struct_sum::{PackableStructSum};
use crate::ll::types::lengths::{read_size_8, read_size_16, read_size_32};
use crate::value::bytes::Bytes;
use std::hash::Hash;

/// Trait to encode values into any writer using PackStream; using a space efficient way
/// to pack.
pub trait Pack<T: Write> : Sized {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError>;
}

/// Trait to decode values from a stream using PackStream.
pub trait Unpack<T: Read> : Sized {
    fn decode(reader: &mut T) -> Result<Self, DecodeError>;
}

impl<T: Read> Unpack<T> for i64 {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        let marker = Marker::decode(reader)?;
        match marker {
            Marker::PlusTinyInt(value) => Ok(value as i64),
            Marker::MinusTinyInt(value) => {
                Ok(byte_to_minus_tiny_int(value) as i64)
            },
            Marker::Int8 => Ok(reader.read_i8()? as i64),
            Marker::Int16 => Ok(reader.read_i16::<BigEndian>()? as i64),
            Marker::Int32 => Ok(reader.read_i32::<BigEndian>()? as i64),
            Marker::Int64 => Ok(reader.read_i64::<BigEndian>()?),

            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

impl<T: Write> Pack<T> for i64 {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
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

impl<T: Read> Unpack<T> for i32 {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        let marker = Marker::decode(reader)?;
        match marker {
            Marker::PlusTinyInt(value) => Ok(value as i32),
            Marker::MinusTinyInt(value) => {
                Ok(byte_to_minus_tiny_int(value) as i32)
            },
            Marker::Int8 => Ok(reader.read_i8()? as i32),
            Marker::Int16 => Ok(reader.read_i16::<BigEndian>()? as i32),
            Marker::Int32 => Ok(reader.read_i32::<BigEndian>()?),

            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

impl<T: Write> Pack<T> for i32 {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
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

impl<T: Read> Unpack<T> for String {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        decode_sized(reader)
    }
}

impl <T: Write> Pack<T> for String {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        encode_sized(self, writer)
    }
}

impl<T: Write, P: Pack<T>> Pack<T> for Vec<P> {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        encode_sized(self, writer)
    }
}

impl<T: Read, P: Unpack<T>> Unpack<T> for Vec<P> {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        decode_sized(reader)
    }
}


impl<T: Read, P: Unpack<T>> Unpack<T> for HashMap<String, P> {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        decode_sized(reader)
    }
}

impl<T: Write, P: Pack<T>> Pack<T> for HashMap<String, P> {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        encode_sized(self, writer)
    }
}

impl<T: Read, P: Unpack<T> + Hash + Eq> Unpack<T> for HashSet<P> {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> { decode_sized(reader) }
}

impl<T: Write, P: Pack<T>> Pack<T> for HashSet<P> {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> { encode_sized(self, writer) }
}

impl<T: Read> Unpack<T> for Bytes {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        decode_sized(reader)
    }
}

impl<T: Write> Pack<T> for Bytes {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        encode_sized(self, writer)
    }
}

impl<T: Read> Unpack<T> for f64 {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        let marker = Marker::decode(reader)?;
        if marker == Marker::Float64 {
            Ok(reader.read_f64::<BigEndian>()?)
        } else {
            Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

impl<T: Write> Pack<T> for f64 {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Marker::Float64.encode(writer)?;
        writer.write_f64::<BigEndian>(*self)?;
        Ok(9)
    }
}

impl<T: Write> Pack<T> for f32 {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Marker::Float64.encode(writer)?;
        writer.write_f64::<BigEndian>(*self as f64)?;
        Ok(5)
    }
}

impl<T: Read> Unpack<T> for bool {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        let marker = Marker::decode(reader)?;
        match marker {
            Marker::True => Ok(true),
            Marker::False => Ok(false),
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

impl<T: Write> Pack<T> for bool {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        if *self {
            Marker::True.encode(writer)?;
            Ok(1)
        } else {
            Marker::False.encode(writer)?;
            Ok(1)
        }
    }
}

impl<T: Write, P: Pack<T>> Pack<T> for Option<P> {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        if let Some(p) = self {
            P::encode(p, writer)
        } else {
            Marker::Null.encode(writer)?;
            Ok(1)
        }
    }
}

impl<T: Read, S: PackableStructSum> Unpack<T> for Value<S> {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        let marker = Marker::decode(reader)?;
        match marker {
            Marker::Null => Ok(Value::Null),
            Marker::True => Ok(Value::Boolean(true)),
            Marker::False => Ok(Value::Boolean(false)),

            Marker::Float64 => Ok(reader.read_f64::<BigEndian>().map(Value::Float)?),

            Marker::PlusTinyInt(value) => Ok(Value::Integer(value as i64)),
            Marker::MinusTinyInt(value) => Ok(Value::Integer(byte_to_minus_tiny_int(value) as i64)),
            Marker::Int8 => Ok(reader.read_i8()? as i64).map(Value::Integer),
            Marker::Int16 => Ok(reader.read_i16::<BigEndian>()? as i64).map(Value::Integer),
            Marker::Int32 => Ok(reader.read_i32::<BigEndian>()? as i64).map(Value::Integer),
            Marker::Int64 => Ok(reader.read_i64::<BigEndian>()?).map(Value::Integer),

            Marker::TinyString(size) => String::read_body(size, reader).map(Value::String),
            Marker::String8 => read_size_8(reader).and_then(|i| String::read_body(i, reader)).map(Value::String),
            Marker::String16 => read_size_16(reader).and_then(|i| String::read_body(i as usize, reader)).map(Value::String),
            Marker::String32 => read_size_32(reader).and_then(|i| String::read_body(i as usize, reader)).map(Value::String),

            Marker::TinyList(size) => <Vec<Value<S>>>::read_body(size, reader).map(Value::List),
            Marker::List8 => read_size_8(reader).and_then(|i| <Vec<Value<S>>>::read_body(i, reader)).map(Value::List),
            Marker::List16 => read_size_16(reader).and_then(|i| <Vec<Value<S>>>::read_body(i as usize, reader)).map(Value::List),
            Marker::List32 => read_size_32(reader).and_then(|i| <Vec<Value<S>>>::read_body(i as usize, reader)).map(Value::List),

            Marker::TinyDictionary(size) => <HashMap<String, Value<S>>>::read_body(size, reader).map(Value::Dictionary),
            Marker::Dictionary8 => read_size_8(reader).and_then(|i| <HashMap<String, Value<S>>>::read_body(i, reader)).map(Value::Dictionary),
            Marker::Dictionary16 => read_size_16(reader).and_then(|i| <HashMap<String, Value<S>>>::read_body(i as usize, reader)).map(Value::Dictionary),
            Marker::Dictionary32 => read_size_32(reader).and_then(|i| <HashMap<String, Value<S>>>::read_body(i as usize, reader)).map(Value::Dictionary),

            Marker::Bytes8 => read_size_8(reader).and_then(|i| <Bytes>::read_body(i, reader)).map(Value::Bytes),
            Marker::Bytes16 => read_size_16(reader).and_then(|i| <Bytes>::read_body(i as usize, reader)).map(Value::Bytes),
            Marker::Bytes32 => read_size_32(reader).and_then(|i| <Bytes>::read_body(i as usize, reader)).map(Value::Bytes),

            Marker::Structure(u) => {
                let tag_byte = reader.read_u8()?;
                S::read_struct_body(u, tag_byte, reader).map(Value::Structure)
            }
        }
    }
}

impl<T: Write, S: PackableStructSum> Pack<T> for Value<S> {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        match self {
            Value::Null => Ok(Marker::Null.encode(writer)?),
            Value::Boolean(b) => bool::encode(b, writer),
            Value::Integer(i) => i64::encode(i, writer),
            Value::Float(f) => f64::encode(f, writer),
            Value::String(s) => String::encode(s, writer),
            Value::Bytes(bs) => Bytes::encode(bs, writer),
            Value::Dictionary(d) => <HashMap<String, Value<S>>>::encode(d, writer),
            Value::List(l) => <Vec<Value<S>>>::encode(l, writer),
            Value::Structure(s) => {
                Marker::Structure(s.fields_len()).encode(writer)?;
                writer.write_u8(s.tag_byte())?;
                Ok(2 + s.write_struct_body(writer)?)
            },
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packable::{Pack, Unpack};
    use std::fmt::Debug;
    use std::io::{Cursor};
    use crate::ll::marker::{MarkerHighNibble};
    use std::collections::HashMap;
    use crate::value::{Value};

    pub fn unpack_pack_test<'a, T: Unpack<&'a [u8]> + Pack<Vec<u8>>>(mut buffer: &'a [u8]) {
        let compare = Vec::from(buffer);
        let res = T::decode(&mut buffer).unwrap();

        let mut res_buffer = Vec::new();
        res.encode(&mut res_buffer).unwrap();

        assert_eq!(compare,
                   res_buffer,
                   "expected '{:X?}' but got '{:X?}'",
                   compare,
                   res_buffer);
    }

    pub fn pack_unpack_test<T: Pack<Vec<u8>> + Unpack<Cursor<Vec<u8>>> + PartialEq + Debug>(values: &[T]) {
        for value in values {
            let mut buffer: Vec<u8> = Vec::new();
            value
                .encode(&mut buffer)
                .expect(&format!("cannot encode '{:?}'", value));


            let mut cursor = Cursor::new(buffer);
            let res =
                T::decode(&mut cursor)
                    .expect(&format!("cannot decode back to '{:?}'", value));
            assert_eq!(value,
                       &res,
                       "'{:?}' got packed->unpacked into '{:?}'",
                       value, res);
        }
    }

    pub fn pack_to_test<T: Pack<Vec<u8>> + Debug>(value: T, bytes: &[u8]) {
        let mut encoded = Vec::new();
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

    pub fn unpack_to_test<'a, T: Unpack<Cursor<&'a [u8]>> + Debug + PartialEq>(bytes: &'a [u8], value: T) {
        assert!(bytes.len() > 0, "Input bytes cannot be empty.");

        let mut cursor = Cursor::new(bytes);
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
        let strs : Vec<String> =
            vec!("hello world",
                 "JErl .aA_E Ae1-233k 12ä##",
                 "",
                 "ß++°",
            "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.")
                .into_iter()
                .map(|s| String::from(s))
                .collect();

        pack_unpack_test(&strs);
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
        let value : Value<()> = Value::Boolean(true);

        let mut buffer: Vec<u8> = Vec::new();
        value.encode(&mut buffer).unwrap();
    }
}
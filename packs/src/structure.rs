use crate::{EncodeError, Marker,DecodeError, Value, Pack, Unpack};
use crate::ll::types::sized::write_body_by_iter;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq)]
/// An anonymous, generic variant for structure values. It does denote different structures by
/// a `tag_byte` field; all fields are written and read as [`Value`](crate::value::Value) in the
/// order in which they were given. This allows for parsing of any structure which is validly
/// encoded, valid in the PackStream specification sense, i.e. the struct marker and the field
/// size, a tag byte denoting the struct type and then a list of the fields.
///
/// It does not allow for recursive patterns; in a struct there is no other structure allowed.
///
/// ## Encode and Decode
/// Encoding and Decoding is given by the generic [`Pack`](crate::packable::Pack) and [`Unpack`](crate::packable::Unpack)
/// implementation for `Value<S>`.
/// ```
/// # use packs::{Value, GenericStruct, Pack, Unpack};
/// let s = GenericStruct {
///         tag_byte: 0x01,
///         fields: vec!(Value::Float(42.0), Value::String(String::from("hello world"))),
/// };
///
/// let value = Value::Structure(s);
///
/// let mut buffer = Vec::new();
/// value.encode(&mut buffer).unwrap();
///
/// let res = <Value<GenericStruct>>::decode(&mut buffer.as_slice()).unwrap();
///
/// assert_eq!(res, value);
/// ```
pub struct GenericStruct {
    pub tag_byte: u8,
    pub fields: Vec<Value<GenericStruct>>,
}

impl Pack for GenericStruct {
    fn encode<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Marker::Structure(self.fields.len(), self.tag_byte).encode(writer)?;
        Ok(2 + write_body_by_iter(&mut self.fields.iter(), writer)?)
    }
}

impl Unpack for GenericStruct {
    fn decode_body<T: Read>(marker: Marker, reader: &mut T) -> Result<Self, DecodeError> {
        match marker {
            Marker::Structure(sz, tag_byte) => {
                let mut res = Vec::with_capacity(sz);
                for _ in 0..sz {
                    let val = <Value<GenericStruct>>::decode(reader)?;
                    res.push(val);
                }

                Ok(GenericStruct {
                    tag_byte,
                    fields: res
                })
            },
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

#[derive(Debug, PartialEq)]
/// A void implementation with `Pack` and `Unpack` which can be used as a placeholder to deny any
/// structures.
pub enum NoStruct {}

impl Pack for NoStruct {
    fn encode<T: Write>(&self, _: &mut T) -> Result<usize, EncodeError> {
        unreachable!()
    }
}

impl Unpack for NoStruct {
    fn decode_body<T: Read>(marker: Marker, _: &mut T) -> Result<Self, DecodeError> {
        match marker {
            Marker::Structure(_, _) => Err(DecodeError::TryingToDecodeNoStruct),
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}
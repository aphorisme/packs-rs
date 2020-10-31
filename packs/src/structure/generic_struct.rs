use std::io::{Read, Write};
use crate::{DecodeError, EncodeError, Value, PackableStructSum};
use crate::packable::{Pack, Unpack};
use crate::ll::marker::Marker;
use byteorder::{WriteBytesExt, ReadBytesExt};

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
/// implementation for `Value<S>` where `S: `[`PackableStructSum`](crate::structure::struct_sum::PackableStructSum):
/// ```
/// # use packs::structure::generic_struct::GenericStruct;
/// # use packs::value::Value;
/// # use packs::packable::{Pack, Unpack};
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

impl PackableStructSum for GenericStruct {
    fn read_struct_body<T: Read>(size: usize, tag_byte: u8, reader: &mut T) -> Result<Self, DecodeError> {
        let mut fields = Vec::with_capacity(size);
        for _ in 0..size {
            let value = Value::decode(reader)?;
            fields.push(value);
        }

        Ok(GenericStruct {
            tag_byte,
            fields,
        })
    }

    fn write_struct_body<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let mut written = 0;
        for val in &self.fields {
            written += val.encode(writer)?;
        }
        Ok(written)
    }

    fn fields_len(&self) -> usize {
        self.fields.len()
    }

    fn tag_byte(&self) -> u8 {
        self.tag_byte
    }
}

impl<T: Write> Pack<T> for GenericStruct {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Marker::Structure(self.fields_len()).encode(writer)?;
        writer.write_u8(self.tag_byte())?;
        Ok(2 + self.write_struct_body(writer)?)
    }
}

impl<T: Read> Unpack<T> for GenericStruct {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        let marker = Marker::decode(reader)?;
        match marker {
            Marker::Structure(sz) => {
                let tag_byte = reader.read_u8()?;
                GenericStruct::read_struct_body(sz, tag_byte, reader)
            },
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}


use std::io::{Read, Write};
use crate::error::{DecodeError, EncodeError};
use crate::ll::marker::Marker;
use byteorder::{WriteBytesExt, ReadBytesExt};
use packable_struct::PackableStruct;

pub mod packable_struct;
pub mod struct_sum;
pub mod generic_struct;

/// Encodes a `PackableStruct` as a PackStream struct with the provided tag byte.
pub fn encode_struct<S: PackableStruct, T: Write>(value: &S, tag_byte: u8, writer: &mut T) -> Result<usize, EncodeError> {
    Marker::Structure(S::FIELDS).encode(writer)?;
    writer.write_u8(tag_byte)?;
    Ok(2 + value.write_structure_body(writer)?)
}

/// Tries to read a PackStream struct with provided tag byte. Checks for the number of fields as
/// well as the read tag byte and fails, if at least one does not match.
pub fn decode_struct<S: PackableStruct, T: Read>(tag_byte: u8, reader: &mut T) -> Result<S, DecodeError> {
    let marker = Marker::decode(reader)?;
    match marker {
        Marker::Structure(u) => {
            if u != S::FIELDS {
                Err(DecodeError::UnexpectedNumberOfFields(S::FIELDS, u))
            } else {
                let tag = reader.read_u8()?;
                if tag != tag_byte {
                    Err(DecodeError::UnexpectedTagByte(tag))
                } else {
                    S::read_structure_body(reader)
                }
            }
        },
        _ => Err(DecodeError::UnexpectedMarker(marker))
    }
}

#[cfg(test)]
pub mod test {
    use crate::{PackableStruct, encode_struct, decode_struct};
    use std::fmt::Debug;

    pub fn pack_unpack_struct_test<T: PackableStruct + PartialEq + Debug>(value: &T, expected_size: usize, tag_byte: u8) {
        let mut buffer = Vec::new();

        let written = encode_struct(value, tag_byte, &mut buffer).unwrap();
        assert_eq!(expected_size, written);

        let res: T = decode_struct(tag_byte, &mut buffer.as_slice()).unwrap();
        assert_eq!(value, &res);
    }
}

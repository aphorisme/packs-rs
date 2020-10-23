use std::io::{Read, Write};
use crate::{DecodeError, EncodeError, Pack, Unpack};
use crate::ll::marker::Marker;
use byteorder::{WriteBytesExt, ReadBytesExt};

/// A sum type of possible structs this type supports. The type is a trait to abstract away
/// from a type which looks something like
/// ```
/// struct S1;
/// struct S2;
///
/// enum ValueStructs {
///     S1(S1),
///     S2(S2)
/// }
/// ```
/// in the context of PackStream. This would then implement the trait as follows:
/// ```
/// # use packs::error::{EncodeError, DecodeError};
/// # use std::io::{Read, Write};
/// # use packs::structure::struct_sum::PackableStructSum;
/// # struct S1;
/// # struct S2;
/// # enum ValueStructs {
/// #    S1(S1),
/// #     S2(S2)
/// # }
/// impl PackableStructSum for ValueStructs {
/// fn read_struct_body<T: Read>(size: usize,tag_byte: u8,reader: &mut T) -> Result<Self, DecodeError> {
///     match tag_byte {
///          0x01 => todo!(), // S1's tag byte, read S1
///          0x02 => todo!(), // S2's tag byte, read S2
///          _ => Err(DecodeError::UnexpectedTagByte(tag_byte))
///     }
/// }
///
/// fn write_struct_body<T: Write>(&self,writer: &mut T) -> Result<usize, EncodeError> {
///     match self {
///         ValueStructs::S1(s1) => todo!(), // write s1's body (no marker, no size, no tag)
///         ValueStructs::S2(s2) => todo!(), // write s2's body (..)
///     }
/// }
///
/// fn fields_len(&self) -> usize {
///     2
/// }
///
/// fn tag_byte(&self) -> u8 {
///     match self {
///         ValueStructs::S1(_) => 0x01,
///         ValueStructs::S2(_) => 0x02,
/// }
/// }
///
/// }
/// ```
/// Usually, such a variant can be generate with a macro provided by `packs_proc`.
///
/// ## PackStream implementation
/// A `PackableStructSum` has an auto-implementation for `Pack` and `Unpack` making it possible to
/// decode and encode user defined structs, which are part of a variant.
pub trait PackableStructSum: Sized {
    fn read_struct_body<T: Read>(size: usize, tag_byte: u8, reader: &mut T) -> Result<Self, DecodeError>;
    fn write_struct_body<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError>;
    fn fields_len(&self) -> usize;
    fn tag_byte(&self) -> u8;
}

#[derive(Debug, PartialEq)]
/// A void implementation for `PackableStructSum` which can be used a placeholder to deny any
/// structures.
pub enum NoStruct {}

impl PackableStructSum for NoStruct {
    fn read_struct_body<T: Read>(_: usize, _: u8, _: &mut T) -> Result<Self, DecodeError> {
        panic!("Trying to read Empty Struct Sum.")
    }

    fn write_struct_body<T: Write>(&self, _: &mut T) -> Result<usize, EncodeError> {
        Ok(0)
    }

    fn fields_len(&self) -> usize {
        0
    }

    fn tag_byte(&self) -> u8 {
        panic!("Trying to retrieve tag byte for Empty Struct Sum")
    }
}

impl<S: PackableStructSum, T: Write> Pack<T> for S {
    fn encode(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Marker::Structure(self.fields_len()).encode(writer)?;
        writer.write_u8(self.tag_byte())?;
        Ok(2 + self.write_struct_body(writer)?)
    }
}

impl<S: PackableStructSum, T: Read> Unpack<T> for S {
    fn decode(reader: &mut T) -> Result<Self, DecodeError> {
        let marker = Marker::decode(reader)?;
        match marker {
            Marker::Structure(sz) => {
                let tag_byte = reader.read_u8()?;
                S::read_struct_body(sz, tag_byte, reader)
            },
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::{PackableStructSum, Pack, Unpack};
    use std::fmt::Debug;

    pub fn pack_unpack_struct_sum_test<T: PackableStructSum + PartialEq + Debug>(value: &T, expected_size: usize) {
        let mut buffer = Vec::new();

        let written = T::encode(value, &mut buffer).unwrap();
        assert_eq!(expected_size, written);

        let res: T = T::decode(&mut buffer.as_slice()).unwrap();
        assert_eq!(value, &res, "value '{:?}' got encoded->decoded into '{:?}'", value, res);
    }

}

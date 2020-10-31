use std::io::{Read, Write};
use crate::{DecodeError, EncodeError};

/// Denotes a struct which can be encoded and decoded using [`encode_struct`](crate::structure::encode_struct)
/// and [`decode_struct`](crate::structure::decode_struct).
///
/// ## Implementation
/// This trait can be derived using `#[derive(PackableStruct)]` if all fields implement
/// [`Pack`](crate::packable::Pack) and [`Unpack`](crate::packable::Unpack) and there are at most
///  `15` fields. This is the recommended way of using this trait.
///
/// For a valid implementation in the context of PackStream, the limit of 15 fields must not be
/// exceeded.
pub trait PackableStruct: Sized {
    const FIELDS: usize;
    fn read_structure_body<T: Read>(reader: &mut T) -> Result<Self, DecodeError>;
    fn write_structure_body<T: Write>(&self, writer: &mut T) -> Result<usize, EncodeError>;
}

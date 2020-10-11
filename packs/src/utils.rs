use crate::{Pack, EncodeError, Unpack, DecodeError};
use std::io::{Write, Read};

/// Encodes a given key and value as a property as used by `Dictionary`. This can be used as a flat
/// shortcut to encode any key-value pair using PackStream. Keys are strings and encoded values can
/// be anything which implements [`Pack`](crate::packable::Pack). Returns the number of bytes written
/// to the stream.
pub fn encode_property<T: Write, V: Pack<T>>(key: &str, value: &V, writer: &mut T) -> Result<usize, EncodeError> {
    Ok(String::from(key).encode(writer)? + value.encode(writer)?)
}

/// Decodes a key-value pair using PackStream. Keys are strings, but values can be anything which
/// implements [`Unpack`](crate::Packable::Unpack).
pub fn decode_property<T: Read, V: Unpack<T>>(reader: &mut T) -> Result<(String, V), DecodeError> {
    let key = String::decode(reader)?;
    let value = V::decode(reader)?;
    Ok((key, value))
}
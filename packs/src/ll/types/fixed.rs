use std::io::Read;
use byteorder::ReadBytesExt;
use crate::ll::marker::Marker;
use std::io::{Write};
use std::io;
use byteorder::{WriteBytesExt, BigEndian};

pub fn decode_minus_tiny_int<T: Read>(mut reader: T) -> io::Result<i8> {
    reader.read_u8().map(byte_to_minus_tiny_int)
}

/// Calculates from a (marker) byte the respective `i8`. The value of a `MinusTinyInt` is
/// encoded as a 2-complement, but ranges only from `0xFF` to `0xF0`.
/// ```
/// use packs::ll::types::fixed::byte_to_minus_tiny_int;
/// let byte : u8 = 0xFF;
/// assert_eq!(-1, byte_to_minus_tiny_int(byte));
/// ```
/// **Note**: This function does not check, if the first nibble is `0xF0`.
pub fn byte_to_minus_tiny_int(value: u8) -> i8 {
    // (value & 0x0F) as i8 - 16
    value as i8
}


/// The reverse function to `byte_to_minus_tiny_int` for correct values.
/// **Note**: This function does not check if the input is a valid `MinusTinyInt`.
pub fn minus_tiny_int_to_byte(value: i8) -> u8 {
    // 0xF0 | (0x0F & (value + 16)) as u8
    value as u8
}


/// Encodes the a `i8` into the bolt type `MINUS_TINY_INT` which can hold any
/// number from `-1` to `-16`. Input outside of this range leads to undefined
/// output.
pub fn encode_minus_tiny_int<T: Write>(from: i8, mut into: T) -> io::Result<usize> {
    let res = minus_tiny_int_to_byte(from);
    into.write_u8(res)?;
    Ok(1)
}

pub fn encode_plus_tiny_int<T: Write>(from: u8, mut into: T) -> io::Result<usize> {
    into.write_u8(from)?;
    Ok(1)
}

pub fn decode_plus_tiny_int<T: Read>(from: &mut T) -> io::Result<u8> {
    from.read_u8()
}

pub fn encode_i8<T: Write>(from: i8, mut into: T) -> io::Result<usize> {
    Marker::Int8.encode(&mut into)?;
    into.write_i8(from)?;
    Ok(2)
}

pub fn encode_i16<T: Write>(from: i16, mut into: T) -> io::Result<usize> {
    Marker::Int16.encode(&mut into)?;
    into.write_i16::<BigEndian>(from)?;
    Ok(3)
}

pub fn encode_i32<T: Write>(from: i32, mut into: T) -> io::Result<usize> {
    Marker::Int32.encode(&mut into)?;
    into.write_i32::<BigEndian>(from)?;
    Ok(5)
}

pub fn encode_i64<T: Write>(from: i64, mut into: T) -> io::Result<usize> {
    Marker::Int64.encode(&mut into)?;
    into.write_i64::<BigEndian>(from)?;
    Ok(9)
}

#[cfg(test)]
pub mod test {
    mod encoding {
        use crate::ll::types::fixed::{encode_minus_tiny_int, decode_minus_tiny_int, encode_plus_tiny_int, decode_plus_tiny_int};
        use crate::ll::bounds::{MAX_PLUS_TINY_INT, MIN_MINUS_TINY_INT};

        #[test]
        fn minus_tiny_ints_encode() {
            let mut buffer = Vec::with_capacity(16);
            for i in MIN_MINUS_TINY_INT..0 {
                encode_minus_tiny_int(i, &mut buffer).unwrap();
            }

            for i in 0..-MIN_MINUS_TINY_INT {
                assert_eq!(buffer[i as usize], 0xF0 | i as u8);
            }
        }

        #[test]
        fn minus_tiny_ints_decode() {
            let mut buffer: Vec<u8> = Vec::with_capacity(16);
            for i in 0x00u8..0x10 {
                buffer.push(0xF0 | i)
            }

            let mut iter = buffer.as_slice();
            for i in MIN_MINUS_TINY_INT..0 {
                assert_eq!(decode_minus_tiny_int(&mut iter).unwrap(), i);
            }
        }

        #[test]
        fn minus_tiny_ints_encode_decode() {
            let mut buffer: Vec<u8> = Vec::with_capacity(16);
            for i in MIN_MINUS_TINY_INT..0 {
                encode_minus_tiny_int(i, &mut buffer).unwrap();
            }

            let mut iter = buffer.as_slice();
            for i in MIN_MINUS_TINY_INT..0 {
                let res = decode_minus_tiny_int(&mut iter).unwrap();
                assert_eq!(res, i);
            }
        }

        #[test]
        fn plus_tiny_ints_encode_decode() {
            let mut buffer: Vec<u8> = Vec::with_capacity(MAX_PLUS_TINY_INT as usize);
            for i in 0u8..(MAX_PLUS_TINY_INT + 1) {
                encode_plus_tiny_int(i, &mut buffer).unwrap();
            }

            let mut iter = buffer.as_slice();
            for i in 0..(MAX_PLUS_TINY_INT + 1) {
                let res = decode_plus_tiny_int(&mut iter).unwrap();
                assert_eq!(res, i);
            }
        }
    }
}


use std::io::Read;
use crate::ll::marker::Marker;
use std::io::{Write};
use std::io;

pub fn decode_minus_tiny_int<T: Read>(mut reader: T) -> io::Result<i8> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    Ok(byte_to_minus_tiny_int(buf[0]))
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
    into.write(&[res])
}

pub fn encode_plus_tiny_int<T: Write>(from: u8, mut into: T) -> io::Result<usize> {
    into.write(&[from])
}

pub fn decode_plus_tiny_int<T: Read>(from: &mut T) -> io::Result<u8> {
    let mut buf = [0; 1];
    from.read_exact(&mut buf)?;
    Ok(buf[0])
}

pub fn encode_i8<T: Write>(from: i8, mut into: T) -> io::Result<usize> {
    Marker::Int8.encode(&mut into)?;
    Ok(1 + into.write(&from.to_be_bytes())?)
}

pub fn decode_body_i8<T: Read>(mut from: T) -> io::Result<i8> {
    let mut buf = [0; 1];
    from.read_exact(&mut buf)?;
    Ok(i8::from_be_bytes(buf))
}

pub fn encode_i16<T: Write>(from: i16, mut into: T) -> io::Result<usize> {
    Marker::Int16.encode(&mut into)?;
    Ok(1 + into.write(&from.to_be_bytes())?)
}

pub fn decode_body_i16<T: Read>(mut from: T) -> io::Result<i16> {
    let mut buf = [0; 2];
    from.read_exact(&mut buf)?;
    Ok(i16::from_be_bytes(buf))
}

pub fn encode_i32<T: Write>(from: i32, mut into: T) -> io::Result<usize> {
    Marker::Int32.encode(&mut into)?;
    Ok(1 + into.write(&from.to_be_bytes())?)
}

pub fn decode_body_i32<T: Read>(mut from: T) -> io::Result<i32> {
    let mut buf = [0; 4];
    from.read_exact(&mut buf)?;
    Ok(i32::from_be_bytes(buf))
}

pub fn encode_i64<T: Write>(from: i64, mut into: T) -> io::Result<usize> {
    Marker::Int64.encode(&mut into)?;
    Ok(1 + into.write(&from.to_be_bytes())?)
}

pub fn decode_body_i64<T: Read>(mut from: T) -> io::Result<i64> {
    let mut buf = [0; 8];
    from.read_exact(&mut buf)?;
    Ok(i64::from_be_bytes(buf))
}

pub fn encode_f64<T: Write>(from: f64, mut into: T) -> io::Result<usize> {
    Ok(Marker::Float64.encode(&mut into)? + into.write(&from.to_be_bytes())?)
}

pub fn decode_body_f64<T: Read>(mut from: T) -> io::Result<f64> {
    let mut buf = [0; 8];
    from.read_exact(&mut buf)?;
    Ok(f64::from_be_bytes(buf))
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


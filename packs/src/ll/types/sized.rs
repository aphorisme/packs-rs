use std::io::{Read, Write};
use crate::ll::marker::{Marker};
use crate::error::{DecodeError, EncodeError};
use crate::packable::{Unpack, Pack};
use std::collections::HashMap;
use crate::ll::types::lengths::{Length, read_size_8, read_size_16, read_size_32};
use crate::value::bytes::Bytes;

//-----------------------//
//         API           //
//-----------------------//
/// Encodes any type which has implemented the internal `SizedType` trait. This provides a way
/// to reuse code of different sized encodings, like String, List and Dictionary.
pub fn encode_sized<T: Write, S: SizedType + SizedTypePack<T> + Sized>(value: &S, writer: &mut T) -> Result<usize, EncodeError> {
    let written =
        write_header(value, writer)? + S::write_body(value, writer)?;
    Ok(written)
}

/// Decodes any type which has implemented the internal `SizedType` trait. Counterpart to
/// `encode_sized`.
pub fn decode_sized<T: Read, S: SizedType + SizedTypeUnpack<T> + Sized>(reader: &mut T) -> Result<S, DecodeError> {
    let size = read_header::<T, S>(reader)?;
    Ok(S::read_body(size, reader)?)
}


//------------------------------//
//           HELPERS            //
//------------------------------//
fn read_header<'a, T: Read, S: SizedTypeUnpack<T>>(mut reader: &mut T) -> Result<usize, DecodeError> {
    let marker = Marker::decode(&mut reader)?;
    S::read_length(marker, reader)
}

fn write_header<T: Write, S: SizedType>(value: &S, writer: &mut T) -> Result<usize, EncodeError> {
    let (marker, len) = value.header();
    Ok(marker.encode(writer)? + len.encode(writer)?)
}

//--------------------------------------------//
//              GENERIC PART                  //
//--------------------------------------------//
pub trait SizedTypePack<T: Write> : Sized {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError>;
}

pub trait SizedTypeUnpack<T: Read> : Sized {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError>;
    fn read_body(len: usize, reader: &mut T) -> Result<Self, DecodeError>;
}

pub trait SizedType: Sized {
    fn header(&self) -> (Marker, Length);

}

impl SizedType for String {
    fn header(&self) -> (Marker, Length) {
        let len = Length::from_usize(self.len()).expect(&format!("string has invalid length '{}'", self.len()));
        match len {
            Length::Tiny(u) => (Marker::TinyString(u as usize), len),
            Length::Bit8(_) => (Marker::String8, len),
            Length::Bit16(_) => (Marker::String16, len),
            Length::Bit32(_) => (Marker::String32, len),
        }
    }
}

impl<T: Write> SizedTypePack<T> for String {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Ok(writer.write(self.as_bytes())?)
    }

}

impl<T: Read> SizedTypeUnpack<T> for String {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
        match marker {
            Marker::TinyString(u) => Ok(u),
            Marker::String8 => read_size_8(reader),
            Marker::String16 => read_size_16(reader),
            Marker::String32 => read_size_32(reader),
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }

    fn read_body(len: usize, reader: &mut T) -> Result<Self, DecodeError> {
        let mut result = String::new();
        reader.take(len as u64).read_to_string(&mut result)?;
        Ok(result)
    }
}

impl<P> SizedType for Vec<P> {
    fn header(&self) -> (Marker, Length) {
        let len =
            Length::from_usize(self.len()).expect(&format!("Vec has invalid length '{}'", self.len()));
        match len {
            Length::Tiny(u) => (Marker::TinyList(u as usize), len),
            Length::Bit8(_) => (Marker::List8, len),
            Length::Bit16(_) => (Marker::List16, len),
            Length::Bit32(_) => (Marker::List32, len),
        }
    }
}

impl<T: Write, P: Pack<T>> SizedTypePack<T> for Vec<P> {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let mut u = 0;
        for p in self {
            u += p.encode(writer)?;
        }
        Ok(u)
    }
}

impl<T: Read, P: Unpack<T>> SizedTypeUnpack<T> for Vec<P> {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
        match marker {
            Marker::TinyList(u) => Ok(u),
            Marker::List8 => read_size_8(reader),
            Marker::List16 => read_size_16(reader),
            Marker::List32 => read_size_32(reader),
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }

    fn read_body(len: usize, reader: & mut T) -> Result<Self, DecodeError> {
        let mut res: Vec<P> = Vec::with_capacity(len);
        for _ in 0..len {
            let p = P::decode(reader)?;
            res.push(p);
        }

        Ok(res)
    }
}

impl<P> SizedType for HashMap<String, P> {
    fn header(&self) -> (Marker, Length) {
        let len =
            Length::from_usize(self.len())
                .expect(&format!("HashMap has invalid length '{}'", self.len()));

        match len {
            Length::Tiny(u) => (Marker::TinyDictionary(u as usize), len),
            Length::Bit8(_) => (Marker::Dictionary8, len),
            Length::Bit16(_) => (Marker::Dictionary16, len),
            Length::Bit32(_) => (Marker::Dictionary32, len),
        }
    }
}

impl<T: Write, P: Pack<T>> SizedTypePack<T> for HashMap<String, P> {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let mut u = 0;
        for (k, p) in self {
            u += String::encode(k, writer)?;
            u += P::encode(p, writer)?;
        }

        Ok(u)
    }

}

impl<T: Read, P: Unpack<T>> SizedTypeUnpack<T> for HashMap<String, P> {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
        match marker {
            Marker::TinyDictionary(u) => Ok(u),
            Marker::Dictionary8 => read_size_8(reader),
            Marker::Dictionary16 => read_size_16(reader),
            Marker::Dictionary32 => read_size_32(reader),
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }

    fn read_body(len: usize, reader: &mut T) -> Result<Self, DecodeError> {
        let mut res = HashMap::with_capacity(len);
        for _ in 0..len {
            let k = String::decode(reader)?;
            let p = P::decode(reader)?;
            res.insert(k, p);
        }

        Ok(res)
    }
}

impl SizedType for Bytes {
    fn header(&self) -> (Marker, Length) {
        let len =
            Length::from_usize(self.0.len())
                .expect(&format!("Vec<u8> has invalid size '{}'", self.0.len()));

        match len {
            Length::Tiny(u) => (Marker::Bytes8, Length::Bit8(u)),
            Length::Bit8(_) => (Marker::Bytes8, len),
            Length::Bit16(_) => (Marker::Bytes16, len),
            Length::Bit32(_) => (Marker::Bytes32, len),
        }
    }
}

impl<T: Write> SizedTypePack<T> for Bytes {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Ok(writer.write(self.0.as_slice())?)
    }
}

impl<T: Read> SizedTypeUnpack<T> for Bytes {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
        match marker {
            Marker::Bytes8 => read_size_8(reader),
            Marker::Bytes16 => read_size_16(reader),
            Marker::Bytes32 => read_size_32(reader),
            _ => Err(DecodeError::UnexpectedMarker(marker))
        }
    }

    fn read_body(len: usize, reader: &mut T) -> Result<Self, DecodeError> {
        let mut res = vec![0; len];
        reader.read_exact(&mut res)?;
        Ok(Bytes(res))
    }
}
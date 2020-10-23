use std::io::{Read, Write};
use crate::ll::marker::{Marker};
use crate::error::{DecodeError, EncodeError};
use crate::packable::{Unpack, Pack};
use std::collections::{HashMap, HashSet};
use crate::ll::types::lengths::{Length, read_size_8, read_size_16, read_size_32, read_list_size, read_dict_size, read_string_size};
use crate::value::bytes::Bytes;
use std::hash::Hash;

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

fn write_body_by_iter<'a, T: Write, P: 'a + Pack<T>, C: Iterator<Item = &'a P>>(collection: &'a mut C, writer: &mut T) -> Result<usize, EncodeError> {
    let mut written = 0;
    for v in collection {
        written += v.encode(writer)?
    }
    Ok(written)
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

// --------------------------------------- //
//          String implementation          //
// --------------------------------------- //
impl SizedType for String {
    fn header(&self) -> (Marker, Length) {
        let len = Length::from_usize(self.len()).expect(&format!("string has invalid length '{}'", self.len()));
        (len.marker(
            Marker::TinyString,
            Marker::String8,
            Marker::String16,
            Marker::String32,
        ), len)
    }
}

impl<T: Write> SizedTypePack<T> for String {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError> {
        Ok(writer.write(self.as_bytes())?)
    }

}

impl<T: Read> SizedTypeUnpack<T> for String {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
        read_string_size(marker, reader)
    }

    fn read_body(len: usize, reader: &mut T) -> Result<Self, DecodeError> {
        let mut result = String::new();
        reader.take(len as u64).read_to_string(&mut result)?;
        Ok(result)
    }
}

// --------------------------------------- //
//            Vec implementation           //
// --------------------------------------- //
impl<P> SizedType for Vec<P> {
    fn header(&self) -> (Marker, Length) {
        let len =
            Length::from_usize(self.len()).expect(&format!("Vec has invalid length '{}'", self.len()));

        (len.marker(
            Marker::TinyList,
            Marker::List8,
            Marker::List16,
            Marker::List32),
         len)
    }
}

impl<T: Read, P: Unpack<T>> SizedTypeUnpack<T> for Vec<P> {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
        read_list_size(marker, reader)
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

impl<T: Write, P: Pack<T>> SizedTypePack<T> for Vec<P> {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError> {
        write_body_by_iter(&mut self.iter(), writer)
    }
}

// --------------------------------------- //
//          HashSet implementation         //
// --------------------------------------- //
impl<T: Write, P: Pack<T>> SizedTypePack<T> for HashSet<P> {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError> {
        write_body_by_iter(&mut self.iter(), writer)
    }
}

impl<T: Read, P: Unpack<T> + Hash + Eq> SizedTypeUnpack<T> for HashSet<P> {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
        read_list_size(marker, reader)
    }

    fn read_body(len: usize, reader: &mut T) -> Result<Self, DecodeError> {
        let mut res = HashSet::with_capacity(len);
        for _ in 0..len {
            let v = P::decode(reader)?;
            res.insert(v);
        }

        Ok(res)
    }
}

impl<P> SizedType for HashSet<P> {
    fn header(&self) -> (Marker, Length) {
        let len =
            Length::from_usize(self.len()).expect(&format!("HashSet has invalid length '{}'", self.len()));

        (len.marker(
            Marker::TinyList,
            Marker::List8,
            Marker::List16,
            Marker::List32),
         len)
    }
}

// --------------------------------------- //
//          HashMap Implementation        //
// --------------------------------------- //
impl<P> SizedType for HashMap<String, P> {
    fn header(&self) -> (Marker, Length) {
        let len =
            Length::from_usize(self.len())
                .expect(&format!("HashMap has invalid length '{}'", self.len()));

        (len.marker(
            Marker::TinyDictionary,
            Marker::Dictionary8,
            Marker::Dictionary16,
            Marker::Dictionary32
        ), len)
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
        read_dict_size(marker, reader)
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


// --------------------------------------- //
//           Bytes Implementation          //
// --------------------------------------- //
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

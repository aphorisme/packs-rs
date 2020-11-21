use std::io::{Write, Read};
use packs::*;

#[derive(Debug, PartialEq)]
struct Part {
    field_a: String,
    field_b: Option<String>,
}

fn pack_part<T: Write>(part: &Part, writer: &mut T) -> Result<usize, EncodeError> {
    Ok(&part.field_a.encode(writer)? + &part.field_b.encode(writer)?)
}

fn unpack_part<T: Read>(reader: &mut T) -> Result<Part, DecodeError> {
    let field_a = String::decode(reader)?;
    let field_b = <Option<String>>::decode(reader)?;

    Ok(
        Part {
            field_a,
            field_b,
        }
    )
}

#[derive(Debug, PartialEq, Unpack, Pack)]
#[tag = 0x01]
struct Complete {
    #[pack(pack_part)]
    #[unpack(unpack_part)]
    #[fields = 2]
    part: Part,
    id: i64,
}


#[test]
fn pack_unpack_struct() {
    let c = Complete {
        part: Part {
            field_a: String::from("field A"),
            field_b: None,
        },
        id: 42,
    };

    let mut buf = Vec::new();
    c.encode(&mut buf).expect("Cannot encode 'Complete'");
    let c_decode =
        Complete::decode(&mut buf.as_slice()).expect("Cannot decode 'Complete'");

    assert_eq!(c, c_decode);
}

#[test]
fn pack_check_struct() {
    let c = Complete {
        part: Part {
            field_a: String::from("field A"),
            field_b: None,
        },
        id: 42,
    };

    let mut buf = Vec::new();
    c.encode(&mut buf).expect("Cannot encode 'Complete'");

    assert_eq!(buf.len(), 1 + 1 + (1 + 7) + 1 + 1);
    assert_eq!(
        buf,
        vec![
            0xB3, 0x01, // header, 3 fields + tag byte
            0x87, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x41, // "field A"
            0xC0, // None
            42]); // 42
}
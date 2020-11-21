use std::io::Write;
use crate::{Pack, EncodeError};

pub fn write_body_by_iter<'a, T: Write, P: 'a + Pack, C: Iterator<Item = &'a P>>(collection: &'a mut C, writer: &mut T) -> Result<usize, EncodeError> {
    let mut written = 0;
    for v in collection {
        written += v.encode(writer)?
    }
    Ok(written)
}

use crate::*;

#[derive(Clone, PartialEq, Debug, PackableStruct, Pack, Unpack)]
#[tag = 0x74]
pub struct LocalTime {
    pub nanoseconds: i64,
}
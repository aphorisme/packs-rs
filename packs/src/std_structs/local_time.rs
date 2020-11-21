use crate::*;

#[derive(Clone, PartialEq, Debug, Pack, Unpack)]
#[tag = 0x74]
pub struct LocalTime {
    pub nanoseconds: i64,
}
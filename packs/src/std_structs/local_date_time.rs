use crate::*;

#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x64]
pub struct LocalDateTime {
    pub seconds: i64,
    pub nanoseconds: i64,
}

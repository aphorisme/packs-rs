use crate::*;

#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x54]
pub struct Time {
    pub nanoseconds: i64,
    pub tz_offset_seconds: i64,
}

impl Time {
    pub fn utc_nanoseconds(&self) -> i64 {
        self.nanoseconds - (self.tz_offset_seconds * 1000000000)
    }
}
use crate::*;

#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x46]
pub struct DateTime {
    pub seconds: i64,
    pub nanoseconds: i64,
    pub tz_offset_minutes: i64,
}

impl DateTime {
    pub fn utc_nanoseconds(&self) -> i64 {
        (self.seconds * 1000000000) + self.nanoseconds - (self.tz_offset_minutes * 60 * 1000000000)
    }
}
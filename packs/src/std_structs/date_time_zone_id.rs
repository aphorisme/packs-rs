use crate::*;

#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x66]
pub struct DateTimeZoneId {
    pub seconds: i64,
    pub nanoseconds: i64,
    pub tz_id: i64,
}

impl DateTimeZoneId {
    pub fn utc_nanoseconds(&self) -> i64 {
        // (seconds * 1000000000) + nanoseconds - get_offset_in_nanoseconds(tz_id)
        todo!()
    }
}
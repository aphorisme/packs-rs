use crate::*;

#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x44]
pub struct Date {
    pub days: i64,
}
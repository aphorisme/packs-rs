use crate::*;

#[derive(Debug, Clone, PartialEq, PackableStruct, Pack, Unpack)]
#[tag = 0x58]
pub struct Point2D {
    pub srid: i64,
    pub x: f64,
    pub y: f64,
}
use crate::*;

#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x59]
pub struct Point3D {
    pub srid: i64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
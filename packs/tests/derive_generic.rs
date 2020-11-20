use packs::*;

#[derive(Debug, Pack, Unpack)]
#[tag = 0xA0]
pub struct Foo<T> {
    t: T,
    id: i64,
}
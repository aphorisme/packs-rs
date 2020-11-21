use crate::*;
use crate::std_structs::{StdStructPrimitive};

#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x52]
pub struct Relationship {
    pub id: i64,
    pub start_node_id: i64,
    pub end_node_id: i64,
    pub _type: String,
    pub properties: Dictionary<StdStructPrimitive>
}

impl Relationship {
    pub fn new(id: i64, _type: &str, from: i64, to: i64) -> Self {
        Relationship {
            id,
            start_node_id: from,
            end_node_id: to,
            _type: String::from(_type),
            properties: Dictionary::new(),
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packable::test::pack_unpack_test;
    use crate::Value;
    use crate::std_structs::relationship::Relationship;

    #[test]
    fn pack_unpack() {
        pack_unpack_test::<Relationship>(&[
            Relationship {
                id: 42,
                start_node_id: 1,
                end_node_id: 2,
                _type: String::from("KNOWS"),
                properties: vec![(String::from("foo"), Value::from(1))].into_iter().collect(),
            }
        ]);
    }
}
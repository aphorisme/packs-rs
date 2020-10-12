use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone, PartialEq, PackableStruct, Pack, Unpack)]
#[tag = 0x52]
pub struct Relationship<T> {
    pub id: i64,
    pub start_node_id: i64,
    pub end_node_id: i64,
    pub _type: String,
    pub properties: HashMap<String, Value<T>>
}

impl<T> Relationship<T> {
    pub fn new(id: i64, _type: &str, from: i64, to: i64) -> Self {
        Relationship {
            id,
            start_node_id: from,
            end_node_id: to,
            _type: String::from(_type),
            properties: HashMap::new(),
        }
    }

    pub fn add_property<V: Into<Value<T>>>(&mut self, key: &str, value: V) -> Option<Value<T>> {
        self.properties.insert(String::from(key), value.into())
    }
}

#[cfg(test)]
pub mod test {
    use crate::packable::test::pack_unpack_test;
    use crate::Value;
    use crate::std_structs::relationship::Relationship;

    #[test]
    fn pack_unpack() {
        pack_unpack_test::<Relationship<()>>(&[
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
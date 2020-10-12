use std::collections::{HashMap, HashSet};
use crate::*;
use crate::std_structs::StdStruct;

#[derive(Debug, Clone, PartialEq, PackableStruct, Pack, Unpack)]
#[tag = 0x4E]
pub struct Node {
    pub id: i64,
    pub labels: HashSet<String>,
    pub properties: HashMap<String, Value<StdStruct>>
}

impl Node {
    pub fn new(id: i64) -> Self {
        Node {
            id,
            labels: HashSet::new(),
            properties: HashMap::new(),
        }
    }

    pub fn add_property<V: Into<Value<StdStruct>>>(&mut self, key: &str, value: V) -> Option<Value<StdStruct>> {
        self.properties.insert(String::from(key), value.into())
    }

    pub fn add_label(&mut self, label: &str) {
        self.labels.insert(String::from(label));
    }
}

#[cfg(test)]
pub mod test {
    use crate::packable::test::pack_unpack_test;
    use crate::std_structs::node::Node;
    use crate::value::Value;

    #[test]
    fn pack_unpack() {
        pack_unpack_test::<Node>(&[
            Node {
                id: 42,
                labels: vec!(String::from("Person"), String::from("Author")).into_iter().collect(),
                properties: vec![
                    (String::from("name"), Value::from("Hans Fallada")),
                    (String::from("age"), Value::from(32)),
                ].into_iter().collect(),
            }
        ],)
    }
}
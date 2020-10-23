use std::collections::{HashSet};
use crate::*;
use crate::std_structs::StdStruct;

#[derive(Debug, Clone, PartialEq, PackableStruct, Pack, Unpack)]
#[tag = 0x4E]
pub struct Node {
    pub id: i64,
    pub labels: HashSet<String>,
    pub properties: Dictionary<StdStruct>
}

impl Node {
    pub fn new(id: i64) -> Self {
        Node {
            id,
            labels: HashSet::new(),
            properties: Dictionary::new(),
        }
    }

    pub fn add_label(&mut self, label: &str) {
        self.labels.insert(String::from(label));
    }
}

#[cfg(test)]
pub mod test {
    use crate::packable::test::{pack_unpack_test, pack_to_test};
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

    #[test]
    fn pack_into() {
        let mut node = Node::new(42);
        node.add_label("Person");
        node.properties.add_property("name", "Hans");
        pack_to_test(
            node,
            &[0xB3, 0x4E,
                0x2A,
                0x91, 0x86, 0x50, 0x65, 0x72, 0x73, 0x6F, 0x6E,
                0xA1,
                    0x84, 0x6E, 0x61, 0x6D, 0x65,
                    0x84, 0x48, 0x61, 0x6E, 0x73]
        )
    }
}
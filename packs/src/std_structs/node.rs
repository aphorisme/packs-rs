use std::collections::{HashMap, HashSet};
use crate::*;

#[derive(Debug, Clone, PartialEq, PackableStruct, Pack, Unpack)]
#[tag = 0x4E]
pub struct Node<T> {
    pub id: i64,
    pub labels: HashSet<String>,
    pub properties: HashMap<String, Value<T>>
}

impl<T> Node<T> {
    pub fn add_property<V: Into<Value<T>>>(&mut self, key: &str, value: V) -> Option<Value<T>> {
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
        pack_unpack_test::<Node<()>>(&[
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
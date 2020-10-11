use std::collections::HashMap;
use crate::*;

#[derive(Debug, Clone, PartialEq, PackableStruct, Pack, Unpack)]
#[tag = 0x4E]
pub struct Node<T> {
    pub id: i64,
    pub labels: Vec<String>,
    pub properties: HashMap<String, Value<T>>
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
                labels: vec!(String::from("Person"), String::from("Author")),
                properties: vec![
                    (String::from("name"), Value::from("Hans Fallada")),
                    (String::from("age"), Value::from(32)),
                ].into_iter().collect(),
            }
        ],)
    }
}
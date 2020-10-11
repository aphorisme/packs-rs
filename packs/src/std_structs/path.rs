use crate::std_structs::relationship::Relationship;
use crate::std_structs::node::Node;
use crate::*;

#[derive(Debug, Clone, PartialEq, PackableStruct, Pack, Unpack)]
#[tag = 0x50]
pub struct Path<T> {
    pub nodes: Vec<Node<T>>,
    pub rels: Vec<Relationship<T>>,
    pub ids: Vec<i64>
}

#[cfg(test)]
pub mod test {
    use crate::packable::test::pack_unpack_test;
    use crate::Value;
    use crate::std_structs::path::Path;
    use crate::std_structs::node::Node;
    use std::collections::HashMap;
    use crate::std_structs::relationship::Relationship;

    #[test]
    fn pack_unpack() {
        pack_unpack_test::<Path<()>>(&[
            Path {
                nodes: vec!(
                    Node { id: 0, labels: vec!(String::from("Person"), String::from("Author")), properties: HashMap::new() },
                    Node {
                        id: 1,
                        labels: vec!(String::from("Book")),
                        properties: vec![(String::from("title"), Value::from("Puh der Bär"))].into_iter().collect()},
                    Node {
                        id: 4,
                        labels: vec!(String::from("Person")),
                        properties: vec![
                            (String::from("name"), Value::from("Oliver")),
                            (String::from("age"), Value::from(i32::max_value() as i64 + 1))]
                            .into_iter().collect(),
                    }
                ),

                rels: vec!(
                    Relationship {
                        id: 0,
                        start_node_id: 0,
                        end_node_id: 1,
                        _type: String::from("HAS_WRITTEN"),
                        properties: HashMap::new() },

                    Relationship {
                        id: 1,
                        start_node_id: 4,
                        end_node_id: 1,
                        _type: String::from("HAS_READ"),
                        properties: HashMap::new() },
                ),
                ids: vec!(0i64, 0i64, 1i64),
            }
        ])
    }
}


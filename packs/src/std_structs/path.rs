use crate::std_structs::relationship::Relationship;
use crate::std_structs::node::Node;
use crate::*;

#[derive(Debug, Clone, PartialEq, PackableStruct, Pack, Unpack)]
#[tag = 0x50]
pub struct Path {
    pub nodes: Vec<Node>,
    pub rels: Vec<Relationship>,
    pub ids: Vec<i64>
}

#[cfg(test)]
pub mod test {
    use crate::packable::test::pack_unpack_test;
    use crate::{Value, Dictionary};
    use crate::std_structs::path::Path;
    use crate::std_structs::node::Node;
    use crate::std_structs::relationship::Relationship;

    #[test]
    fn pack_unpack() {
        pack_unpack_test::<Path>(&[
            Path {
                nodes: vec!(
                    Node {
                        id: 0,
                        labels: vec!(String::from("Person"), String::from("Author")).into_iter().collect(),
                        properties: Dictionary::new() },
                    Node {
                        id: 1,
                        labels: vec!(String::from("Book")).into_iter().collect(),
                        properties: vec![(String::from("title"), Value::from("Puh der Bär"))].into_iter().collect()},
                    Node {
                        id: 4,
                        labels: vec!(String::from("Person")).into_iter().collect(),
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
                        properties: Dictionary::new() },

                    Relationship {
                        id: 1,
                        start_node_id: 4,
                        end_node_id: 1,
                        _type: String::from("HAS_READ"),
                        properties: Dictionary::new() },
                ),
                ids: vec!(0i64, 0i64, 1i64),
            }
        ])
    }
}


use crate::*;
use std::collections::HashMap;
use crate::std_structs::StdStruct;

#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x72]
pub struct UnboundRelationship {
    pub id: i64,
    pub _type: String,
    pub properties: HashMap<String, Value<StdStruct>>,
}

#[cfg(test)]
pub mod test {
    use crate::packable::test::pack_unpack_test;
    use crate::std_structs::unbound_relationship::UnboundRelationship;
    use crate::Value;

    #[test]
    fn pack_unpack() {
        pack_unpack_test::<UnboundRelationship>(&[
            UnboundRelationship {
                id: 0,
                _type: String::from("Hello # ö World"),
                properties:
                vec![(String::from("#"), Value::from(1)),
                     (String::from("yes"), Value::from(true))]
                    .into_iter().collect()
            }
        ]);
    }
}
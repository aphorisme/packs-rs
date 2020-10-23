# Version 0.2.0

### Breaking Changes:
- Removed `add_property` from `Node`.
- Replaced `()` with new `NoStruct` type for `PackableStructSum`.

### Additions:
- New `NoStruct` type to deny any struct sums on `Value<NoStruct>`.
- Added `Dictionary<T>` which replaces `HashMap<String, Value<T>>` in the
standard structs. 
- Added `ExtractRef` trait which denotes types into which a value
can be extracted to.
- Added `extract_list` variants to extract a `Value::List` into a
homogenous list if possible.

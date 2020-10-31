# Version 0.2.0

### Breaking Changes:
- Removed `add_property` from `Node`.
- Replaced `()` with new `NoStruct` type for `PackableStructSum`.
- Removed the default implementations for `Pack` and `Unpack` for all
`PackableStructSum`. Use the derive macros for `Pack` and `Unpack` now.
- Made the `#[tag = u8]` attribute mandatory (from optional) for sums as well
as for structs when deriving `Pack` and/or `Unpack`. 

### Additions:
- New `NoStruct` type to deny any struct sums on `Value<NoStruct>`.
- Added support in `DecodeError` for `NoStruct`.
- Added `Dictionary<T>` which replaces `HashMap<String, Value<T>>` in the
standard structs. 
- Added `ExtractRef` trait which denotes types into which a value
can be extracted to, as well as its variants `Extract` and `ExtractMut`.
- Added `extract_list` variants to extract a `Value::List` into a
homogenous list if possible.
- Added an `Unpack<T>` implementation for `Option<V: Unpack<T>>` on 
a `T: Read + Seek`. This is sufficient for `BufReader` usage at the 
moment.
- Added support for `enum` types in the derive-macros of `Pack` and `Unpack`.
The macro considers these types as "StructSums".

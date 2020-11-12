# Version 0.2.0

### Breaking Changes:
- Moved the `T: Read` and `T: Write` from `Unpack<T>` and `Pack<T>` to their
corresponding functions, making the type parameter function local.
- Removed `add_property` from `Node`.
- Removed `PackableStructSum` and `PackableStruct`. Everything is now just `Pack`
and `Unpack`.
- Removed the default implementations for `Pack` and `Unpack` for all
`PackableStructSum`. Use the derive macros for `Pack` and `Unpack` now.
- Made the `#[tag = u8]` attribute mandatory (from optional) for sums as well
as for structs when deriving `Pack` and/or `Unpack`. 
- The trait `Unpack` has now as a non-optional function `decode_body` from which
`decode` is defined.

### Additions:
- `Marker::Structure` has now `tag_byte` as a part of it, making `Marker`
in this case two bytes long. Its `decode` function takes care of it and
reads out a `tag_byte`; the `encode` function writes the `Structure` marker
as well as the `tag_byte`. 
- Removed the dependency `byteorders`.
- New `NoStruct` type to deny any struct sums on `Value<NoStruct>`.
- Added support in `DecodeError` for `NoStruct`.
- Added `Dictionary<T>` which replaces `HashMap<String, Value<T>>` in the
standard structs. 
- Added `ExtractRef` trait which denotes types into which a `Value`
can be extracted to, as well as its variants `Extract` and `ExtractMut`.
- Added `extract_list` variants to extract a `Value::List` into a
homogenous list if possible.
- Added an `Unpack<T>` implementation for `Option<V: Unpack<T>>`.
- Added support for `enum` types in the derive-macros of `Pack` and `Unpack`.
The macro considers these types as "Struct Sums".
- Allowed the deriving of `Pack` and `Unpack` for `struct` not only as a 
Structure but as a flat Dictionary as well, using the `#[dict]` attribute.
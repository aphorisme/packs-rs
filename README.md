# packs
[![Crates.io][crates-badge]][crates-url]
[![Docs.io][docs-badge]][docs-url]


[docs-badge]: https://docs.rs/packs/badge.svg
[docs-url]: https://docs.rs/packs

[crates-badge]: https://img.shields.io/crates/v/packs.svg
[crates-url]: https://crates.io/crates/packs
A [PackStream](https://7687.org/packstream/packstream-specification-1.html) implementation written in Rust 🦀.

| PackStream Version | Supported  |
| :-----------:  |  :------------:   |
|    1           |   *yes*               |


⚠️ This package has yet to proof to be largely bug free.

⚠️ Be aware this package is part of a bigger project which might
dictate another architecture and hence imply *heavy changes*.

It will stay open source nevertheless and will continue to follow its
philosophy, by providing a PackStream implementation which is decoupled
from the bolt protocol as well as from the standard structs (e.g. `Node`).

This package is used as the PackStream implementation for the bolt driver 
[`raio-rs`](https://github.com/aphorisme/raio-rs).

## Overview
PackStream is a streamable binary format for a range of data types,
used by the [bolt protocol](https://7687.org/#bolt) – the protocol of
the graph database neo4j. 

The protocol can be used for any byte wise packing; it is
especially suitable for streams. This library keeps this open and is
not tied to bolt nor neo4j specific implementation details.

### Primitive Types

Currently, the following primitive types are supported:

| type  | rust variant |
| :--- |  :--------  |
| boolean | `bool` |
| string | `String` |
| integer | `i64`, `i32` |
| float | `f64`, *encode only*: `f32` |
| list | `Vec<T>` |
| dictionary | `HashMap<String, T>` |
| byte array | *wrapped* `Vec<u8>` |
| null | `Option<T>` |

All of them are also part of a light typed variant `Value` which 
allows for decoding of a value which type is unknown. 

### Structs

Besides, these primitive types, PackStream supports structs with
up to 15 fields.

Although the specification defines some structs already (e.g. `DateTime`
and `Node`) this library has these structs as opt-out. One can either
use the standard structs as they are specified in the official specification,
extend them, completely ignore them, or provide one's own. 

The library also provides a derive-macro to derive the encoding and decoding
traits (`Pack` and `Unpack`) for structs and enums.

Those different options can be controlled through feature flags.

| feature flag | mode | cargo settings |
| :---- | :------ | :---- | 
| *default* | derive macros, standard structs are included | 
| std_structs | same as default | `default-features = false`, `features = ["std_structs"]`
| derive | only derive macros, no standard structs | `default-features = false`, `features = ["derive"]`
|  | no derive macros, no standard structs | `default-features = false`

The tests cover the standard structs; they are seen as fully supported
by the library and come with utility functions.

## Derive `Pack`/`Unpack`

Structs and enums can use the derive-macros for `Pack` and `Unpack`. They will then get encoded
and decoded as if they were PackStream structures like `Node` or `Relationship`. 

### Deriving on `struct`

Since every PackStream structure has its own tag, one needs to provide a 
tag by using the `#[tag = u8]` attribute. 

For example, the `Node` structure uses the macro:

```rust
#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
#[tag = 0x4E]
pub struct Node {
    pub id: i64,
    pub labels: HashSet<String>,
    pub properties: Dictionary<StdStructPrimitive>
}
```

where each field implements `Pack`/`Unpack`.

### Deriving on `enum`

Enums are considered as sum types of structures, hence the different variants
need `#[tag = u8]` attributes. For example, the enum containing all standard
structures of the PackStream protocol: 

```rust
#[derive(Debug, Clone, PartialEq, Pack, Unpack)]
pub enum StdStruct {
    #[tag = 0x4E]
    Node(Node),
    #[tag = 0x52]
    Relationship(Relationship),
    #[tag = 0x72]
    UnboundRelationship(UnboundRelationship),
    #[tag = 0x50]
    Path(Path),
    #[tag = 0x44]
    Date(Date),
    /* ... snip .. */
}
```

Each variant has one field which has to implement `Pack`/`Unpack`.

### Deriving with custom attributes

Sometimes fields of a `struct` should not implement `Pack`/`Unpack` but provide
a custom function to encode/decode. `Pack` and `Unpack` are meant to provide
functions which turn values into valid PackStream objects. But sometimes fields
do not provide meaningful headers are markers and are rather a part of Structure
(in the sense of PackStream) then values of themselves. 

For this scenario, one can provide an encode function
```rust
fn pack_field<T: Write>(&Field, writer: &mut T) -> Result<usize, EncodeError>
```
returning the number of bytes written or an `EncodeError`, and a decode function
```rust
fn unpack_field<T: Read>(reader: &mut T) -> Result<Field, DecodeError>
```
which returns the field read from the `reader` or a `DecodeError`.

Given such functions, one can use the `#[pack(pack_field)]` and `#[unpack(unpack_field)]`
attributes on fields of a `struct`. There is also a `#[fields = usize]` attribute
to state for how many fields the structure counts: 

```rust
use std::io::{Write, Read};
use packs::*;

#[derive(Debug, PartialEq)]
struct Part {
    field_a: String,
    field_b: Option<String>,
}

fn pack_part<T: Write>(part: &Part, writer: &mut T) -> Result<usize, EncodeError> {
    Ok(&part.field_a.encode(writer)? + &part.field_b.encode(writer)?)
}

fn unpack_part<T: Read>(reader: &mut T) -> Result<Part, DecodeError> {
    let field_a = String::decode(reader)?;
    let field_b = <Option<String>>::decode(reader)?;

    Ok(
        Part {
            field_a,
            field_b,
        }
    )
}

#[derive(Debug, PartialEq, Unpack, Pack)]
#[tag = 0x01]
struct Complete {
    #[pack(pack_part)]
    #[unpack(unpack_part)]
    #[fields = 2]
    part: Part,
    id: i64,
}
```

## Contribute

You are welcome to contribute! Especially utility functions for 
standard struct types, documentation, optimization, and heavier testing. I try 
to keep any open tasks as issues in github for you to pick. And of course,
there will be bugs.

I'm not into changing architecture for now, as this library will be used 
in a bigger project. I first have to see how it fits in; I'm open
for suggestions though.

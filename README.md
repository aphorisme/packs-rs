# packs
A [PackStream](https://7687.org/packstream/packstream-specification-1.html) implementation written in Rust 🦀.

| PackStream Version | Supported Version |
| :-----------:  |  :------------:   |
|    1           |   1               |

**State**: This package is on its way to version `0.1`. For it to reach `0.1`
it misses
- More test coverage
- Better documentation
- A few ergonomic functions for its structs

⚠️ Be aware that this package is part of a bigger project which might
dictate another architecture and hence imply *heavy changes*.

It will stay open source nevertheless and will continue to follow its
philosophy, by providing a PackStream implementation which is decoupled
from bolt and neo4j as well as from the standard structs (e.g. `Node`).

This package is used as the PackStream implementation for the bolt driver 
[`raio-rs`](https://github.com/aphorisme/raio-rs).

## Overview
PackStream is a streamable binary format for a range of data types,
used by the [bolt protocol](https://7687.org/#bolt) – the protocol of
the graph database neo4j. 

But the protocol can be used for any byte wise packing; it is
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
| null | *encode only*: `Option<T>` |

all of them are also part of a light typed variant `Value` which 
allows for decoding of a value which type is unknown. 

### Structs

Besides these primitive types, PackStream supports structs with
up to 15 fields.

Although the specification defines some structs already (e.g. `DateTime`
and `Node`) this library has these structs as opt-out. One can either
use the standard structs as they are specified in the official specification,
extend them, completely ignore them, or provide one's own. 

The library also provides a derive macro to derive the encoding and decoding
traits (`Pack` and `Unpack`) for structs, as well as a macro to define
a sum type to extend `Value`.

Those different options can be controlled through feature flags.

| feature flag | mode | cargo settings |
| :---- | :------ | :---- | 
| *default* | derive macros, standard structs are included | 
| std_structs | same as default | `default-features = false`, `features = ["std_structs"]`
| derive | only derive macros, no standard structs | `default-features = false`, `features = ["derive"]`
|  | no derive macros, no standard structs | `default-features = false`

The tests cover the standard structs; they are seen as fully supported
by the library and come with utility functions.

## Contribute

There are still a few things to do. You are welcome to contribute, of 
course! Especially utility functions for standard struct types, 
documentation and heavier test. I try to keep any open tasks as 
issues in github for you to pick.

I'm not into changing architecture for now, as this library will be used 
in a bigger project and I first have to see how it fits in; I'm open
for suggestions though.

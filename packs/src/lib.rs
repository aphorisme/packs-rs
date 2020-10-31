//! A [PackStream](https://7687.org/packstream/packstream-specification-1.html) implementation written
//! in Rust.
//!
//! # API
//! The trait [`Pack`](crate::packable::Pack) is for encoding, the trait [`Unpack`](crate::packable::Unpack)
//! is for decoding. They abstracted over [`Write`](std::io::Write) and [`Read`](std::io::Read) respectively.
//!
//! The traits are implemented for some basic types as well as for a standard set of structs which come
//! with the PackStream specification, see the [`std_structs`](crate::std_structs) module.
//! ```
//! use packs::{Pack, Unpack};
//! use packs::std_structs::Node;
//!
//! let mut node = Node::new(42);
//! node.properties.add_property("title", "A Book's Title");
//! node.properties.add_property("pages", 302);
//!
//! // encode `node` into a `Vec<u8>`:
//! let mut buffer = Vec::new();
//! node.encode(&mut buffer).unwrap();
//!
//! // and recover it from these bytes:
//! let recovered = Node::decode(&mut buffer.as_slice()).unwrap();
//!
//! assert_eq!(node, recovered);
//! ```
//! # User-Defined Structs
//! Using the derive macros for [`PackableStruct`](crate::structure::packable_struct), `Pack` and
//! `Unpack` as well as providing a tag byte, user defined structure can be encoded and decoded
//! as well. These are then treated as if they were part of the PackStream specification, i.e. like
//! if they were a `Point2D` or `Node` or such. This especially means that they are not packed as `Node`,
//! but as their own.
//! ```
//! use packs::*;
//!
//! #[derive(Debug, PartialEq, PackableStruct, Pack, Unpack)]
//! #[tag = 0x0B]
//! struct Book {
//!     pub title: String,
//!     pub pages: i64,
//! }
//!
//! // this is not packed as a `Node`. It is a genuinely user defined struct,
//! // it will differ in its byte structure to the `Node` above.
//! let book = Book { title: String::from("A Book's title"), pages: 302 };
//!
//! let mut buffer = Vec::new();
//! book.encode(&mut buffer).unwrap();
//!
//! let recovered = Book::decode(&mut buffer.as_slice()).unwrap();
//!
//! assert_eq!(book, recovered);
//! ```
//! ## Providing a sum type
//! User defined structs are often sumed up in an `enum` which denotes all possible structs
//! the protocol should be able to encode and decode. This can be given by deriving `PackableStructSum`.
//! The `tag` attribute on the different variants is not optional, but it can differ from the one `tag`
//! attribute provided to the structs themselves.
//! ```
//! use packs::*;
//!
//! #[derive(Debug, PartialEq, PackableStruct, Pack, Unpack)]
//! #[tag = 0x0B]
//! struct Book {
//!     pub title: String,
//!     pub pages: i64,
//! }
//!
//! #[derive(Debug, PartialEq, PackableStruct, Pack, Unpack)]
//! #[tag = 0x0C]
//! struct Person {
//!     pub name: String,
//! }
//!
//! #[derive(Debug, PartialEq, PackableStructSum, Pack, Unpack)]
//! enum MyStruct {
//!     #[tag = 0x0B] // can be a different tag, but same here for consistency
//!     Book(Book),
//!     #[tag = 0x0C]
//!     Person(Person),
//! }
//!
//! let person = Person { name: String::from("Check Mate") };
//!
//! let mut buffer = Vec::new();
//! person.encode(&mut buffer).unwrap();
//!
//! // recover via `MyStruct`:
//! let my_struct = MyStruct::decode(&mut buffer.as_slice()).unwrap();
//!
//! assert_eq!(MyStruct::Person(person), my_struct);
//! ```
//! ## Consistency on tag bytes
//! The tags provided by the types themselves and defined as part of the variant can be different,
//! which might be useful in some situations (for example to re-use a structure in a different setting),
//! but then decoding has to go the same way (sum-type or direct) as it was encoded to yield a valid
//! result.
//!
//! # Runtime-typed values
//! Besides using the types directly, values can be encoded and decoded through a sum type
//! [`Value`](crate::value::Value) which allows for decoding of any value without knowing its type
//! beforehand.
//! ```
//! use packs::{Value, Unpack, Pack, NoStruct};
//! use packs::std_structs::StdStruct;
//!
//! let mut buffer = Vec::new();
//! 42i64.encode(&mut buffer).unwrap();
//!
//! let value = <Value<NoStruct>>::decode(&mut buffer.as_slice()).unwrap();
//!
//! assert_eq!(Value::Integer(42), value);
//! ```
//! The type `Value` is abstracted over possible structures. One can use `NoStruct` to deny any
//! structures or use `Value<StdStruct>` (c.f. [`StdStruct`](crate::std_structs::StdStruct))
//! to allow any standard structures as part of `Value`.
//!
//! To continue on the example from above, `Value<MyStruct>` could have been used there as well:
//! ```
//! # use packs::*;
//! # #[derive(Debug, PartialEq, PackableStruct, Pack, Unpack)]
//! # #[tag = 0x0B]
//! # struct Book {
//! #     pub title: String,
//! #     pub pages: i64,
//! # }
//! # #[derive(Debug, PartialEq, PackableStruct, Pack, Unpack)]
//! # #[tag = 0x0C]
//! # struct Person {
//! #     pub name: String,
//! # }
//! # #[derive(Debug, PartialEq, PackableStructSum, Pack, Unpack)]
//! # enum MyStruct {
//! #    #[tag = 0x0B] // can be a different tag, but same here for consistency
//! #    Book(Book),
//! #    #[tag = 0x0C]
//! #    Person(Person),
//! # }
//! let mut buffer = Vec::new();
//! let person = Person { name: String::from("Check Mate") };
//! person
//!     .encode(&mut buffer)
//!     .unwrap();
//!
//! let runtime_typed = <Value<MyStruct>>::decode(&mut buffer.as_slice()).unwrap();
//!
//! assert_eq!(Value::Structure(MyStruct::Person(person)), runtime_typed);
//! ```
pub mod value;
pub mod structure;
pub mod packable;
pub mod error;
pub mod ll;
pub mod utils;

#[cfg(feature = "std_structs")]
pub mod std_structs;

#[cfg(feature = "derive")]
pub use packs_proc::*;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use std::io::Write;

#[cfg(feature = "derive")]
#[doc(hidden)]
pub use std::io::Read;

// Public API:
pub use packable::{Pack, Unpack};
pub use error::{EncodeError, DecodeError};
pub use value::{Value, Extract, ExtractRef, ExtractMut};
pub use value::bytes::Bytes;
pub use value::dictionary::Dictionary;
pub use ll::marker::Marker;
pub use structure::packable_struct::{PackableStruct};
pub use structure::{encode_struct, decode_struct};
pub use structure::generic_struct::GenericStruct;
pub use structure::struct_sum::{PackableStructSum, NoStruct};
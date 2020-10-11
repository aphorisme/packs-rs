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
pub use value::Value;
pub use value::bytes::Bytes;
pub use structure::packable_struct::{PackableStruct};
pub use structure::{encode_struct, decode_struct};
pub use structure::generic_struct::GenericStruct;
pub use structure::struct_sum::PackableStructSum;
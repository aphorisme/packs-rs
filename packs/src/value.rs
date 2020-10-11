use std::collections::HashMap;
use std::fmt::Debug;
use crate::value::bytes::Bytes;

pub mod bytes;


#[derive(Debug, Clone, PartialEq)]
/// A type for all possible values which can be serialized through PackStream.
/// This type abstracts over structure types, which allows the user to define
/// their own structures which should be part of `Value`. There are two standard
/// implementations, either `Value<()>` to denote a value where only the unit is
/// allowed as a structure, or `Value<GenericStruct>` which reads any valid structure
/// in a generic way, see [`GenericStruct`](crate::value::generic_struct::GenericStruct).
pub enum Value<S> {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Bytes(Bytes),
    String(String),
    List(Vec<Value<S>>),
    Dictionary(HashMap<String, Value<S>>),
    Structure(S)
}

impl<S> From<i64> for Value<S> {
    fn from(i: i64) -> Self {
        Value::Integer(i)
    }
}

impl<S> From<bool> for Value<S> {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl<S> From<f64> for Value<S> {
    fn from(f: f64) -> Self {
        Value::Float(f)
    }
}

impl<S> From<&str> for Value<S> {
    fn from(s: &str) -> Self {
        Value::String(String::from(s))
    }
}

impl<S> From<String> for Value<S> {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

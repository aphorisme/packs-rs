use std::fmt::Debug;
use crate::value::bytes::Bytes;
use crate::value::dictionary::Dictionary;
use std::iter::FromIterator;

pub mod bytes;
pub mod dictionary;


#[derive(Debug, Clone, PartialEq)]
/// A type for all possible values which can be serialized through PackStream.
/// This type abstracts over structure types, which allows the user to define
/// their own structures which should be part of `Value`. There are two standard
/// implementations, either `Value<NoStruct>` to denote a value where nothing further
/// allowed as a structure, or `Value<GenericStruct>` which reads any valid structure
/// generic, see [`GenericStruct`](crate::structure::generic_struct::GenericStruct).
///
/// In conjunction with `std_structs` one can use `Value<StdStruct>` to use the default
/// structs as given by the PackStream specification.
///
/// ## Creating a Value
/// For once, a `Value` is just a sum type, so by choosing an appropiated case, one can construct
/// a `Value` directly, e.g. `Value::Integer(3)`. But besides, there are either possibilities to use
/// the `From` implementation for some types or collecting items into a `Value::List` or `Value::Dictionary`:
/// ```
/// use packs::*;
///
/// // collect a list of values which can be converted to values
/// // into `Value::List`:
/// let value : Value<()> =
///     vec!(42i64, 0, 1).into_iter().collect();
///
/// assert_eq!(
///     value,
///     Value::List(
///         vec!(Value::Integer(42),
///              Value::Integer(0),
///              Value::Integer(1))));
/// ```
/// This is also possible with pairs which are then get collected into `Value::Dictionary`:
/// ```
/// use packs::*;
///
/// // collect into a dictionary:
/// let value : Value<()> =
///     vec!(
///         (String::from("name"), <&str as Into<Value<()>>>::into("Jane Doe")),
///         (String::from("age"), 42.into()),
///    ).into_iter().collect();
///
/// assert_eq!(
///     value,
///     Value::Dictionary(
///         vec!(
///             (String::from("name"), String::from("Jane Doe").into()),
///             (String::from("age"), 42.into()),
///         ).into_iter().collect()
///     ));
/// ```
///
/// ## Extracting
/// A `Value` can be extracted into a supported type by using the [`Extract`](crate::value::Extract)
/// traits (there are variants for borrowed and mutably borrowed values). This checks on the value
/// and tries to recover the inner type from it:
/// ```
/// use packs::*;
///
/// let value : Value<()> = Value::Float(42.42);
/// assert_eq!(f64::extract(value).unwrap(), 42.42);
/// ```
///
/// ## The Null
/// PackStream has `Value::Null` as a possible value, denoting the absence of a value. From
/// within Rust, this transforms any `Value -> T` into a `Value -> Option<T>`, returning `None`
/// on `Value::Null`; but since this is only one way to treat `Value::Null` this package does not
/// decide but provides different ways to extract values:
///
/// 1. The trait `Extract<T>` has an implementation for `Option<E: Extract<T>>` to treat `Value::Null` as `None`.
/// 2. The traits `ExtractMut<T>` and `ExtractRef<T>` provide functions with default implementations to extract
/// any `Value::Null` as a `None` and treat every other `v` as `Some(v)`.
/// 3. Otherwise, any extract of `Value::Null` will fail with `None`.
/// ```
/// use packs::*;
///
/// // case 1:
/// assert_eq!(<Option<bool>>::extract(<Value<NoStruct>>::Boolean(true)), Some(Some(true)));
/// assert_eq!(<Option<bool>>::extract(<Value<NoStruct>>::Null), Some(None));
///
/// // case 2:
/// assert_eq!(<bool>::extract_opt_ref(&<Value<NoStruct>>::Boolean(true)), Some(Some(&true)));
/// assert_eq!(<bool>::extract_opt_ref(&<Value<NoStruct>>::Null), Some(None));
///
/// // case 3:
/// assert_eq!(<bool>::extract_ref(&<Value<NoStruct>>::Null), None)
/// ```
pub enum Value<S> {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    Bytes(Bytes),
    String(String),
    List(Vec<Value<S>>),
    Dictionary(Dictionary<S>),
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

impl<S> From<Vec<Value<S>>> for Value<S> {
    fn from(s: Vec<Value<S>>) -> Self {
        Value::List(s)
    }
}

impl<S, V: Into<Value<S>>> From<Option<V>> for Value<S> {
    fn from(opt: Option<V>) -> Self {
        if let Some(v) = opt {
            v.into()
        } else {
            Value::Null
        }
    }
}

impl<S, V: Into<Value<S>>> FromIterator<V> for Value<S> {
    fn from_iter<T: IntoIterator<Item=V>>(iter: T) -> Self {
        Value::List(iter.into_iter().map(|v| v.into()).collect())
    }
}

impl<S, V: Into<Value<S>>> FromIterator<(String, V)> for Value<S> {
    fn from_iter<T: IntoIterator<Item=(String, V)>>(iter: T) -> Self {
        Value::Dictionary(
            iter.into_iter().map(|(key, val)| (key, val.into())).collect()
        )
    }
}

//--------------------------------------//
//       Extract trait and impls        //
//--------------------------------------//
/// A trait to denote types which can be extracted out of a `Value<T>`.
/// ```
/// # use packs::{Value, Extract};
/// let value : Value<()> = Value::Integer(42);
/// assert_eq!(i64::extract(value), Some(42))
/// ```
/// For each accessibility kind there is an `Option` variant, treating a `Value` as a `Option<T>`, where
/// `Value::Null` denotes `None` and every other value is tried to get parsed into `Some(t)`:
/// ```
/// # use packs::{Value, Extract};
///
/// // any `Value::Null` is a `None`:
/// let value : Value<()> = Value::Null;
/// assert_eq!(<Option<String>>::extract(value), Some(None));
///
/// let value : Value<()> = Value::String(String::from("Hello World!"));
/// assert_eq!(<Option<String>>::extract(value), Some(Some(String::from("Hello World!"))));
/// ```
pub trait Extract<T> : Sized {
    fn extract(from: Value<T>) -> Option<Self>;
}

/// An `Extract` variant for borrowed values.
pub trait ExtractRef<T> : Sized {
    fn extract_ref(from: &Value<T>) -> Option<&Self>;
    fn extract_opt_ref(from: &Value<T>) -> Option<Option<&Self>> {
        match from {
            Value::Null => Some(None),
            v => Self::extract_ref(v).map(Some)
        }
    }
}

/// An `Extract` variant for mutably borrowed values.
pub trait ExtractMut<T> : Sized {
    fn extract_mut(from: &mut Value<T>) -> Option<&mut Self>;
    fn extract_opt_mut(from: &mut Value<T>) -> Option<Option<&mut Self>> {
        match from {
            Value::Null => Some(None),
            v => Self::extract_mut(v).map(Some)
        }
    }
}

macro_rules! impl_extract {
    ($ty_for:ty, $into:ident) => {
        impl<T> Extract<T> for $ty_for {
            fn extract(from: Value<T>) -> Option<Self> {
                match from {
                    Value::$into(x) => Some(x),
                    _ => None,
                }
            }
        }

        impl<T> ExtractRef<T> for $ty_for {
            fn extract_ref(from: &Value<T>) -> Option<&Self> {
                match from {
                    Value::$into(x) => Some(x),
                    _ => None,
                }
            }
        }
        impl<T> ExtractMut<T> for $ty_for {
            fn extract_mut(from: &mut Value<T>) -> Option<&mut Self> {
                match from {
                    Value::$into(x) => Some(x),
                    _ => None,
                }
            }
        }
    }
}

impl_extract!(i64, Integer);
impl_extract!(f64, Float);
impl_extract!(bool, Boolean);
impl_extract!(Bytes, Bytes);
impl_extract!(String, String);
impl_extract!(Vec<Value<T>>, List);
impl_extract!(Dictionary<T>, Dictionary);

impl<T, E: Extract<T>> Extract<T> for Option<E> {
    fn extract(from: Value<T>) -> Option<Self> {
        match from {
            Value::Null => Some(None),
            v => E::extract(v).map(Some)
        }
    }
}

/// Extracts a `Value::List` with the same runtime type values into a vector of extracted values.
/// ```
/// # use packs::{NoStruct, Value, extract_list_ref};
/// let value : Value<NoStruct> = vec!(42, -1, 3332).into_iter().collect();
///
/// let ints = extract_list_ref::<NoStruct, i64>(&value);
/// assert_eq!(Some(vec!(&42, &-1, &3332)), ints);
/// ```
/// Does return `None` whenever either the provided `value` is not a `List` or any of the items of
/// the list cannot be extracted to `T`.
/// ```
/// # use packs::{Value, NoStruct, extract_list_ref};
/// let value : Value<NoStruct> =
///     vec!(
///         Value::Integer(42),
///         Value::Boolean(false))
///     .into_iter().collect();
///
/// let extract = extract_list_ref::<NoStruct, i64>(&value);
/// assert_eq!(None, extract);
/// ```
pub fn extract_list_ref<S, T: ExtractRef<S>>(value: &Value<S>) -> Option<Vec<&T>> {
    match value {
        Value::List(vs) => {
            let extracted : Vec<&T> = vs.iter().flat_map(T::extract_ref).collect();
            if extracted.len() < vs.len() {
                None
            } else {
                Some(extracted)
            }
        },
        _ => None,
    }
}

/// A variant of [`extract_list_ref`](crate::value::extract_list_ref) with a moved `Value`.
pub fn extract_list<S, T: Extract<S>>(value: Value<S>) -> Option<Vec<T>> {
    match value {
        Value::List(vs) => {
            let len = vs.len();
            let extracted : Vec<T> = vs.into_iter().flat_map(T::extract).collect();
            if extracted.len() < len {
                None
            } else {
                Some(extracted)
            }
        },
        _ => None,
    }
}


/// A variant of [`extract_list_ref`](crate::value::extract_list_ref) with a mutable borrow.
pub fn extract_list_mut<S, T: ExtractMut<S>>(value: &mut Value<S>) -> Option<Vec<&mut T>> {
    match value {
        Value::List(vs) => {
            let len = vs.len();
            let extracted : Vec<&mut T> = vs.iter_mut().flat_map(T::extract_mut).collect();
            if extracted.len() < len {
                None
            } else {
                Some(extracted)
            }
        },
        _ => None,
    }
}
use std::collections::HashMap;
use crate::{Value, EncodeError, PackableStructSum, Pack, DecodeError, Unpack, Extract};
use std::io::{Write, Read};
use crate::ll::types::sized::{SizedTypePack, SizedTypeUnpack, SizedType};
use crate::ll::marker::Marker;
use crate::ll::types::lengths::{read_dict_size, Length};
use std::collections::hash_map::{Iter, IterMut};
use std::iter::FromIterator;
use crate::value::ExtractRef;

#[derive(Debug, Clone, PartialEq)]
/// A `Dictionary` is a map of `String` to [`Value<T>`](crate::value::Value) pairs. These pairs are
/// also called `properties` and can be seen as named values. The type parameter denotes the allowed
/// structures for a value.
pub struct Dictionary<T>(HashMap<String, Value<T>>);

impl<T> Dictionary<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Dictionary(HashMap::with_capacity(capacity))
    }

    pub fn new() -> Self {
        Dictionary(HashMap::new())
    }

    /// Adds a key-value pair to the `Dictionary`. Returns the original value of the property
    /// if it was already set.
    pub fn add_property<V: Into<Value<T>>>(&mut self, key: &str, value: V) -> Option<Value<T>> {
        self.0.insert(String::from(key), value.into())
    }

    pub fn has_property(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn extract_property(&mut self, key: &str) -> Option<Value<T>> {
        self.0.remove(key)
    }

    /// Removes the property from the dictionary and returns it. Tries to extract the value
    /// strongly typed.
    ///
    /// **Panics** if it cannot cast the value to provided type.
    pub fn extract_property_typed<V: Extract<T>>(&mut self, key: &str) -> Option<V> {
        self.0.remove(key).map(|v| V::extract(v).unwrap())
    }

    /// Retrieves a property.
    pub fn get_property(&self, key: &str) -> Option<&Value<T>> {
        self.0.get(key)
    }

    /// Retrieves the value of a property in a strongly typed manner.
    ///
    /// **Panics** if it cannot cast the value to provided type.
    /// ```
    /// # use packs::*;
    /// // create a Dictionary with a list of values as property:
    /// let mut dict = Dictionary::with_capacity(1);
    /// let list: Vec<Value<()>> = vec!(Value::Integer(1), Value::Boolean(true));
    /// dict.add_property("foo", list.clone());
    ///
    /// // retrieve this list:
    /// let get_list = dict.get_property_typed("foo");
    /// assert_eq!(Some(&list), get_list);
    /// ```
    pub fn get_property_typed<V: ExtractRef<T>>(&self, key: &str) -> Option<&V> {
        self.0.get(key).map(|v| V::extract_ref(v).unwrap())
    }

    pub fn properties(&self) -> Iter<String, Value<T>> {
        self.0.iter()
    }

    pub fn properties_mut(&mut self) -> IterMut<String, Value<T>> {
        self.0.iter_mut()
    }
}

impl<T> FromIterator<(String, Value<T>)> for Dictionary<T> {
    fn from_iter<I: IntoIterator<Item=(String, Value<T>)>>(iter: I) -> Self {
        let data: HashMap<String, Value<T>> = iter.into_iter().collect();
        Dictionary(data)
    }
}


impl<T: Write, S: PackableStructSum> SizedTypePack<T> for Dictionary<S> {
    fn write_body(&self, writer: &mut T) -> Result<usize, EncodeError> {
        let mut written = 0;
        for (key, val) in &self.0 {
            written += key.encode(writer)?;
            written += val.encode(writer)?;
        }
        Ok(written)
    }
}

impl<T: Read, S: PackableStructSum> SizedTypeUnpack<T> for Dictionary<S> {
    fn read_length(marker: Marker, reader: &mut T) -> Result<usize, DecodeError> {
        read_dict_size(marker, reader)
    }

    fn read_body(len: usize, reader: &mut T) -> Result<Self, DecodeError> {
        let mut data = HashMap::with_capacity(len);
        for _ in 0..len {
            let key = String::decode(reader)?;
            let val = <Value<S>>::decode(reader)?;
            data.insert(key, val);
        }
        Ok(Dictionary(data))
    }
}

impl<S> SizedType for Dictionary<S> {
    fn header(&self) -> (Marker, Length) {
        let len =
            Length::from_usize(self.0.len())
                .expect(&format!("Dictionary has invalid size '{}'", self.0.len()));

        (len.marker(
            Marker::TinyDictionary,
            Marker::Dictionary8,
            Marker::Dictionary16,
            Marker::Dictionary32
        ), len)
    }
}

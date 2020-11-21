use std::collections::HashMap;
use crate::{Value, Extract};
use std::collections::hash_map::{Iter, IterMut, Entry};
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

    pub fn from_inner(map: HashMap<String, Value<T>>) -> Self {
        Dictionary(map)
    }

    pub fn into_inner(self) -> HashMap<String, Value<T>> {
        self.0
    }

    pub fn inner(&self) -> &HashMap<String, Value<T>> {
        &self.0
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

    pub fn entry(&mut self, key: String) -> Entry<String, Value<T>> {
        self.0.entry(key)
    }
    
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> FromIterator<(String, Value<T>)> for Dictionary<T> {
    fn from_iter<I: IntoIterator<Item=(String, Value<T>)>>(iter: I) -> Self {
        let data: HashMap<String, Value<T>> = iter.into_iter().collect();
        Dictionary(data)
    }
}


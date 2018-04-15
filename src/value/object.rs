/*
 * Dermis is an interpreter for a pure, statically typed, imperitive language designed to be edited with a custom IDE.
 * Copyright (C) 2018 Amelorate
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use im::HashMap;
use im::hashmap::{Keys, Values};
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use value::Value;

/// Returns an empty object.
///
/// In dermis, a null value is represented with an empty object. This function will get you a null
/// object in a optimal way.
///
/// # Example
/// ```
/// use dermis::value::{Value, get_null};
/// let null = get_null();
///
/// match &*null {
///     Value::Object(ref a) => assert_eq!(a.len(), 0),
///     _ => unreachable!(),
/// }
///  ```
pub fn get_null() -> Arc<Value> {
    Arc::new(Value::Object(Object::empty()))
}

/// Like a Javascript Object, is a mapping between a key and a value, where both are of any
/// type.
///
/// For documentation about `Object`'s member functions, see [`im::HashMap`](HashMap).
///
/// # Example
/// ```
/// use dermis::value::{Object, Value, get_null};
///
/// let mut obj: Object = Object::empty();
///
/// // See Value::Symbol for a good type for a key.
/// // This example does not use Symbol because it requires initializing an interpreter.
/// obj.set_mut(Value::String("number".to_string()), Value::Number(12.0.into()));
/// obj.set_mut("string".into(), Value::String("Hello!".to_string()));
///
/// let obj_different = obj.set("number_2".into(), 2.0.into());
/// // ^ Avoids cloning obj, because it uses Arc<Value>'s for storage.
///
/// // In the below assertions the value of obj.get is derefrenced because it returns an
/// // Arc<Value>. In some places you may need to use &* to make sure you aren't moving the value.
/// assert_eq!(*obj.get(&"number".into()), 12.0.into());
/// assert_eq!(*obj.get(&"string".into()), "Hello!".into());
///
/// assert_eq!(obj.get(&"number_2".into()), get_null());
/// assert_eq!(*obj_different.get(&"number_2".into()), Value::from(2.0));
/// ```
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, Default, From, Into)]
pub struct Object(pub HashMap<Value, Value>);

impl From<BTreeMap<Value, Value>> for Object {
    fn from(val: BTreeMap<Value, Value>) -> Object {
        HashMap::from(val).into()
    }
}

impl From<Object> for BTreeMap<Value, Value> {
    fn from(val: Object) -> BTreeMap<Value, Value> {
        val.0
            .into_iter()
            .map(|(k, v)| {
                (
                    Arc::try_unwrap(k).unwrap_or_else(|e| (&*e).clone()),
                    Arc::try_unwrap(v).unwrap_or_else(|e| (&*e).clone()),
                )
            })
            .collect()
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for (k, v) in &self.0 {
            if !first {
                write!(f, ", ")?;
            } else {
                first = false;
            }
            write!(f, "{}: {}", k, v)?;
        }
        write!(f, "}}")
    }
}

impl Object {
    /// Returns the value with the given key. If nothing is found, an empty Value::Object will be
    /// returned.
    ///
    /// # Example
    /// ```
    /// use dermis::value::{Value, Object, get_null};
    ///
    /// let mut obj = Object::default();
    ///
    /// obj.set_mut("bar".into(), 12.0.into());
    ///
    /// let a = obj.get(&"foo".into());
    /// let b = obj.get(&"bar".into());
    ///
    /// assert_eq!(a, get_null());
    /// assert_eq!(&*b, &Value::from(12.0));
    /// ```
    pub fn get(&self, key: &Value) -> Arc<Value> {
        self.0.get(key).unwrap_or_else(|| get_null())
    }

    /// See [`im::HashMap::get`](HashMap::get)
    pub fn get_opt(&self, k: &Value) -> Option<Arc<Value>> {
        self.0.get(k)
    }

    pub fn empty() -> Self {
        Object::default()
    }
}

impl Object {
    pub fn singleton(k: Value, v: Value) -> Self {
        Object(HashMap::singleton(k, v))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn keys(&self) -> Keys<Value, Value> {
        self.0.keys()
    }

    pub fn values(&self) -> Values<Value, Value> {
        self.0.values()
    }

    pub fn get_or(&self, k: &Value, default: Value) -> Arc<Value> {
        self.0.get_or(k, default)
    }

    pub fn contains_key(&self, k: &Value) -> bool {
        self.0.contains_key(k)
    }

    pub fn insert(&self, k: Value, v: Value) -> Self {
        Object(self.0.insert(k, v))
    }

    pub fn insert_mut(&mut self, k: Value, v: Value) {
        self.0.insert_mut(k, v)
    }

    pub fn set(&self, k: Value, v: Value) -> Self {
        Object(self.0.set(k, v))
    }

    pub fn set_mut(&mut self, k: Value, v: Value) {
        self.0.set_mut(k, v)
    }

    pub fn insert_with<F>(self, k: Value, v: Value, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>) -> Arc<Value>,
    {
        Object(self.0.insert_with(k, v, f))
    }

    pub fn insert_with_key<F>(self, k: Value, v: Value, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>, Arc<Value>) -> Arc<Value>,
    {
        Object(self.0.insert_with_key(k, v, f))
    }

    pub fn insert_lookup_with_key<F>(self, k: Value, v: Value, f: F) -> (Option<Arc<Value>>, Self)
    where
        F: Fn(Arc<Value>, Arc<Value>, Arc<Value>) -> Arc<Value>,
    {
        let (l, r) = self.0.insert_lookup_with_key(k, v, f);
        (l, Object(r))
    }

    pub fn update<F>(&self, k: &Value, f: F) -> Self
    where
        F: Fn(Arc<Value>) -> Option<Arc<Value>>,
    {
        Object(self.0.update(k, f))
    }

    pub fn update_with_key<F>(&self, k: &Value, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>) -> Option<Arc<Value>>,
    {
        Object(self.0.update_with_key(k, f))
    }

    pub fn update_lookup_with_key<F>(&self, k: &Value, f: F) -> (Option<Arc<Value>>, Self)
    where
        F: Fn(Arc<Value>, Arc<Value>) -> Option<Arc<Value>>,
    {
        let (l, r) = self.0.update_lookup_with_key(k, f);
        (l, Object(r))
    }

    pub fn alter<F>(&self, f: F, k: Value) -> Self
    where
        F: Fn(Option<Arc<Value>>) -> Option<Arc<Value>>,
    {
        Object(self.0.alter(f, k))
    }

    pub fn remove(&self, k: &Value) -> Self {
        Object(self.0.remove(k))
    }

    pub fn remove_mut(&mut self, k: &Value) {
        self.0.remove_mut(k)
    }

    pub fn pop(&self, k: &Value) -> Option<(Arc<Value>, Self)> {
        self.0.pop(k).map(|(r, l)| (r, Object(l)))
    }

    pub fn pop_mut(&mut self, k: &Value) -> Option<Arc<Value>> {
        self.0.pop_mut(k)
    }

    pub fn pop_with_key(&self, k: &Value) -> Option<(Arc<Value>, Arc<Value>, Self)> {
        self.0.pop_with_key(k).map(|(l, m, r)| (l, m, Object(r)))
    }

    pub fn pop_with_key_mut(&mut self, k: &Value) -> Option<(Arc<Value>, Arc<Value>)> {
        self.0.pop_with_key_mut(k)
    }

    pub fn union(&self, other: &Self) -> Self {
        Object(self.0.union(&other.0))
    }

    pub fn union_with<F>(&self, other: &Object, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>) -> Arc<Value>,
    {
        Object(self.0.union_with(&other.0, f))
    }

    pub fn union_with_key<F>(&self, other: &Object, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>, Arc<Value>) -> Arc<Value>,
    {
        Object(self.0.union_with_key(&other.0, f))
    }

    pub fn difference(&self, other: &Object) -> Self {
        Object(self.0.difference(&other.0))
    }

    pub fn difference_with<F>(&self, other: &Object, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>) -> Option<Arc<Value>>,
    {
        Object(self.0.difference_with(&other.0, f))
    }

    pub fn difference_with_key<F>(&self, other: &Object, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>, Arc<Value>) -> Option<Arc<Value>>,
    {
        Object(self.0.difference_with_key(&other.0, f))
    }

    pub fn intersection(&self, other: &Object) -> Self {
        Object(self.0.intersection(&other.0))
    }

    pub fn intersection_with<F>(&self, other: &Object, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>) -> Arc<Value>,
    {
        Object(self.0.intersection_with(&other.0, f))
    }

    pub fn intersection_with_key<F>(&self, other: &Object, f: F) -> Self
    where
        F: Fn(Arc<Value>, Arc<Value>, Arc<Value>) -> Arc<Value>,
    {
        Object(self.0.intersection_with_key(&other.0, f))
    }

    pub fn is_submap_by<F>(&self, other: &Object, cmp: F) -> bool
    where
        F: Fn(Arc<Value>, Arc<Value>) -> bool,
    {
        self.0.is_submap_by(&other.0, cmp)
    }

    pub fn is_proper_submap_by<F>(&self, other: &Object, cmp: F) -> bool
    where
        F: Fn(Arc<Value>, Arc<Value>) -> bool,
    {
        self.0.is_proper_submap_by(&other.0, cmp)
    }

    pub fn is_submap(&self, other: &Object) -> bool {
        self.0.is_submap(&other.0)
    }

    pub fn is_proper_submap(&self, other: &Object) -> bool {
        self.0.is_proper_submap(&other.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn object_from_btree() {
        let mut tree: BTreeMap<Value, Value> = BTreeMap::new();

        tree.insert("a".into(), 12.0.into());
        tree.insert("b".into(), 2.0.into());

        let obj: Object = tree.into();

        assert_eq!(*obj.get(&"a".into()), 12.0.into());
        assert_eq!(*obj.get(&"b".into()), 2.0.into());
    }

    #[test]
    fn btree_from_object() {
        let mut obj = Object::empty();

        obj.set_mut("a".into(), 12.0.into());
        obj.set_mut("b".into(), 2.0.into());

        let tree: BTreeMap<Value, Value> = obj.into();

        assert_eq!(*tree.get(&"a".into()).unwrap(), 12.0.into());
        assert_eq!(*tree.get(&"b".into()).unwrap(), 2.0.into());
    }

    #[test]
    fn get_null() {
        let null = super::get_null();

        match &*null {
            Value::Object(ref a) => assert_eq!(a.len(), 0),
            _ => unreachable!(),
        }
    }

    #[test]
    fn object_get_set() {
        let mut obj = Object::default();
        obj.set_mut("bar".into(), 12.0.into());
        let b = obj.get(&"bar".into());

        assert_eq!(&*b, &Value::from(12.0));
    }

    #[test]
    /// A crate this uses had issues printing empty maps, and the mistake was really easy to make.
    /// Thus, this test.
    fn object_fmt_empty() {
        let empty = Object::empty();

        assert_eq!(format!("{}", empty), "{}");
    }

    #[test]
    fn object_fmt_1() {
        let mut obj = Object::empty();

        obj.set_mut("a".into(), 1.0.into());

        assert_eq!(format!("{}", obj), "{\"a\": 1}");
    }

    #[test]
    fn object_fmt_2() {
        let mut obj = Object::empty();

        obj.set_mut("a".into(), 1.0.into());
        obj.set_mut("b".into(), 2.0.into());

        assert_eq!(format!("{}", obj), "{\"a\": 1, \"b\": 2}");
    }
}

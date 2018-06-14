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

//! Owned version of [`dermis::value::Object`](::value::Object)

use im::hashmap::{Keys, Values};
use im::HashMap;
use std::collections::BTreeMap;
use std::convert::From;
use std::sync::Arc;

use value::owned::value::OwnedValue;
use value::Object;

/// See [`get_null`](value::object::get_null).
pub fn get_null_owned() -> Arc<OwnedValue> {
    Arc::new(OwnedValue::Object(OwnedObject::empty()))
}

/// Owned version of [`dermis::value::Object`](::value::Object)
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, Default, From, Into, Serialize,
         Deserialize)]
pub struct OwnedObject(pub HashMap<OwnedValue, OwnedValue>);

impl From<Object> for OwnedObject {
    fn from(obj: Object) -> OwnedObject {
        OwnedObject(
            obj.0
                .into_iter()
                .map(|(k, v)| {
                    (
                        OwnedValue::from((*k).clone()),
                        OwnedValue::from((*v).clone()),
                    )
                })
                .collect(),
        )
    }
}

impl From<BTreeMap<OwnedValue, OwnedValue>> for OwnedObject {
    fn from(val: BTreeMap<OwnedValue, OwnedValue>) -> OwnedObject {
        HashMap::from(val).into()
    }
}

impl From<OwnedObject> for BTreeMap<OwnedValue, OwnedValue> {
    fn from(val: OwnedObject) -> BTreeMap<OwnedValue, OwnedValue> {
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

impl OwnedObject {
    /// See [`Object::get`](Object::get)
    pub fn get(&self, key: &OwnedValue) -> Arc<OwnedValue> {
        self.0.get(key).unwrap_or_else(|| get_null_owned())
    }

    /// See [`im::HashMap::get`](HashMap::get)
    pub fn get_opt(&self, k: &OwnedValue) -> Option<Arc<OwnedValue>> {
        self.0.get(k)
    }

    pub fn empty() -> Self {
        OwnedObject::default()
    }
}

impl OwnedObject {
    pub fn singleton(k: OwnedValue, v: OwnedValue) -> Self {
        OwnedObject(HashMap::singleton(k, v))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn keys(&self) -> Keys<OwnedValue, OwnedValue> {
        self.0.keys()
    }

    pub fn values(&self) -> Values<OwnedValue, OwnedValue> {
        self.0.values()
    }

    pub fn get_or(&self, k: &OwnedValue, default: OwnedValue) -> Arc<OwnedValue> {
        self.0.get_or(k, default)
    }

    pub fn contains_key(&self, k: &OwnedValue) -> bool {
        self.0.contains_key(k)
    }

    pub fn insert(&self, k: OwnedValue, v: OwnedValue) -> Self {
        OwnedObject(self.0.insert(k, v))
    }

    pub fn insert_mut(&mut self, k: OwnedValue, v: OwnedValue) {
        self.0.insert_mut(k, v)
    }

    pub fn set(&self, k: OwnedValue, v: OwnedValue) -> Self {
        OwnedObject(self.0.set(k, v))
    }

    pub fn set_mut(&mut self, k: OwnedValue, v: OwnedValue) {
        self.0.set_mut(k, v)
    }

    pub fn insert_with<F>(self, k: OwnedValue, v: OwnedValue, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>) -> Arc<OwnedValue>,
    {
        OwnedObject(self.0.insert_with(k, v, f))
    }

    pub fn insert_with_key<F>(self, k: OwnedValue, v: OwnedValue, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>, Arc<OwnedValue>) -> Arc<OwnedValue>,
    {
        OwnedObject(self.0.insert_with_key(k, v, f))
    }

    pub fn insert_lookup_with_key<F>(
        self,
        k: OwnedValue,
        v: OwnedValue,
        f: F,
    ) -> (Option<Arc<OwnedValue>>, Self)
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>, Arc<OwnedValue>) -> Arc<OwnedValue>,
    {
        let (l, r) = self.0.insert_lookup_with_key(k, v, f);
        (l, OwnedObject(r))
    }

    pub fn update<F>(&self, k: &OwnedValue, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>) -> Option<Arc<OwnedValue>>,
    {
        OwnedObject(self.0.update(k, f))
    }

    pub fn update_with_key<F>(&self, k: &OwnedValue, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>) -> Option<Arc<OwnedValue>>,
    {
        OwnedObject(self.0.update_with_key(k, f))
    }

    pub fn update_lookup_with_key<F>(&self, k: &OwnedValue, f: F) -> (Option<Arc<OwnedValue>>, Self)
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>) -> Option<Arc<OwnedValue>>,
    {
        let (l, r) = self.0.update_lookup_with_key(k, f);
        (l, OwnedObject(r))
    }

    pub fn alter<F>(&self, f: F, k: OwnedValue) -> Self
    where
        F: Fn(Option<Arc<OwnedValue>>) -> Option<Arc<OwnedValue>>,
    {
        OwnedObject(self.0.alter(f, k))
    }

    pub fn remove(&self, k: &OwnedValue) -> Self {
        OwnedObject(self.0.remove(k))
    }

    pub fn remove_mut(&mut self, k: &OwnedValue) {
        self.0.remove_mut(k)
    }

    pub fn pop(&self, k: &OwnedValue) -> Option<(Arc<OwnedValue>, Self)> {
        self.0.pop(k).map(|(r, l)| (r, OwnedObject(l)))
    }

    pub fn pop_mut(&mut self, k: &OwnedValue) -> Option<Arc<OwnedValue>> {
        self.0.pop_mut(k)
    }

    pub fn pop_with_key(&self, k: &OwnedValue) -> Option<(Arc<OwnedValue>, Arc<OwnedValue>, Self)> {
        self.0
            .pop_with_key(k)
            .map(|(l, m, r)| (l, m, OwnedObject(r)))
    }

    pub fn pop_with_key_mut(
        &mut self,
        k: &OwnedValue,
    ) -> Option<(Arc<OwnedValue>, Arc<OwnedValue>)> {
        self.0.pop_with_key_mut(k)
    }

    pub fn union(&self, other: &Self) -> Self {
        OwnedObject(self.0.union(&other.0))
    }

    pub fn union_with<F>(&self, other: &OwnedObject, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>) -> Arc<OwnedValue>,
    {
        OwnedObject(self.0.union_with(&other.0, f))
    }

    pub fn union_with_key<F>(&self, other: &OwnedObject, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>, Arc<OwnedValue>) -> Arc<OwnedValue>,
    {
        OwnedObject(self.0.union_with_key(&other.0, f))
    }

    pub fn difference(&self, other: &OwnedObject) -> Self {
        OwnedObject(self.0.difference(&other.0))
    }

    pub fn difference_with<F>(&self, other: &OwnedObject, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>) -> Option<Arc<OwnedValue>>,
    {
        OwnedObject(self.0.difference_with(&other.0, f))
    }

    pub fn difference_with_key<F>(&self, other: &OwnedObject, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>, Arc<OwnedValue>) -> Option<Arc<OwnedValue>>,
    {
        OwnedObject(self.0.difference_with_key(&other.0, f))
    }

    pub fn intersection(&self, other: &OwnedObject) -> Self {
        OwnedObject(self.0.intersection(&other.0))
    }

    pub fn intersection_with<F>(&self, other: &OwnedObject, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>) -> Arc<OwnedValue>,
    {
        OwnedObject(self.0.intersection_with(&other.0, f))
    }

    pub fn intersection_with_key<F>(&self, other: &OwnedObject, f: F) -> Self
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>, Arc<OwnedValue>) -> Arc<OwnedValue>,
    {
        OwnedObject(self.0.intersection_with_key(&other.0, f))
    }

    pub fn is_submap_by<F>(&self, other: &OwnedObject, cmp: F) -> bool
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>) -> bool,
    {
        self.0.is_submap_by(&other.0, cmp)
    }

    pub fn is_proper_submap_by<F>(&self, other: &OwnedObject, cmp: F) -> bool
    where
        F: Fn(Arc<OwnedValue>, Arc<OwnedValue>) -> bool,
    {
        self.0.is_proper_submap_by(&other.0, cmp)
    }

    pub fn is_submap(&self, other: &OwnedObject) -> bool {
        self.0.is_submap(&other.0)
    }

    pub fn is_proper_submap(&self, other: &OwnedObject) -> bool {
        self.0.is_proper_submap(&other.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn owned_object_from_btree() {
        let mut tree: BTreeMap<OwnedValue, OwnedValue> = BTreeMap::new();

        tree.insert("a".into(), 12.0.into());
        tree.insert("b".into(), 2.0.into());

        let obj: OwnedObject = tree.into();

        assert_eq!(*obj.get(&"a".into()), 12.0.into());
        assert_eq!(*obj.get(&"b".into()), 2.0.into());
    }

    #[test]
    fn owned_object_from_array() {
        let mut obj = OwnedObject::empty();

        obj.set_mut("a".into(), 12.0.into());
        obj.set_mut("b".into(), 2.0.into());

        let tree: BTreeMap<OwnedValue, OwnedValue> = obj.into();

        assert_eq!(*tree.get(&"a".into()).unwrap(), 12.0.into());
        assert_eq!(*tree.get(&"b".into()).unwrap(), 2.0.into());
    }
}

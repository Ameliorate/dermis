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

//! Owned version of [`dermis::value::Array`](::value::Array)

use im::Vector;
use im::vector::Iter;
use std::cmp::Ordering;
use std::sync::Arc;
use value::Array;
use value::owned::value::OwnedValue;

use std::convert::From;

/// Owned version of [`dermis::value::Array`](::value::Array)
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, Default, From, Into, Add, Serialize,
         Deserialize)]
pub struct OwnedArray(pub Vector<OwnedValue>);

impl From<Array> for OwnedArray {
    fn from(arr: Array) -> OwnedArray {
        OwnedArray(
            arr.0
                .into_iter()
                .map(|x| OwnedValue::from((*x).clone()))
                .collect(),
        )
    }
}

impl From<Vec<OwnedValue>> for OwnedArray {
    fn from(arr: Vec<OwnedValue>) -> OwnedArray {
        OwnedArray(arr.into())
    }
}

impl From<OwnedArray> for Vec<OwnedValue> {
    fn from(arr: OwnedArray) -> Vec<OwnedValue> {
        arr.0
            .into_iter()
            .map(|a: Arc<_>| Arc::try_unwrap(a).unwrap_or_else(|e| (&*e).clone()))
            .collect()
    }
}

impl OwnedArray {
    pub fn new() -> Self {
        OwnedArray(Vector::new())
    }

    pub fn empty() -> Self {
        OwnedArray::new()
    }
}

impl OwnedArray {
    pub fn singleton(a: OwnedValue) -> Self {
        OwnedArray(Vector::singleton(a))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> Iter<OwnedValue> {
        self.0.iter()
    }

    pub fn head(&self) -> Option<Arc<OwnedValue>> {
        self.0.head()
    }

    pub fn tail(&self) -> Option<OwnedArray> {
        self.0.tail().map(|a| OwnedArray(a))
    }

    pub fn last(&self) -> Option<Arc<OwnedValue>> {
        self.0.last()
    }

    pub fn init(&self) -> Option<OwnedArray> {
        self.0.init().map(|a| OwnedArray(a))
    }

    pub fn get(&self, index: usize) -> Option<Arc<OwnedValue>> {
        self.0.get(index)
    }

    pub fn get_unwrapped(&self, index: usize) -> Arc<OwnedValue> {
        self.0.get_unwrapped(index)
    }

    pub fn set(&self, index: usize, value: OwnedValue) -> Self {
        OwnedArray(self.0.set(index, value))
    }

    pub fn set_mut(&mut self, index: usize, value: OwnedValue) {
        self.0.set_mut(index, value)
    }

    pub fn push_back(&self, value: OwnedValue) -> Self {
        OwnedArray(self.0.push_back(value))
    }

    pub fn push_back_mut(&mut self, value: OwnedValue) {
        self.0.push_back_mut(value)
    }

    pub fn push_front(&self, value: OwnedValue) -> Self {
        OwnedArray(self.0.push_front(value))
    }

    pub fn push_front_mut(&mut self, value: OwnedValue) {
        self.0.push_front_mut(value)
    }

    pub fn append(&self, arr: OwnedArray) -> Self {
        OwnedArray(self.0.append(arr.0))
    }

    pub fn pop_back(&self) -> Option<(Arc<OwnedValue>, Self)> {
        self.0.pop_back().map(|(a, arr)| (a, OwnedArray(arr)))
    }

    pub fn pop_back_mut(&mut self) -> Option<Arc<OwnedValue>> {
        self.0.pop_back_mut()
    }

    pub fn pop_front(&self) -> Option<(Arc<OwnedValue>, Self)> {
        self.0.pop_front().map(|(a, arr)| (a, OwnedArray(arr)))
    }

    pub fn pop_front_mut(&mut self) -> Option<Arc<OwnedValue>> {
        self.0.pop_front_mut()
    }

    pub fn split_at(&self, index: usize) -> (Self, Self) {
        let (l, r) = self.0.split_at(index);
        (OwnedArray(l), OwnedArray(r))
    }

    pub fn skip(&self, count: usize) -> Self {
        OwnedArray(self.0.skip(count))
    }

    pub fn take(&self, count: usize) -> Self {
        OwnedArray(self.0.take(count))
    }

    pub fn slice(&self, start_index: usize, end_index: usize) -> Self {
        OwnedArray(self.0.slice(start_index, end_index))
    }

    pub fn reverse(&self) -> Self {
        OwnedArray(self.0.reverse())
    }

    pub fn reverse_mut(&mut self) {
        self.0.reverse_mut()
    }

    pub fn sort(&self) -> Self {
        OwnedArray(self.0.sort())
    }

    pub fn sort_by<F>(&self, cmp: F) -> Self
    where
        F: Fn(&OwnedValue, &OwnedValue) -> Ordering,
    {
        OwnedArray(self.0.sort_by(cmp))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn owned_array_from_array() {
        use decorum::N64;
        use value::Array;

        let array: Array = vec![12.0.into()].into();
        let owned: OwnedArray = array.into();
        assert_eq!(*owned.0.get_unwrapped(0), N64::from(12.0).into());
    }

    #[test]
    fn owned_to_vec() {
        let owned: OwnedArray = vec![OwnedValue::from("a"), OwnedValue::from("b")].into();

        let vec: Vec<OwnedValue> = owned.into();

        assert_eq!(vec[0], OwnedValue::from("a"));
        assert_eq!(vec[1], OwnedValue::from("b"));
    }

    /// This test is needed because there's a branch in From<OwnedArray> for Vec<OwnedValue> if
    /// the array is cloned.
    #[test]
    fn owned_to_vec_cloned() {
        let owned: OwnedArray = vec![OwnedValue::from("a"), OwnedValue::from("b")].into();
        let _cloned = owned.clone();

        let vec: Vec<OwnedValue> = owned.into();

        assert_eq!(vec[0], OwnedValue::from("a"));
        assert_eq!(vec[1], OwnedValue::from("b"));
    }
}

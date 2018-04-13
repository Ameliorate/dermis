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

use im::Vector;
use im::vector::Iter;
use std::cmp::Ordering;
use std::sync::Arc;

use value::{get_null, Value};

/// Any number of [`Value`](Value)s.
///
/// Does not necessarialy contain values of the same type.
///
/// For the documentation of Array's member functions, see [`im::Vector`](Vector).
///
/// # Examples
/// ```
/// use dermis::value::{Value, Array};
///
/// let array: Array = vec![Value::String("Foo".to_string()), Value::Number(12.0.into())].into();
///
/// assert_eq!(*array.get_unwrapped(0), Value::String("Foo".to_string()));
/// assert_eq!(*array.0.get_unwrapped(1), Value::Number(12.0.into()));
/// ```
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, Default, From, Into, Add)]
pub struct Array(pub Vector<Value>);

impl From<Vec<Value>> for Array {
    fn from(val: Vec<Value>) -> Array {
        Vector::from(val).into()
    }
}

impl From<Array> for Vec<Value> {
    fn from(arr: Array) -> Vec<Value> {
        arr.0
            .into_iter()
            .map(|a: Arc<_>| Arc::try_unwrap(a).unwrap_or_else(|e| (&*e).clone()))
            .collect()
    }
}

impl Array {
    pub fn new() -> Self {
        Array(Vector::new())
    }

    pub fn empty() -> Self {
        Array::new()
    }

    /// Gets the value at index. If the index given is past the end of the array, an empty
    /// Value::Object will be returned.
    ///
    /// # Example
    /// ```
    /// use dermis::value::{Value, Array, get_null};
    ///
    /// let arr: Array = vec!["a".into(), 12.0.into()].into();
    ///
    /// assert_eq!(*arr.get(0), Value::from("a"));
    /// assert_eq!(*arr.get(1), Value::from(12.0));
    /// assert_eq!(arr.get(2), get_null());
    /// ```
    pub fn get(&self, index: usize) -> Arc<Value> {
        self.0.get(index).unwrap_or_else(|| get_null())
    }

    /// See [`im::Vector::get`](im::Vector::get).
    pub fn get_opt(&self, index: usize) -> Option<Arc<Value>> {
        self.0.get(index)
    }
}

impl Array {
    pub fn singleton(a: Value) -> Self {
        Array(Vector::singleton(a))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> Iter<Value> {
        self.0.iter()
    }

    pub fn head(&self) -> Option<Arc<Value>> {
        self.0.head()
    }

    pub fn tail(&self) -> Option<Array> {
        self.0.tail().map(|a| Array(a))
    }

    pub fn last(&self) -> Option<Arc<Value>> {
        self.0.last()
    }

    pub fn init(&self) -> Option<Array> {
        self.0.init().map(|a| Array(a))
    }

    pub fn get_unwrapped(&self, index: usize) -> Arc<Value> {
        self.0.get_unwrapped(index)
    }

    pub fn set(&self, index: usize, value: Value) -> Self {
        Array(self.0.set(index, value))
    }

    pub fn set_mut(&mut self, index: usize, value: Value) {
        self.0.set_mut(index, value)
    }

    pub fn push_back(&self, value: Value) -> Self {
        Array(self.0.push_back(value))
    }

    pub fn push_back_mut(&mut self, value: Value) {
        self.0.push_back_mut(value)
    }

    pub fn push_front(&self, value: Value) -> Self {
        Array(self.0.push_front(value))
    }

    pub fn push_front_mut(&mut self, value: Value) {
        self.0.push_front_mut(value)
    }

    pub fn append(&self, arr: Array) -> Self {
        Array(self.0.append(arr.0))
    }

    pub fn pop_back(&self) -> Option<(Arc<Value>, Self)> {
        self.0.pop_back().map(|(a, arr)| (a, Array(arr)))
    }

    pub fn pop_back_mut(&mut self) -> Option<Arc<Value>> {
        self.0.pop_back_mut()
    }

    pub fn pop_front(&self) -> Option<(Arc<Value>, Self)> {
        self.0.pop_front().map(|(a, arr)| (a, Array(arr)))
    }

    pub fn pop_front_mut(&mut self) -> Option<Arc<Value>> {
        self.0.pop_front_mut()
    }

    pub fn split_at(&self, index: usize) -> (Self, Self) {
        let (l, r) = self.0.split_at(index);
        (Array(l), Array(r))
    }

    pub fn skip(&self, count: usize) -> Self {
        Array(self.0.skip(count))
    }

    pub fn take(&self, count: usize) -> Self {
        Array(self.0.take(count))
    }

    pub fn slice(&self, start_index: usize, end_index: usize) -> Self {
        Array(self.0.slice(start_index, end_index))
    }

    pub fn reverse(&self) -> Self {
        Array(self.0.reverse())
    }

    pub fn reverse_mut(&mut self) {
        self.0.reverse_mut()
    }

    pub fn sort(&self) -> Self {
        Array(self.0.sort())
    }

    pub fn sort_by<F>(&self, cmp: F) -> Self
    where
        F: Fn(&Value, &Value) -> Ordering,
    {
        Array(self.0.sort_by(cmp))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array_to_vec() {
        let arr: Array = vec![Value::from("a"), Value::from("b")].into();

        let vec: Vec<Value> = arr.into();

        assert_eq!(vec[0], Value::from("a"));
        assert_eq!(vec[1], Value::from("b"));
    }

    /// This test is needed because there's a branch in From<Array> for Vec<Value> if
    /// the array is cloned.
    #[test]
    fn array_to_vec_cloned() {
        let arr: Array = vec![Value::from("a"), Value::from("b")].into();
        let _cloned = arr.clone();

        let vec: Vec<Value> = arr.into();

        assert_eq!(vec[0], Value::from("a"));
        assert_eq!(vec[1], Value::from("b"));
    }

    #[test]
    fn array_get() {
        let mut arr = Array::empty();
        arr.push_back_mut(Value::from(1.0));
        arr.push_back_mut(Value::from(2.0));

        assert_eq!(*arr.get(0), Value::from(1.0));
        assert_eq!(*arr.get(1), Value::from(2.0));
    }

    #[test]
    fn array_null_get() {
        let arr = Array::empty();

        assert_eq!(arr.get(5), get_null());
    }
}

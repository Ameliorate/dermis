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

use Value;

/// Any number of [`Value`](Value)s.
///
/// Does not necessarialy contain values of the same type.
///
/// # Examples
/// ```
/// use dermis::value::{Value, Array};
///
/// let array: Array = vec![Value::String("Foo".to_string()), Value::Number(12.0.into())].into();
///
/// assert_eq!(array[0], Value::String("Foo".to_string()));
/// assert_eq!(array[1], Value::Number(12.0.into()));
/// ```
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, From, Into, Index, IndexMut)]
pub struct Array(pub Vec<Value>);

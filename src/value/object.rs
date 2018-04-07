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

use std::collections::BTreeMap;

use Value;

/// Like a Javascript Object, is a mapping between a key and a value, where both are of any
/// type.
///
/// # Example
/// ```
/// use dermis::value::{Object, Value};
/// use std::collections::BTreeMap;
///
/// let mut obj: Object = BTreeMap::new().into();
/// // See Value::Symbol for a good type for a key.
/// // This example does not use Symbol because it requires initializing an interpreter.
/// obj.0.insert(Value::String("number".to_string()), Value::Number(12.0.into()));
/// obj.0.insert(Value::String("string".to_string()), Value::String("Hello!".to_string()));
///
/// assert_eq!(obj.0.get(&Value::String("number".to_string())).unwrap(), &Value::Number(12.0.into()));
/// assert_eq!(obj.0.get(&Value::String("string".to_string())).unwrap(), &Value::String("Hello!".to_string()));
/// ```
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, From, Into)]
pub struct Object(pub BTreeMap<Value, Value>);

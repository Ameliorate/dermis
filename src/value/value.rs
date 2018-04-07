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

use std::fmt;
use std::fmt::{Display, Formatter};

use Array;
use Number;
use Object;
use Symbol;

/// Denotes any basic value possible in Dermis.
///
/// This enum can not be serialized, because it has references into interpreter internals.
///
/// For a serializeable version of this enum see the [`dermis::value::owned`](owned) module.
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, From)]
pub enum Value {
    /// Equal to a [`f64`](https://doc.rust-lang.org/std/primitive.f64.html).
    ///
    /// Integer types are not needed, as a double is equal to a 52 byte signed integer.
    /// In addition, the interpreter has (will have) types for simulating integer types.
    ///
    /// It should be noted that the decimal component will be discarded when doing comparisons
    /// between number types.
    ///
    /// See [`dermis::value::Number`](Number) for more info.
    ///
    /// # Example
    /// ```
    /// use dermis::value::Value;
    ///
    /// let number = Value::Number(12.0.into());
    ///
    /// let another_number: Value = 3.14.into();
    /// ```
    Number(Number),

    /// Basic string type.
    ///
    /// # Example
    /// ```
    /// use dermis::value::Value;
    ///
    /// let string = Value::String("Hello World!".to_string());
    ///
    /// let another_string: Value = "Hello Rust!".to_string().into();
    /// let third_string: Value = "Hello Dermis!".into();
    /// ```
    String(String),

    /// Like a string, however the value of a symbol can not be mutated.
    ///
    /// A symbol is used where an identifier would be in other languages.
    ///
    /// See [`dermis::value::symbol::Symbol`](Symbol) for more info.
    ///
    /// # Example
    /// ```
    /// use dermis::Interpreter;
    /// use dermis::value::{Value, Symbol};
    ///
    /// let mut interpreter = Interpreter::new();
    /// let symbol = Value::Symbol(Symbol::new("'foo".to_string(), &mut interpreter));
    ///
    /// let another_symbol: Value = Symbol::new("'bar".to_string(), &mut interpreter).into();
    /// ```
    Symbol(Symbol),

    /// Any number of other `Value`s.
    ///
    /// See [`dermis::value::Array`](Array) for more info.
    ///
    /// # Example
    /// ```
    /// use dermis::value::{Value, Array};
    ///
    /// let array = Value::Array(
    ///     vec![Value::String("a".to_string()),
    ///          Value::Number(12.0.into())].into());
    ///
    /// let another_array: Value = Array::from(vec!["a".into(), 3.14.into()]).into();
    /// ```
    Array(Array),

    /// Mirrors a Javascript Object in function and purpose.
    ///
    /// See [`Object`](Object) for more info.
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
    /// let obj_value = Value::Object(obj.clone());
    /// let another_obj: Value = obj.clone().into();
    /// ```
    Object(Object),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Value::Number(ref n) => write!(f, "{}", n),
            Value::String(ref s) => write!(f, "\"{}\"", s),
            Value::Symbol(ref s) => write!(f, "{}", s),

            Value::Array(Array(ref a)) => {
                write!(f, "[")?;
                let mut first = true;
                for v in a {
                    if !first {
                        write!(f, ", ")?;
                    } else {
                        first = false;
                    }
                    write!(f, "{}", v)?;
                }
                write!(f, "]")
            }

            Value::Object(Object(ref m)) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in m {
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
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Value {
        Value::Number(val.into())
    }
}

impl<'a> From<&'a str> for Value {
    fn from(val: &'a str) -> Value {
        Value::String(val.to_string())
    }
}

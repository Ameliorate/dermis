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

//! Provides an owned version of [`dermis::value::Value`](::value::Value)

use std::fmt::{Display, Formatter};
use std::fmt;
use std::convert::From;

use value::owned::array::OwnedArray;
use value::owned::object::OwnedObject;
use value::owned::symbol::OwnedSymbol;
use value::{Array, Number, Object, Symbol, Value};

/// Owned version of [`dermis::value::Value`](::value::Value)
///
/// Unlike [`Value`](::value::Value), this enum can be seralized and cloned without any reference
/// to the interpreter. If the interpreter is dropped while this value is held, this value will
/// continue to function as expected.
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, From)]
pub enum OwnedValue {
    Number(Number),
    String(String),
    Symbol(OwnedSymbol),
    Array(OwnedArray),
    Object(OwnedObject),
}

impl Display for OwnedValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            OwnedValue::Number(ref n) => write!(f, "{}", n),
            OwnedValue::String(ref s) => write!(f, "\"{}\"", s),
            OwnedValue::Symbol(ref s) => write!(f, "{}", s),

            OwnedValue::Array(OwnedArray(ref a)) => {
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

            OwnedValue::Object(OwnedObject(ref m)) => {
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

impl From<Value> for OwnedValue {
    fn from(val: Value) -> OwnedValue {
        match val {
            Value::Number(val) => OwnedValue::Number(val),
            Value::String(val) => OwnedValue::String(val),
            Value::Symbol(val) => OwnedValue::Symbol(val.into()),
            Value::Array(val) => OwnedValue::Array(val.into()),
            Value::Object(val) => OwnedValue::Object(val.into()),
        }
    }
}

impl From<f64> for OwnedValue {
    fn from(val: f64) -> OwnedValue {
        OwnedValue::Number(val.into())
    }
}

impl<'a> From<&'a str> for OwnedValue {
    fn from(val: &'a str) -> OwnedValue {
        OwnedValue::String(val.to_string())
    }
}

impl From<Object> for OwnedValue {
    fn from(val: Object) -> OwnedValue {
        OwnedValue::Object(val.into())
    }
}

impl From<Array> for OwnedValue {
    fn from(val: Array) -> OwnedValue {
        OwnedValue::Array(val.into())
    }
}

impl From<Symbol> for OwnedValue {
    fn from(val: Symbol) -> OwnedValue {
        OwnedValue::Symbol(val.into())
    }
}

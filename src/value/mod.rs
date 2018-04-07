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

//! Provides types to represent a value in Dermis.
//!
//! By default these types carry a reference to the interpreter that created them, and may panic if
//! the interpreter is dropped.
//! See the [`dermis::value::owned`](owned) module for types that do not carry this kind of
//! reference. In addition, owned values can be seralized, while the types in this module can
//! not.

pub mod array;
pub mod number;
pub mod object;
pub mod symbol;
pub mod value;

pub mod owned;

pub use self::array::Array;
pub use self::number::Number;
pub use self::object::Object;
pub use self::symbol::Symbol;
pub use self::value::Value;

pub use self::owned::array::OwnedArray;
pub use self::owned::object::OwnedObject;
pub use self::owned::symbol::OwnedSymbol;
pub use self::owned::value::OwnedValue;

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

#[macro_use]
extern crate derive_more;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate im;

#[macro_use]
mod macros;

pub mod value;

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use value::Symbol;

/// The central value for Dermis interpreter.
///
/// Multiple interpreters may be constructed per-process without issue.
#[derive(Debug)]
pub struct Interpreter {
    symbol_table: Arc<RwLock<SymbolTable>>,
}

impl Interpreter {
    /// Constucts a new Dermis Interpreter.
    ///
    /// # Examples
    ///
    /// ```
    /// use dermis::Interpreter;
    ///
    /// let interpreter = Interpreter::new();
    /// ```
    pub fn new() -> Interpreter {
        Interpreter {
            symbol_table: Arc::new(RwLock::new(Default::default())),
        }
    }
}

/// Internal table for symbol values.
#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct SymbolTable {
    global_symbols: Vec<Arc<String>>,
    symbols: HashMap<Symbol, Vec<Arc<String>>>,
}

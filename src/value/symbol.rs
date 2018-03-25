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

use std::hash::{Hash, Hasher};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::sync::{Arc, RwLock, Weak};

use {Interpreter, SymbolTable};

/// Like a string, however the value of a symbol can not be mutated.
///
/// A symbol is used where an identifier would be in other languages.
///
/// This struct carries a reference to the interpreter in order to store the string value of the
/// symbol.
/// If that is not desired, [`OwnedSymbol`](::value::owned::symbol::OwnedSymbol) should be used.
/// [`OwnedSymbol`](::value::owned::symbol::OwnedSymbol) also allows being serialized.
///
/// # Example
/// ```
/// use dermis::value::Symbol;
/// use dermis::Interpreter;
///
/// let mut interpreter = Interpreter::new();
///
/// let symbol = Symbol::new("a".to_string(), &mut interpreter);
///
/// assert_eq!(symbol.get_name(), "'a");
/// ```
#[derive(Debug, Clone)]
pub struct Symbol {
    name: Arc<String>,
    symbol_table: Weak<RwLock<SymbolTable>>,
}

impl Symbol {
    /// Returns a new symbol.
    ///
    /// If `name` does not start with an apostraphe ('), one will be added.
    ///
    /// Repeated callings of `Symbol::new` with the same name and interpreter will return `Symbol`s
    /// equal to each other.
    ///
    /// This function leaks memory equal to the size of Arc<String>.
    /// This would require great code reworks to be eliminated.
    ///
    /// # Example
    /// ```
    /// use dermis::value::Symbol;
    /// use dermis::Interpreter;
    ///
    /// let mut interpreter = Interpreter::new();
    ///
    /// let symbol_1 = Symbol::new("a".to_string(), &mut interpreter);
    /// let symbol_2 = Symbol::new("a".to_string(), &mut interpreter);
    ///
    ///
    /// assert_eq!(symbol_1.get_name(), "'a");
    /// assert_eq!(symbol_1, symbol_2);
    /// ```
    ///
    /// # Panics
    /// `name` contained a space. This limitation is in place to ease the creation of an input
    /// method for an IDE.
    pub fn new(mut name: String, interpreter: &mut Interpreter) -> Symbol {
        if !name.starts_with("'") {
            name.insert_str(0, "'");
        }

        if name.contains(" ") {
            panic!(
                "Symbols can not contain spaces but symbol {} contained a space",
                name
            );
        }

        let symbols: Vec<Arc<String>> = {
            interpreter
                .symbol_table
                .read()
                .expect(&format!(
                    "lock poisoned while creating symbol {}",
                    name.clone()
                ))
                .symbols
                .clone() // !!!
        }; // This is a block so that the read lock is dropped early.

        let name_a = symbols
            .iter()
            .find(|n| ***n == name)
            .map(|n| n.clone())
            .unwrap_or_else(|| {
                let name_a = Arc::new(name.clone());
                interpreter
                    .symbol_table
                    .write()
                    .expect(&format!(
                        "lock poisoned while creating symbol {}",
                        name.clone()
                    ))
                    .symbols
                    .push(name_a.clone());
                name_a
            })
            .clone();

        Symbol {
            name: name_a,
            symbol_table: Arc::downgrade(&interpreter.symbol_table),
        }
    }

    /// Returns the name of the symbol with the leading apostraphe.
    ///
    /// # Example
    /// ```
    /// use dermis::value::Symbol;
    /// use dermis::Interpreter;
    ///
    /// let mut interpreter = Interpreter::new();
    ///
    /// let symbol = Symbol::new("'symbol".to_string(), &mut interpreter);
    ///
    /// assert_eq!(symbol.get_name(), "'symbol");
    /// ```
    pub fn get_name(&self) -> &String {
        &*self.name
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        *self.name == *other.name
            && self.symbol_table
                .upgrade()
                .map(|s| {
                    other
                        .symbol_table
                        .upgrade()
                        .map(|o_s| Arc::ptr_eq(&s, &o_s))
                        .unwrap_or(false)
                })
                .unwrap_or(false)
    }
}

impl Eq for Symbol {}

impl Hash for Symbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.symbol_table.upgrade().is_some().hash(state);

        if let Some(table) = self.symbol_table.clone().upgrade() {
            (&*(table.read().unwrap()) as *const SymbolTable).hash(state);
        }
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Symbol) -> Option<Ordering> {
        (
            &self.name,
            self.symbol_table
                .upgrade()
                .map(|t| &*(t.read().unwrap()) as *const SymbolTable),
        ).partial_cmp(&(
            &other.name,
            other
                .symbol_table
                .upgrade()
                .map(|t| &*(t.read().unwrap()) as *const SymbolTable),
        ))
    }
}

impl Ord for Symbol {
    fn cmp(&self, other: &Symbol) -> Ordering {
        (
            &self.name,
            self.symbol_table
                .upgrade()
                .map(|t| &*(t.read().unwrap()) as *const SymbolTable),
        ).cmp(&(
            &other.name,
            other
                .symbol_table
                .upgrade()
                .map(|t| &*(t.read().unwrap()) as *const SymbolTable),
        ))
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

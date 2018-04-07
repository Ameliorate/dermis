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

use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
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
    /// The namespace of the symbol. If this is None, then the symbol is in the global namespace
    /// and was created with new instead of new_local.
    pub namespace: Option<Box<Symbol>>,
    symbol_table: Weak<RwLock<SymbolTable>>,
}

impl Symbol {
    /// Returns a new symbol in the global namespace.
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

        let mut name_a: Option<Arc<String>> = interpreter
            .symbol_table
            .read()
            .expect(&format!("lock poisoned while creating symbol {}", &name))
            .global_symbols
            .iter()
            .find(|n| ***n == name)
            .map(|n| n.clone());

        if name_a.is_none() {
            name_a = Some(Arc::new(name.clone()));
            interpreter
                .symbol_table
                .write()
                .expect(&format!("lock poisoned while creating symbol {}", &name))
                .global_symbols
                .push(name_a.clone().unwrap());
        }

        Symbol {
            name: name_a.unwrap(),
            namespace: None,
            symbol_table: Arc::downgrade(&interpreter.symbol_table),
        }
    }

    /// Creates a symbol local to it's namespace. Two symbols that share a name but not a namespace
    /// are not equal.
    ///
    /// See [`Symbol::new`](Symbol::new) for more info.
    ///
    /// # Example
    /// ```
    /// use dermis::{Interpreter, Symbol};
    ///
    /// let mut interpreter = Interpreter::new();
    ///
    /// let foo_namespace = Symbol::new("foo_namespace".to_string(), &mut interpreter);
    /// let local_a = Symbol::new_local("a".to_string(), foo_namespace.clone(), &mut interpreter);
    ///
    /// let bar_namespace = Symbol::new("bar_namespace".to_string(), &mut interpreter);
    /// let local_b = Symbol::new_local("a".to_string(), bar_namespace.clone(), &mut interpreter);
    ///
    /// assert_eq!(local_a.get_name(), local_b.get_name());
    /// assert_ne!(local_a, local_b);
    /// ```
    pub fn new_local(mut name: String, namespace: Symbol, interpreter: &mut Interpreter) -> Symbol {
        if !name.starts_with("'") {
            name.insert_str(0, "'");
        }

        if name.contains(" ") {
            panic!(
                "Symbols can not contain spaces but symbol {} contained a space",
                name
            );
        }

        let mut name_a: Option<Arc<String>> = interpreter
            .symbol_table
            .write()
            .expect(&format!("lock poisoned while creating symbol {}", &name))
            .symbols
            .entry(namespace.clone())
            .or_insert_with(|| vec![])
            .iter()
            .find(|n| ***n == name)
            .map(|n| n.clone());

        if name_a.is_none() {
            name_a = Some(Arc::new(name.clone()));
            interpreter
                    .symbol_table
                    .write()
                    .expect(&format!("lock poisoned while creating symbol {}", &name))
                    .symbols
                    .get_mut(&namespace)
                    .expect("symbol table namespace lookup should have been some") // Above will always set it to an empty vec if None.
                    .push(name_a.clone().unwrap());
        }

        Symbol {
            name: name_a.unwrap(),
            namespace: Some(Box::new(namespace)),
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
    /// let symbol_b = Symbol::new("another_symbol".to_string(), &mut interpreter);
    /// // In symbol_b's name, note the lack of a leading apostraphe.
    ///
    /// assert_eq!(symbol.get_name(), "'symbol");
    /// assert_eq!(symbol_b.get_name(), "'another_symbol"); // Note how one is added.
    /// ```
    pub fn get_name(&self) -> &String {
        &*self.name
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Symbol) -> bool {
        self.name == other.name && self.namespace == other.namespace
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
        self.namespace.hash(state);
        self.symbol_table.upgrade().is_some().hash(state);

        if let Some(table) = self.symbol_table.clone().upgrade() {
            (&*table as *const RwLock<SymbolTable>).hash(state);
        }
    }
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Symbol) -> Option<Ordering> {
        (
            &self.name,
            &self.namespace,
            self.symbol_table
                .upgrade()
                .map(|t| &*(t.read().unwrap()) as *const SymbolTable),
        ).partial_cmp(&(
            &other.name,
            &other.namespace,
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
            &self.namespace,
            self.symbol_table
                .upgrade()
                .map(|t| &*(t.read().unwrap()) as *const SymbolTable),
        ).cmp(&(
            &other.name,
            &other.namespace,
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

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
use std::convert::From;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::sync::{Arc, RwLock, Weak};

use self::format::SymbolFormat;
use value::OwnedSymbol;
use value::owned::symbol::LocalOwnedSymbol;
use {Interpreter, SymbolTable};

pub(crate) mod format;

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
/// let symbol = Symbol::new_global("a".to_string(), &mut interpreter);
///
/// assert_eq!(symbol.get_name(), "a");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    Local(LocalSymbol),
    Global(GlobalSymbol),
}

#[derive(Debug, Clone)]
pub struct LocalSymbol {
    pub(crate) name: Arc<String>,
    pub(crate) namespace: Box<Symbol>,
    pub(crate) symbol_table: Weak<RwLock<SymbolTable>>,
}

#[derive(Debug, Clone)]
pub struct GlobalSymbol {
    pub(crate) name: Arc<String>,
    pub(crate) symbol_table: Weak<RwLock<SymbolTable>>,
}

impl Symbol {
    /// Returns a new symbol in the global namespace.
    ///
    /// Repeated callings of `Symbol::new_global` with the same name and interpreter will return `Symbol`s
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
    /// let symbol_1 = Symbol::new_global("a".to_string(), &mut interpreter);
    /// let symbol_2 = Symbol::new_global("a".to_string(), &mut interpreter);
    ///
    ///
    /// assert_eq!(symbol_1.get_name(), "a");
    /// assert_eq!(symbol_1, symbol_2);
    /// ```
    ///
    /// # Panics
    /// `name` contained a space. This limitation is in place to ease the creation of an input
    /// method for an IDE.
    pub fn new_global(name: String, interpreter: &mut Interpreter) -> Symbol {
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

        Symbol::Global(GlobalSymbol {
            name: name_a.unwrap(),
            symbol_table: Arc::downgrade(&interpreter.symbol_table),
        })
    }

    /// Creates a symbol local to it's namespace. Two symbols that share a name but not a namespace
    /// are not equal.
    ///
    /// See [`Symbol::new_global`](Symbol::new_global) for more info.
    ///
    /// # Example
    /// ```
    /// use dermis::Interpreter;
    /// use dermis::value::Symbol;
    ///
    /// let mut interpreter = Interpreter::new();
    ///
    /// let foo_namespace = Symbol::new_global("foo_namespace".to_string(), &mut interpreter);
    /// let local_a = Symbol::new_local("a".to_string(), foo_namespace.clone(), &mut interpreter);
    ///
    /// let bar_namespace = Symbol::new_global("bar_namespace".to_string(), &mut interpreter);
    /// let local_b = Symbol::new_local("a".to_string(), bar_namespace.clone(), &mut interpreter);
    ///
    /// assert_eq!(local_a.get_name(), local_b.get_name());
    /// assert_ne!(local_a, local_b);
    /// ```
    pub fn new_local(name: String, namespace: Symbol, interpreter: &mut Interpreter) -> Symbol {
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

        Symbol::Local(LocalSymbol {
            name: name_a.unwrap(),
            namespace: Box::new(namespace),
            symbol_table: Arc::downgrade(&interpreter.symbol_table),
        })
    }

    /// Converts from an owned symbol.
    pub fn from_owned(owned: &OwnedSymbol, interpreter: &mut Interpreter) -> Symbol {
        (owned, interpreter).into()
    }

    /// Returns the name of the symbol.
    ///
    /// # Example
    /// ```
    /// use dermis::value::Symbol;
    /// use dermis::Interpreter;
    ///
    /// let mut interpreter = Interpreter::new();
    ///
    /// let symbol = Symbol::new_global("symbol".to_string(), &mut interpreter);
    /// let symbol_b = Symbol::new_global("another_symbol".to_string(), &mut interpreter);
    ///
    /// assert_eq!(symbol.get_name(), "symbol");
    /// assert_eq!(symbol_b.get_name(), "another_symbol"); // Note how one is added.
    /// ```
    pub fn get_name(&self) -> &String {
        match self {
            Symbol::Global(GlobalSymbol {
                name,
                symbol_table: _,
            }) => &name,
            Symbol::Local(LocalSymbol {
                name,
                namespace: _,
                symbol_table: _,
            }) => &name,
        }
    }
}

impl PartialEq for GlobalSymbol {
    fn eq(&self, other: &GlobalSymbol) -> bool {
        self.name == other.name
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

impl Eq for GlobalSymbol {}

impl PartialEq for LocalSymbol {
    fn eq(&self, other: &LocalSymbol) -> bool {
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

impl Eq for LocalSymbol {}

impl Hash for LocalSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.namespace.hash(state);

        if let Some(table) = self.symbol_table.clone().upgrade() {
            (&*table as *const RwLock<SymbolTable>).hash(state);
        }
    }
}

impl Hash for GlobalSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);

        if let Some(table) = self.symbol_table.clone().upgrade() {
            (&*table as *const RwLock<SymbolTable>).hash(state);
        }
    }
}

impl PartialOrd for GlobalSymbol {
    fn partial_cmp(&self, other: &GlobalSymbol) -> Option<Ordering> {
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

impl Ord for GlobalSymbol {
    fn cmp(&self, other: &GlobalSymbol) -> Ordering {
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

impl PartialOrd for LocalSymbol {
    fn partial_cmp(&self, other: &LocalSymbol) -> Option<Ordering> {
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

impl Ord for LocalSymbol {
    fn cmp(&self, other: &LocalSymbol) -> Ordering {
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

impl<'a> From<&'a Symbol> for SymbolFormat<'a> {
    fn from(val: &'a Symbol) -> SymbolFormat<'a> {
        use Symbol::*;
        match val {
            Global(GlobalSymbol {
                name,
                symbol_table: _,
            }) => SymbolFormat::Global(&name),
            Local(LocalSymbol {
                name,
                namespace,
                symbol_table: _,
            }) => SymbolFormat::Local(&name, Box::new((&**namespace).into())),
        }
    }
}

impl<'a, 'b> From<(&'a OwnedSymbol, &'b mut Interpreter)> for Symbol {
    fn from((val, i): (&'a OwnedSymbol, &'b mut Interpreter)) -> Symbol {
        use value::owned::symbol::OwnedSymbol::*;
        match val {
            Global(_) => Symbol::new_global(val.get_name().clone(), i),
            Local(LocalOwnedSymbol {
                ref name,
                namespace,
            }) => Symbol::new_local(name.clone(), (&**namespace, &mut *i).into(), i),
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", SymbolFormat::from(self))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn symbol_from_owned_global() {
        let owned = symbol_o!(foo);
        let mut i = Interpreter::new();

        let s: Symbol = (&owned, &mut i).into();

        assert_eq!(s.to_string(), owned.to_string());
    }

    #[test]
    fn symbol_from_owned_local_1() {
        let owned = symbol_o!(foo;bar);
        let mut i = Interpreter::new();

        let s: Symbol = (&owned, &mut i).into();

        assert_eq!(s.to_string(), owned.to_string());
    }

    #[test]
    fn symbol_from_owned_local_2() {
        let owned = symbol_o!(foo;bar;baz);
        let mut i = Interpreter::new();

        let s: Symbol = (&owned, &mut i).into();

        assert_eq!(s.to_string(), owned.to_string());
    }
}

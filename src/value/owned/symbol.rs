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

//! Provides an owned version of [`dermis::value::symbol::Symbol`](::value::Symbol).

use std::convert::From;
use std::fmt;
use std::fmt::{Display, Formatter};

use value::Symbol;
use value::symbol::format::SymbolFormat;

/// Provides an owned version of [`dermis::value::symbol::Symbol`](::value::Symbol).
///
/// This value, unlike [`Symbol`](::value::Symbol), can be created without a reference to an
/// interpreter.
///
/// # Example
///
/// ```
/// use dermis::value::{OwnedSymbol, Symbol};
/// use dermis::Interpreter;
///
/// let owned_symbol = OwnedSymbol::new("'foo".to_string());
///
/// assert_eq!(owned_symbol.get_name(), "'foo");
///
/// let another_symbol: OwnedSymbol = {
///     let mut interpreter = Interpreter::new();
///     let symbol = Symbol::new("'bar".to_string(), &mut interpreter);
///     symbol.into()
/// };
///
/// assert_eq!(another_symbol.get_name(), "'bar");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct OwnedSymbol {
    name: String,
    pub namespace: Option<Box<OwnedSymbol>>,
}

impl OwnedSymbol {
    /// Creates a new symbol.
    ///
    /// # Panics
    /// `name` contained a space. This limitation is in place to ease the creation of an input
    /// method for an IDE.
    pub fn new(name: String) -> OwnedSymbol {
        if name.contains(" ") {
            panic!(
                "Symbols can not contain spaces but symbol {} contained a space.",
                name
            );
        }

        OwnedSymbol {
            name,
            namespace: None,
        }
    }

    /// Creates a new symbol in the given namespace. See [`Symbol::new_local`](Symbol::new_local) for more info.
    ///
    /// # Panics
    /// `name` contained a space.
    pub fn new_local(name: String, namespace: OwnedSymbol) -> OwnedSymbol {
        if name.contains(" ") {
            panic!(
                "Symbols can not contain spaces but symbol {} contained a space.",
                name
            );
        }

        OwnedSymbol {
            name,
            namespace: Some(Box::new(namespace)),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_namespace(&self) -> Option<&OwnedSymbol> {
        self.namespace.as_ref().map(|ns: &Box<_>| &**ns)
    }
}

impl<'a> From<&'a OwnedSymbol> for SymbolFormat<'a> {
    fn from(val: &'a OwnedSymbol) -> SymbolFormat<'a> {
        if let Some(namespace) = &val.namespace {
            SymbolFormat::Local(val.get_name(), Box::new((&**namespace).into()))
        } else {
            SymbolFormat::Global(val.get_name())
        }
    }
}

impl Display for OwnedSymbol {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", SymbolFormat::from(self))
    }
}

impl From<Symbol> for OwnedSymbol {
    fn from(val: Symbol) -> OwnedSymbol {
        OwnedSymbol::new(val.get_name().to_string())
    }
}

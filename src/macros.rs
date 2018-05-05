/*
 * Dermis is an interpreter for a pure, statically typed, imperative language designed to be edited with a custom IDE.
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

//! Contains several utility macros for initializing values, etc.

/// Initializes a local or global owned symbol.
///
/// See [`OwnedSymbol`](::value::OwnedSymbol).
///
/// For a global symbol, you should call this macro with the name of the symbol, without any
/// quotes.
///
/// For a local symbol, you should separate each namespace of the symbol with semicolons, with the
/// name of the symbol at the end. It would significantly complicate this macro to use a double
/// colon instead.
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate dermis;
///
/// # fn main() {
/// let global = symbol_o!(global);
/// let local = symbol_o!(foo;bar;baz);
///
/// assert_eq!(&global.to_string(), "'global");
/// assert_eq!(&local.to_string(), "'foo::bar::baz");
/// # }
/// ```
#[macro_export]
macro_rules! symbol_o {
    ($name:ident) => {{
        $crate::value::OwnedSymbol::new_global(stringify!($name).to_string())
    }};
    ($head:ident ; $( $tail:ident );+) => {{
        let mut sym = $crate::value::OwnedSymbol::new_global(stringify!($head).to_string());
        $(
            sym = $crate::value::OwnedSymbol::new_local(stringify!($tail).to_string(), sym);
        )*
        sym
    }};
}

/// Initalizes a local or global symbol.
///
/// See [`Symbol`](::value::Symbol).
///
/// This macro has a syntax like `symbol!([NAME], &mut [INTERPRETER])` where `[NAME]` and
/// `[INTERPRETER]` are replaced with the name of the symbol and the interpreter. The name should be
/// a valid rust identifier (although beware Symbol values can be invalid rust identifiers).
///
/// The name can also be a series of identifiers seperated by semicolons, in order to denote a
/// local symbol. Semicolons are used because a double colon (as is used in the rest of the
/// interpreter) would overcomplicate the macro definition.
///
/// # Example
/// ```
/// #[macro_use]
/// extern crate dermis;
/// use dermis::Interpreter;
///
/// # fn main() {
/// let mut interpreter = Interpreter::new();
///
/// let global = symbol!(global, &mut interpreter);
/// let local = symbol!(foo;bar;baz, &mut interpreter);
///
/// assert_eq!(&global.to_string(), "'global");
/// assert_eq!(&local.to_string(), "'foo::bar::baz");
/// # }
/// ```
#[macro_export]
macro_rules! symbol {
    ($name:ident, $interp:expr) => {{
        $crate::value::Symbol::new_global(stringify!($name).to_string(), $interp)
    }};
    ($head:ident ; $( $tail:ident );+ , $interp:expr) => {{
        let mut sym = $crate::value::Symbol::new_global(stringify!($head).to_string(), $interp);
        $(
            sym = $crate::value::Symbol::new_local(stringify!($tail).to_string(), sym, $interp);
        )*
        sym
    }};
}

#[cfg(test)]
mod test {
    use Interpreter;

    #[test]
    fn symbol_o_macro_global() {
        let sym = symbol_o!(test);

        assert_eq!(sym.get_name(), "test");
    }

    #[test]
    fn symbol_o_macro_local_1() {
        let sym = symbol_o!(test;a);

        assert_eq!(&sym.to_string(), "'test::a");
    }

    #[test]
    fn symbol_o_macro_local_2() {
        let sym = symbol_o!(test;a;b);

        assert_eq!(&sym.to_string(), "'test::a::b");
    }

    #[test]
    fn symbol_macro_global() {
        let mut i = Interpreter::new();
        let sym = symbol!(test, &mut i);

        assert_eq!(sym.get_name(), "test");
    }

    #[test]
    fn symbol_macro_local_1() {
        let mut i = Interpreter::new();
        let sym = symbol!(test;a, &mut i);

        assert_eq!(&sym.to_string(), "'test::a");
    }

    #[test]
    fn symbol_macro_local_2() {
        let mut i = Interpreter::new();
        let sym = symbol!(test;a;b, &mut i);

        assert_eq!(&sym.to_string(), "'test::a::b");
    }
}

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

/// Allows for the formatting of a symbol in a generic way.
#[derive(Debug, Clone)]
pub(crate) enum SymbolFormat<'a> {
    Global(&'a str),
    Local(&'a str, Box<SymbolFormat<'a>>),
    Anonymous,
}

impl<'a> Display for SymbolFormat<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.fmt_(f)
    }
}

impl<'a> SymbolFormat<'a> {
    fn fmt_(&self, f: &mut Formatter) -> fmt::Result {
        use self::SymbolFormat::*;
        match self {
            Global(ref name) => write!(f, "'{}", name),
            Local(ref name, ref namespace) => {
                namespace.fmt_(f)?;
                write!(f, "::{}", &name)
            }
            Anonymous => write!(f, "'_"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn symbol_format_global() {
        let spec = SymbolFormat::Global("foo");

        assert_eq!(spec.to_string(), "'foo");
    }

    #[test]
    fn symbol_format_local_global() {
        let ns = SymbolFormat::Global("foo");
        let spec = SymbolFormat::Local("bar", ns.into());

        assert_eq!(spec.to_string(), "'foo::bar");
    }

    #[test]
    fn symbol_format_local_local_global() {
        let ns1 = SymbolFormat::Global("foo");
        let ns2 = SymbolFormat::Local("bar", ns1.into());
        let spec = SymbolFormat::Local("dee", ns2.into());

        assert_eq!(spec.to_string(), "'foo::bar::dee");
    }

    #[test]
    fn symbol_format_anon() {
        assert_eq!(SymbolFormat::Anonymous.to_string(), "'_");
    }

    #[test]
    fn symbol_format_anon_local() {
        let ns = SymbolFormat::Anonymous;
        let spec = SymbolFormat::Local("foo", ns.into());

        assert_eq!(spec.to_string(), "'_::foo");
    }
}

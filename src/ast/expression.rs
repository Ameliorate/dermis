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

use value::{OwnedValue, OwnedObject};

type E = Box<Expression>;

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Expression {
    Nop,
    /// Used for setting IDE-specific options on an expression.
    ///
    /// When ran, this will return the value of id.
    IdeOption { id: E, options: OwnedObject },

    // Logical Operators:
    Cond { cond: E, if_true: E, if_false: E, display: CondDisplay},
//    LAnd(E, E),
//    LOr(E, E),
//    LXor(E, E),
//    LNot(E),
//    NotNull(E),
//    IsNull(E),
//
//    // Comparison Operators:
//    /// Rounds and then compares two floats.
//    ///
//    /// `abs(lhs - rhs) < max(lhs, rhs) * rounding_factor` is how this is calculated.
//    ///
//    /// This calculaton was taken from the J programming language, see 
//    /// http://code.jsoftware.com/wiki/Essays/Tolerant_Comparison
//    ///
//    /// If the value of `rounding_factor` is negative 2^-44 will be used.
//    FloatingEqual { lhs: E, rhs: E, rounding_factor: f64 },
//    FloatingNE(E, E),
//    Equal(E, E),
//    NotEqual(E, E),
//    LessThan(E, E),
//    GreaterThan(E, E),
//    LesserOrEqual(E, E),
//    GreaterOrEqual(E, E),
//
//    // Math Operators:
//    StrConcat(E, E),
//    Add(E, E),
//    Subtract(E, E),
//    Multiply(E, E),
//    Divide(E, E),
//    IntDivide(E, E),
//    Exponent(E, E),
//    Sqrt(E),
//    Log(E),
} // That's a lot of E's

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug, Serialize, Deserialize)]
pub enum CondDisplay {
    /// Display like an if/else expression.
    If,
    /// Display like a C-style ternary expression.
    Ternary,
}

impl From<CondDisplay> for OwnedValue {
    fn from(d: CondDisplay) -> OwnedValue {
        use self::CondDisplay::*;
        match d {
            If => symbol_o!(Ast;CondDisplay;If).into(),
            Ternary => symbol_o!(Ast;CondDisplay;Ternary).into(),
        }
    }
}

impl From<Expression> for OwnedValue {
    fn from(expr: Expression) -> OwnedValue {
        use self::Expression::*;
        match expr {
            Nop => symbol_o!(ast;nop).into(),
            IdeOption { id, options } => {
                let mut o = OwnedObject::empty();
                o.set_mut(symbol_o!(Ast;IdeOption;Id).into(), (*id).into());
                o.set_mut(symbol_o!(Ast;IdeOption;Options).into(), options.into());
                o.into()
            }
            Cond { cond, if_true, if_false, display } => {
                let mut o = OwnedObject::empty();
                o.set_mut(symbol_o!(Ast;Cond).into(), (*cond).into());
                o.set_mut(symbol_o!(Ast;Cond;IfFalse).into(), (*if_false).into());
                o.set_mut(symbol_o!(Ast;Cond;IfTrue).into(), (*if_true).into());
                o.set_mut(symbol_o!(Ast;Cond;Display).into(), display.into());
                o.into()
            }
        }
    }
}

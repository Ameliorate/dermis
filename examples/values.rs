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

extern crate dermis;

use dermis::value::{Symbol, Value};
use dermis::Interpreter;

fn main() {
    let mut i = Interpreter::new(); // Symbols point into the interpreter, so we must create one beforehand.

    let number = Value::Number(12.0.into());
    // Standard floating number. Ints are not needed, since a double can store equal to a 52 byte int.

    let string = Value::String("Hello World!".to_string()); // Nothing notible about a string.

    let symbol = Symbol::new("lst".to_string(), &mut i);
    let symbol_val = Value::Symbol(symbol.clone());
    // A symbol is like a string, but it can't be mutated and is more
    // like an identifier in other languages.
    // It should be noted that the symbol's name is leaked for the lifetime of the interpreter.

    let array = Value::Array(vec![number.clone(), Value::Number(13.0.into())].into()); // Standard array

    // TODO: Add object example.
    println!(
        "number: {}, string: {}, symbol: {}, symbol_val: {}, array: {}",
        number,
        string,
        symbol.clone(),
        symbol_val,
        array
    );

    assert_eq!(Value::Number(12.65.into()), Value::Number(12.9.into()));
    // It should be noted that due to implementation details, numbers are trunctuated when
    // compared.
}

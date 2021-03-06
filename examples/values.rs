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

use dermis::value::{Object, Symbol, Value};
use dermis::Interpreter;

use std::sync::Arc;

fn main() {
    let mut i = Interpreter::new(); // Symbols point into the interpreter, so we must create one beforehand.

    let number = Value::Number(12.0.into());
    // Standard floating number. Ints are not needed, since a double can store equal to a 52 byte int.

    let string = Value::String("Hello World!".to_string()); // Nothing notible about a string.

    let symbol = Symbol::new_global("lst".to_string(), &mut i);
    let symbol_val = Value::Symbol(symbol.clone());
    // A symbol is like a string, but it can't be mutated and is more
    // like an identifier in other languages.
    // It should be noted that the symbol's name is leaked for the lifetime of the interpreter.

    let array = Value::Array(vec![number.clone(), Value::Number(13.0.into())].into()); // Standard array

    let mut obj = Object::empty();

    obj = obj.set(symbol_val.clone(), array.clone());
    // Set will clone the object, thus the obj =.
    // It uses an Arc to avoid cloning the whole list, though.
    // Being able to have many slightly different lists is a net-gain in the interpreter
    // preformance-regards.

    obj.set_mut(
        Symbol::new_global("num".to_string(), &mut i).into(),
        number.clone(),
    );
    // Object::set_mut will automatically handle mutibility.
    //
    // Value has a bunch of From impl's that allow you to convert the value inner types easily.

    let obj_num: Arc<Value> = obj.get(&Symbol::new_global("num".to_string(), &mut i).into());

    println!(
        "number: {}, string: {}, symbol: {}, symbol_val: {}, array: {}, obj: {}, obj.num: {}",
        number, string, symbol, symbol_val, array, obj, obj_num
    );
}

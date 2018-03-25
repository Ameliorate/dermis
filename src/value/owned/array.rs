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

//! Owned version of [`dermis::value::Array`](::value::Array)

use value::owned::value::OwnedValue;
use value::Array;

use std::convert::From;

/// Owned version of [`dermis::value::Array`](::value::Array)
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, From, Into, Index)]
pub struct OwnedArray(pub Vec<OwnedValue>);

impl From<Array> for OwnedArray {
    fn from(arr: Array) -> OwnedArray {
        OwnedArray(arr.0.into_iter().map(|x| x.into()).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn owned_array_from_array() {
        use value::{Array, Number};

        let array: Array = vec![12.0.into()].into();
        let owned: OwnedArray = array.into();
        assert_eq!(owned[0], Number::from(12.0).into());
    }
}

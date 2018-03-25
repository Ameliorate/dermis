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

//! Owned version of [`dermis::value::Object`](::value::Object)

use std::collections::BTreeMap;
use std::convert::From;

use value::owned::value::OwnedValue;
use value::Object;

/// Owned version of [`dermis::value::Object`](::value::Object)
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone, From, Into)]
pub struct OwnedObject(pub BTreeMap<OwnedValue, OwnedValue>);

impl From<Object> for OwnedObject {
    fn from(obj: Object) -> OwnedObject {
        OwnedObject(
            obj.0
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

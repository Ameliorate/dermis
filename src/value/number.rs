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

/// Contains a basic [`f64`](https://doc.rust-lang.org/std/primitive.f64.html), adding needed
/// equality and ordering traits.
///
/// It should be noted that for the traits Eq, PartialEq, and Ord that the decimal portion is
/// trunctuated when two Numbers are compared. Notibly missing from that list is the PartialOrd
/// trait. As a result, the built-in operators of rust are spared this effect.
/// For example, the following test passes:
///
/// # Example
/// ```
/// use dermis::value::Number;
///
/// let number = Number::from(12.0);
/// let other_number = Number::from(12.5);
///
/// assert_eq!(number.val, 12.0);
/// assert_eq!(f64::from(number), 12.0);
/// // Under some circumstances, you could use number.into(),
/// // but here is not one of those circumstances.
///
/// assert_eq!(number, other_number);
///
/// assert!(number <  other_number);
/// assert!(number <= other_number);
/// assert!(other_number >  number);
/// assert!(other_number >= number);
/// ```
#[derive(Debug, Clone, Copy, PartialOrd, Serialize, Deserialize, From, Into)]
pub struct Number {
    /// The number contained.
    ///
    /// No accuracy is lost while the value is stored, only in comparison.
    pub val: f64,
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        self.val as i64 == other.val as i64
    }
}

impl Eq for Number {}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.val as i64).hash(state);
    }
}

impl Ord for Number {
    fn cmp(&self, other: &Number) -> Ordering {
        (self.val as i64).cmp(&(other.val as i64))
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

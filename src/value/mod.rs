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

//! Provides types to represent a value in Dermis.
//!
//! By default these types carry a reference to the interpreter that created them, and may panic if
//! the interpreter is dropped.
//! See the [`dermis::value::owned`](owned) module for types that do not carry this kind of
//! reference. In addition, owned values can be seralized, while the types in this module can
//! not.

pub mod array;
pub mod object;
pub mod symbol;
pub mod value;

pub mod owned;

pub use self::array::Array;
pub use self::object::{get_null, Object};
pub use self::symbol::Symbol;
pub use self::value::Value;

pub use self::owned::array::OwnedArray;
pub use self::owned::object::OwnedObject;
pub use self::owned::symbol::OwnedSymbol;
pub use self::owned::value::OwnedValue;

pub use decorum::N64;

use std::cmp::Ordering;
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use Interpreter;

pub type Number = N64;

/// Any sort of value, owned or unowned.
///
/// It should be noted that the PartialEq, PartialOrd, and Ord implementations for this enum are
/// somewhat slow, because they have to clone their value and convert them to OwnedValues if
/// needed.
///
/// Upon Seralialization, this enum will always convert itself to the `Owned` variant. Thus, the
/// only variant that can be present upon deserializing this value is `A`.
///
/// # Example
/// ```
/// use dermis::value::{Number, AValue, Value, OwnedValue};
///
/// let num: Number = 12.0.into();
/// let val: Value = num.clone().into();
/// let owned: OwnedValue = num.clone().into();
///
/// let a: AValue = val.into();
/// let b: AValue = owned.into();
///
/// assert_eq!(a, b);
/// ```
#[derive(Debug, Clone, From)]
pub enum AValue {
    Owned(OwnedValue),
    A(Value),
}

/// Used for comparing [`AValue`](AValue)'s by type rather by value.
///
/// Comparisons between these are faster than [`AValue`](AValue), since this enum uses the stock
/// rust `derive`'s to implement those traits.
///
/// # Example
/// ```
/// use dermis::value::{Number, AValue, Value, OwnedValue};
/// 
/// let num: Number = 12.0.into();
/// let val: Value = num.clone().into();
/// let owned: OwnedValue = num.clone().into();
///
/// let a: AValue = val.into();
/// let b: AValue = owned.into();
/// let a_c = a.cmp_variants();
/// let b_c = b.cmp_variants();
///
/// assert_eq!(a, b);
/// assert_ne!(a_c, b_c);
/// ```
#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Debug, Clone)]
pub enum VariantCmpAValue<'val> {
    Owned(&'val OwnedValue),
    A(&'val Value),
}

impl<'val> From<&'val AValue> for VariantCmpAValue<'val> {
    fn from(val: &'val AValue) -> VariantCmpAValue<'val> {
        match val {
            AValue::A(ref val) => VariantCmpAValue::A(val),
            AValue::Owned(ref val) => VariantCmpAValue::Owned(val),
        }
    }
}

impl AValue {
    /// Convert this value into an [`OwnedValue`](OwnedValue). This will call `.into()` on an `A` variant, but is a
    /// no-op if it is a `Owned` variant.
    ///
    /// # Example
    /// ```
    /// use dermis::value::{AValue, Value, OwnedValue, Number};
    ///
    /// let num: Number = 12.0.into();
    /// let val: Value = num.into();
    /// let a_val: AValue = val.into();
    /// let owned: OwnedValue = a_val.into_owned();
    ///
    /// assert_eq!(owned, OwnedValue::from(num));
    /// ```
    pub fn into_owned(self) -> OwnedValue {
        use self::AValue::*;
        match self {
            Owned(val) => val,
            A(val) => val.into(),
        }
    }

    /// Convert this value to a normal [`Value`](Value). If this is the `Owned` variant, the value
    /// will be converted to a [`Value`](Value). If it is the correct variant, this is a no-op.
    ///
    /// This function is not yet implemented. See issue #4 for more info.
    pub fn into_unowned(self, _: &mut Interpreter) -> Value {
        unimplemented!() // TODO: issue #4
    }
}

impl<'val> AValue {
    /// Compare the individual variants of the [`AValue`](AValue) enum rather than the value held
    /// by each variant.
    ///
    /// This makes comparisons as fast as the default rust `derive`'s.
    ///
    /// See also [`VariantCmpAValue`](VariantCmpAValue).
    ///
    /// # Example
    /// ```
    /// use dermis::value::{Number, AValue, Value, OwnedValue};
    ///
    /// let num: Number = 12.0.into();
    /// let val: Value = num.clone().into();
    /// let owned: OwnedValue = num.clone().into();
    ///
    /// let a: AValue = val.into();
    /// let b: AValue = owned.into();
    /// let a_c = a.cmp_variants();
    /// let b_c = b.cmp_variants();
    ///
    /// assert_eq!(a, b);
    /// assert_ne!(a_c, b_c);
    /// ```
    pub fn cmp_variants(&'val self) -> VariantCmpAValue<'val> {
        self.into()
    }
}

impl PartialEq for AValue {
    fn eq(&self, other: &AValue) -> bool {
        use self::AValue::*;
        match self {
            A(val) => OwnedValue::from(val.clone()),
            Owned(val) => val.clone(),
        }.eq(&match other {
            A(val) => OwnedValue::from(val.clone()),
            Owned(val) => val.clone(),
        })
    }
}

impl Eq for AValue {}

impl PartialOrd for AValue {
    fn partial_cmp(&self, other: &AValue) -> Option<Ordering> {
        use self::AValue::*;
        match self {
            A(val) => OwnedValue::from(val.clone()),
            Owned(val) => val.clone(),
        }.partial_cmp(&match other {
            A(val) => OwnedValue::from(val.clone()),
            Owned(val) => val.clone(),
        })
    }
}

impl Ord for AValue {
    fn cmp(&self, other: &AValue) -> Ordering {
        use self::AValue::*;
        match self {
            A(val) => OwnedValue::from(val.clone()),
            Owned(val) => val.clone(),
        }.cmp(&match other {
            A(val) => OwnedValue::from(val.clone()),
            Owned(val) => val.clone(),
        })
    }
}

impl Serialize for AValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use self::AValue::*;
        let val = match self {
            A(v) => OwnedValue::from(v.clone()),
            Owned(v) => v.clone(),
        };
        let mut state = serializer.serialize_struct("AValue", 1)?;
        state.serialize_field("val", &val)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for AValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Val,
        }

        struct AValueVisitor;
        impl<'de> Visitor<'de> for AValueVisitor {
            type Value = AValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum AValue")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<AValue, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let val = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(AValue::Owned(val))
            }

            fn visit_map<V>(self, mut map: V) -> Result<AValue, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut val = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Val => {
                            if val.is_some() {
                                return Err(de::Error::duplicate_field("val"));
                            }
                            val = Some(map.next_value()?);
                        }
                    }
                }

                let val = val.ok_or_else(|| de::Error::missing_field("val"))?;
                Ok(AValue::Owned(val))
            }
        }

        const FIELDS: &'static [&'static str] = &["val"];
        deserializer.deserialize_struct("AValue", FIELDS, AValueVisitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn avalue_owned_ser_transitive() {
        let owned: OwnedValue = 12.0.into();
        let avalue: AValue = owned.into();

        let ser = serde_json::to_string(&avalue).unwrap();
        let deser: AValue = serde_json::from_str(&ser).unwrap();

        assert_eq!(avalue, deser);
    }

    #[test]
    fn avalue_owned_a_equal() {
        let num: Number = 12.0.into();
        let val: Value = num.clone().into();
        let owned: OwnedValue = num.clone().into();

        let a: AValue = val.into();
        let b: AValue = owned.into();

        assert_eq!(a, b);
    }

    #[test]
    fn avalue_variant_cmp_ne() {
        let num: Number = 12.0.into();
        let val: Value = num.clone().into();
        let owned: OwnedValue = num.clone().into();

        let a: AValue = val.into();
        let b: AValue = owned.into();
        let a_c = a.cmp_variants();
        let b_c = b.cmp_variants();

        assert_ne!(a_c, b_c);
    }

    #[test]
    fn avalue_variant_cmp_a_eq() {
        let num: Number = 12.0.into();
        let val1: Value = num.clone().into();
        let val2: Value = num.clone().into();

        let a: AValue = val1.into();
        let b: AValue = val2.into();
        let a_c = a.cmp_variants();
        let b_c = b.cmp_variants();

        assert_eq!(a_c, b_c);
    }

    #[test]
    fn avalue_cmp_a_eq() {
        let num: Number = 12.0.into();
        let val1: Value = num.clone().into();
        let val2: Value = num.clone().into();

        let a: AValue = val1.into();
        let b: AValue = val2.into();

        assert_eq!(a, b);
    }
}

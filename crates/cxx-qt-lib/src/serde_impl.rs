// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{QList, QListElement, QSet, QSetElement, QStringList, QVector, QVectorElement};
use cxx::ExternType;
use serde::de::{SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Formatter};
use std::marker::PhantomData;
use std::num::NonZeroIsize;

/// Serde deserializers provide an `Option<usize>` size hint, but Qt containers use signed types
/// for size. This helper function converts between the two.
/// It also returns `None` if the size hint is 0, because there's no need to reserve capacity of 0.
const fn get_size_hint(size_hint: Option<usize>) -> Option<NonZeroIsize> {
    match size_hint {
        Some(n) if n <= isize::MAX as usize => NonZeroIsize::new(n as isize),
        _ => None,
    }
}

/// Serializes and deserializes a list-like container by iterating over values.
macro_rules! seq_impl {
    ($t:ident, $element:ident, $insert:expr) => {
        impl<T> Serialize for $t<T>
        where
            T: $element + Serialize,
        {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_seq(self.iter())
            }
        }

        impl<'de, T> Deserialize<'de> for $t<T>
        where
            T: $element + Deserialize<'de> + ExternType<Kind = cxx::kind::Trivial>,
        {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct SeqVisitor<T: $element> {
                    marker: PhantomData<$t<T>>,
                }

                impl<'de, T> Visitor<'de> for SeqVisitor<T>
                where
                    T: $element + Deserialize<'de> + ExternType<Kind = cxx::kind::Trivial>,
                {
                    type Value = $t<T>;

                    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                        formatter.write_str("a sequence")
                    }

                    #[inline]
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        let mut values = Self::Value::default();
                        if let Some(size_hint) = get_size_hint(seq.size_hint()) {
                            values.reserve(size_hint.get());
                        }
                        while let Some(value) = seq.next_element()? {
                            $insert(&mut values, value);
                        }
                        Ok(values)
                    }
                }

                let visitor = SeqVisitor {
                    marker: PhantomData,
                };
                deserializer.deserialize_seq(visitor)
            }
        }
    };
}

seq_impl!(QList, QListElement, QList::append);
seq_impl!(QSet, QSetElement, QSet::insert);
seq_impl!(QVector, QVectorElement, QVector::append);

/// Like seq_impl, but for Qt classes that dereference to a container.
macro_rules! deref_impl {
    ($t:ty) => {
        impl Serialize for $t {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                (**self).serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct SeqVisitor;

                impl<'de> Visitor<'de> for SeqVisitor {
                    type Value = $t;

                    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                        formatter.write_str("a sequence")
                    }

                    #[inline]
                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: SeqAccess<'de>,
                    {
                        let mut list = Self::Value::default();
                        let values = &mut *list;
                        if let Some(size_hint) = get_size_hint(seq.size_hint()) {
                            values.reserve(size_hint.get());
                        }
                        while let Some(value) = seq.next_element()? {
                            values.append(value);
                        }
                        Ok(list)
                    }
                }

                let visitor = SeqVisitor;
                deserializer.deserialize_seq(visitor)
            }
        }
    };
}

deref_impl!(QStringList);

#[cfg(test)]
pub fn roundtrip<T>(value: &T) -> T
where
    T: Serialize + serde::de::DeserializeOwned,
{
    let serialized = serde_json::to_value(value).expect("error serializing value");
    match serde_json::from_value(serialized) {
        Ok(deserialized) => deserialized,
        Err(e) => panic!(
            "error deserializing {}: {e}",
            serde_json::to_value(value).unwrap()
        ),
    }
}

// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{
    DateFormat, QDate, QDateTime, QList, QListElement, QSet, QSetElement, QString, QStringList,
    QTime, QVector, QVectorElement,
};
use cxx::ExternType;
use serde::de::{Error as _, SeqAccess, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{self, Formatter};
use std::marker::PhantomData;
use std::num::NonZeroIsize;

/// Serializes and deserializes a time-like value using an ISO-8601 string as the intermediary.
macro_rules! datetime_impl {
    ($t:ident, $construct:expr, $expected:literal) => {
        impl Serialize for $t {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.to_format(DateFormat::ISODate).serialize(serializer)
            }
        }

        impl<'de> Deserialize<'de> for $t {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let string = <&str>::deserialize(deserializer)?;
                $construct(&QString::from(string), DateFormat::ISODate)
                    .ok_or(D::Error::invalid_value(Unexpected::Str(string), &$expected))
            }
        }
    };
}

datetime_impl!(QDate, QDate::from_string_enum, "ISO-8601 date");
datetime_impl!(QDateTime, QDateTime::from_string, "ISO-8601 datetime");
datetime_impl!(QTime, QTime::from_string_enum_opt, "ISO-8601 time");

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
                deserializer.deserialize_map(visitor)
            }
        }
    };
}

seq_impl!(QList, QListElement, QList::append);
seq_impl!(QSet, QSetElement, QSet::insert);
seq_impl!(QVector, QVectorElement, QVector::append);

/// Like seq_impl, but for Qt classes that dereference to QList.
macro_rules! list_impl {
    ($t:ty) => {
        impl Serialize for $t {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                QList::serialize(self, serializer)
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
                deserializer.deserialize_map(visitor)
            }
        }
    };
}

list_impl!(QStringList);

#[cfg(feature = "qt_gui")]
list_impl!(crate::QPolygon);
#[cfg(feature = "qt_gui")]
list_impl!(crate::QPolygonF);

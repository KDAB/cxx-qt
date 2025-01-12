// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{DateFormat, QDate, QDateTime, QString, QTime};
use serde::de::{Error as _, Unexpected};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

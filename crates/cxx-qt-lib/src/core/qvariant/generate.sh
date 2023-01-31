#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_bridge_primitive() {
    tee "$SCRIPTPATH/qvariant_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_$1"]
        fn qvariantCanConvert$2(variant: &QVariant) -> bool;
        #[rust_name = "construct_$1"]
        fn qvariantConstruct(value: &$1) -> QVariant;
        #[rust_name = "value_or_default_$1"]
        fn qvariantValueOrDefault(variant: &QVariant) -> $1;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_$1(variant)
}

pub(crate) fn construct(value: &$1) -> ffi::QVariant {
    ffi::construct_$1(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> $1 {
    ffi::value_or_default_$1(variant)
}
EOF
    rustfmt "$SCRIPTPATH/qvariant_$1.rs"
}

function generate_bridge_qt() {
    tee "$SCRIPTPATH/qvariant_$2.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/$2.h");
        type $1 = crate::$1;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_$1"]
        fn qvariantCanConvert$1(variant: &QVariant) -> bool;
        #[rust_name = "construct_$1"]
        fn qvariantConstruct(value: &$1) -> QVariant;
        #[rust_name = "value_or_default_$1"]
        fn qvariantValueOrDefault(variant: &QVariant) -> $1;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_$1(variant)
}

pub(crate) fn construct(value: &ffi::$1) -> ffi::QVariant {
    ffi::construct_$1(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::$1 {
    ffi::value_or_default_$1(variant)
}
EOF
    rustfmt "$SCRIPTPATH/qvariant_$2.rs"
}

generate_bridge_primitive "bool" "Bool"
generate_bridge_primitive "f32" "F32"
generate_bridge_primitive "f64" "F64"
generate_bridge_primitive "i8" "I8"
generate_bridge_primitive "i16" "I16"
generate_bridge_primitive "i32" "I32"
generate_bridge_primitive "i64" "I64"
generate_bridge_qt "QByteArray" "qbytearray"
generate_bridge_qt "QColor" "qcolor"
generate_bridge_qt "QDate" "qdate"
generate_bridge_qt "QDateTime" "qdatetime"
generate_bridge_qt "QModelIndex" "qmodelindex"
generate_bridge_qt "QPersistentModelIndex" "qpersistentmodelindex"
generate_bridge_qt "QPoint" "qpoint"
generate_bridge_qt "QPointF" "qpointf"
generate_bridge_qt "QRect" "qrect"
generate_bridge_qt "QRectF" "qrectf"
generate_bridge_qt "QSize" "qsize"
generate_bridge_qt "QSizeF" "qsizef"
generate_bridge_qt "QString" "qstring"
generate_bridge_qt "QStringList" "qstringlist"
generate_bridge_qt "QTime" "qtime"
generate_bridge_qt "QUrl" "qurl"
generate_bridge_primitive "u8" "U8"
generate_bridge_primitive "u16" "U16"
generate_bridge_primitive "u32" "U32"
generate_bridge_primitive "u64" "U64"

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

# $1: Unqualified rust type name, used as the suffix of the Rust monomorphized type and the C++ header.
# $2: Rust type ($1) in lowercase, used as the suffix of the Rust module name.
# $3: Qualified Rust type name, allowing the CXX bridge to be generated without additional `use` imports.
function generate_bridge_qlist() {
    tee "$SCRIPTPATH/qvariant_qlist_$2.rs" <<EOF
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Yuri Knigavko <yuri.knigavko@qt.io>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qlist/qlist_$1.h");
        type QList_$1 = crate::QList<$3>;

        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;
    }

    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        #[rust_name = "can_convert_QList_$1"]
        fn qvariantCanConvertQList_$1(variant: &QVariant) -> bool;
        #[rust_name = "construct_QList_$1"]
        fn qvariantConstruct(value: &QList_$1) -> QVariant;
        #[rust_name = "value_or_default_QList_$1"]
        fn qvariantValueOrDefault(variant: &QVariant) -> QList_$1;
    }
}

pub(crate) fn can_convert(variant: &ffi::QVariant) -> bool {
    ffi::can_convert_QList_$1(variant)
}

pub(crate) fn construct(value: &ffi::QList_$1) -> ffi::QVariant {
    ffi::construct_QList_$1(value)
}

pub(crate) fn value_or_default(variant: &ffi::QVariant) -> ffi::QList_$1 {
    ffi::value_or_default_QList_$1(variant)
}
EOF
    rustfmt "$SCRIPTPATH/qvariant_qlist_$2.rs"
}

generate_bridge_primitive "bool" "Bool"
generate_bridge_primitive "f32" "F32"
generate_bridge_primitive "f64" "F64"
generate_bridge_primitive "i8" "I8"
generate_bridge_primitive "i16" "I16"
generate_bridge_primitive "i32" "I32"
generate_bridge_primitive "i64" "I64"
generate_bridge_qt "QByteArray" "qbytearray"
generate_bridge_qt "QDate" "qdate"
generate_bridge_qt "QDateTime" "qdatetime"
generate_bridge_qt "QJsonArray" "qjsonarray"
generate_bridge_qt "QJsonObject" "qjsonobject"
generate_bridge_qt "QJsonValue" "qjsonvalue"
generate_bridge_qt "QLine" "qline"
generate_bridge_qt "QLineF" "qlinef"
generate_bridge_qt "QModelIndex" "qmodelindex"
generate_bridge_qt "QObjectMutPtr" "qobjectmutptr"
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
generate_bridge_qt "QUuid" "quuid"
generate_bridge_primitive "u8" "U8"
generate_bridge_primitive "u16" "U16"
generate_bridge_primitive "u32" "U32"
generate_bridge_primitive "u64" "U64"

generate_bridge_qt "QColor" "qcolor"
generate_bridge_qt "QFont" "qfont"
generate_bridge_qt "QImage" "qimage"
generate_bridge_qt "QPen" "qpen"
generate_bridge_qt "QPolygon" "qpolygon"
generate_bridge_qt "QPolygonF" "qpolygonf"
generate_bridge_qt "QQuaternion" "qquaternion"
generate_bridge_qt "QRegion" "qregion"
generate_bridge_qt "QVector2D" "qvector2d"
generate_bridge_qt "QVector3D" "qvector3d"
generate_bridge_qt "QVector4D" "qvector4d"

generate_bridge_qlist "bool" "bool" "bool"
generate_bridge_qlist "f32" "f32" "f32"
generate_bridge_qlist "f64" "f64" "f64"
generate_bridge_qlist "i8" "i8" "i8"
generate_bridge_qlist "i16" "i16" "i16"
generate_bridge_qlist "i32" "i32" "i32"
generate_bridge_qlist "i64" "i64" "i64"
generate_bridge_qlist "u8" "u8" "u8"
generate_bridge_qlist "u16" "u16" "u16"
generate_bridge_qlist "u32" "u32" "u32"
generate_bridge_qlist "u64" "u64" "u64"
generate_bridge_qlist "QObjectMutPtr" "qobjectmutptr" "crate::QObjectMutPtr"

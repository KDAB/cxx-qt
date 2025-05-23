#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")
#INCLUDEPATH="~/cxx-qt/crates/cxx-qt-lib/include/core/qvector"

function generate_qvector_header(){
tee "$SCRIPTPATH/../../../include/core/qvector/qvector_$1.h" <<EOF
// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This is an auto-generated file. Do not edit.
//! Edit instead: cxx-qt-lib/src/core/qvector/generate.sh
#pragma once
#include "qvector_private.h"
$3
using QVector_$1 = QVector<$2>;
EOF
}

function generate_bridge_primitive() {
    tee "$SCRIPTPATH/qvector_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//! This is an auto-generated file. Do not edit.
//! Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/core/qvector/qvector_$1.h");
        type QVector_$1 = crate::QVector<$1>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_$1);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_$1, _: &$1) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_$1"]
        fn construct(_: &QVector_$1) -> QVector_$1;
        #[rust_name = "qvector_default_$1"]
        fn construct() -> QVector_$1;
        #[rust_name = "qvector_drop_$1"]
        fn drop(_: &mut QVector_$1);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_$1"]
        fn qvectorReserve(_: &mut QVector_$1, size: isize);
        #[rust_name = "append_$1"]
        fn qvectorAppend(_: &mut QVector_$1, _: &$1);
        #[rust_name = "get_unchecked_$1"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qvectorGetUnchecked<'a>(set: &'a QVector_$1, pos: isize) -> &'a $1;
        #[rust_name = "index_of_$1"]
        fn qvectorIndexOf(_: &QVector_$1, _: &$1) -> isize;
        #[rust_name = "insert_$1"]
        fn qvectorInsert(_: &mut QVector_$1, _: isize, _: &$1);
        #[rust_name = "len_$1"]
        fn qvectorLen(_: &QVector_$1) -> isize;
        #[rust_name = "remove_$1"]
        fn qvectorRemove(_: &mut QVector_$1, _: isize);
    }
}

pub(crate) fn append(v: &mut ffi::QVector_$1, value: &$1) {
    ffi::append_$1(v, value);
}

pub(crate) fn clone(v: &ffi::QVector_$1) -> ffi::QVector_$1 {
    ffi::qvector_clone_$1(v)
}

pub(crate) fn reserve(v: &mut ffi::QVector_$1, size: isize) {
    ffi::reserve_$1(v, size);
}

pub(crate) fn default() -> ffi::QVector_$1 {
    ffi::qvector_default_$1()
}

pub(crate) fn drop(v: &mut ffi::QVector_$1) {
    ffi::qvector_drop_$1(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QVector_$1, pos: isize) -> &$1 {
    ffi::get_unchecked_$1(v, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_$1, value: &$1) -> isize {
    ffi::index_of_$1(v, value)
}

pub(crate) fn insert(v: &mut ffi::QVector_$1, pos: isize, value: &$1) {
    ffi::insert_$1(v, pos, value);
}

pub(crate) fn len(v: &ffi::QVector_$1) -> isize {
    ffi::len_$1(v)
}

pub(crate) fn remove(s: &mut ffi::QVector_$1, pos: isize) {
    ffi::remove_$1(s, pos);
}
EOF
    rustfmt "$SCRIPTPATH/qvector_$1.rs"
}

function generate_bridge_qt() {
    tee "$SCRIPTPATH/qvector_$2.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
//!This is an auto-generated file. Do not edit. Edit instead: generate.sh

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/$2.h");
        type $1 = crate::$1;

        include!("cxx-qt-lib/core/qvector/qvector_$1.h");
        type QVector_$1 = crate::QVector<$1>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QVector_$1);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QVector_$1, _: &$1) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qvector_clone_$1"]
        fn construct(_: &QVector_$1) -> QVector_$1;
        #[rust_name = "qvector_default_$1"]
        fn construct() -> QVector_$1;
        #[rust_name = "qvector_drop_$1"]
        fn drop(_: &mut QVector_$1);
    }

    #[namespace = "rust::cxxqtlib1::qvector"]
    unsafe extern "C++" {
        #[rust_name = "reserve_$1"]
        fn qvectorReserve(_: &mut QVector_$1, size: isize);
        #[rust_name = "append_$1"]
        fn qvectorAppend(_: &mut QVector_$1, _: &$1);
        #[rust_name = "get_unchecked_$1"]
        unsafe fn qvectorGetUnchecked(set: &QVector_$1, pos: isize) -> &$1;
        #[rust_name = "index_of_$1"]
        fn qvectorIndexOf(_: &QVector_$1, _: &$1) -> isize;
        #[rust_name = "insert_$1"]
        fn qvectorInsert(_: &mut QVector_$1, _: isize, _: &$1);
        #[rust_name = "remove_$1"]
        fn qvectorRemove(_: &mut QVector_$1, _: isize);
        #[rust_name = "len_$1"]
        fn qvectorLen(_: &QVector_$1) -> isize;
    }
}

pub(crate) fn append(v: &mut ffi::QVector_$1, value: &ffi::$1) {
    ffi::append_$1(v, value);
}

pub(crate) fn clone(s: &ffi::QVector_$1) -> ffi::QVector_$1 {
    ffi::qvector_clone_$1(s)
}

pub(crate) fn reserve(v: &mut ffi::QVector_$1, size: isize) {
    ffi::reserve_$1(v, size);
}

pub(crate) fn default() -> ffi::QVector_$1 {
    ffi::qvector_default_$1()
}

pub(crate) fn drop(s: &mut ffi::QVector_$1) {
    ffi::qvector_drop_$1(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QVector_$1, pos: isize) -> &ffi::$1 {
    ffi::get_unchecked_$1(s, pos)
}

pub(crate) fn index_of(v: &ffi::QVector_$1, value: &ffi::$1) -> isize {
    ffi::index_of_$1(v, value)
}

pub(crate) fn insert(s: &mut ffi::QVector_$1, pos: isize, value: &ffi::$1) {
    ffi::insert_$1(s, pos, value);
}

pub(crate) fn len(s: &ffi::QVector_$1) -> isize {
    ffi::len_$1(s)
}

pub(crate) fn remove(s: &mut ffi::QVector_$1, pos: isize) {
    ffi::remove_$1(s, pos);
}
EOF
    rustfmt "$SCRIPTPATH/qvector_$2.rs"
}

generate_qvector_header "bool" "bool" 
generate_qvector_header "f32" "float"
generate_qvector_header "f64" "double"
generate_qvector_header "i8" "::std::int8_t"
generate_qvector_header "i16" "::std::int16_t"
generate_qvector_header "i32" "::std::int32_t"
generate_qvector_header "i64" "std::int64_t"
generate_qvector_header "QByteArray" "::QByteArray" "#include <QtCore/QByteArray>"
generate_qvector_header "QDate" "::QDate" "#include <QtCore/QDate>"
generate_qvector_header "QDateTime" "::QDateTime" "#include <QtCore/QDateTime>"
generate_qvector_header "QLine" "::QLine" "#include <QtCore/QLine>"
generate_qvector_header "QLineF" "::QLineF" "#include <QtCore/QLineF>"
generate_qvector_header "QMargins" "::QMargins" "#include <QtCore/QMargins>"
generate_qvector_header "QMarginsF" "::QMarginsF" "#include <QtCore/QMarginsF>"
generate_qvector_header "QPersistentModelIndex" "::QPersistentModelIndex" "#include <QtCore/QPersistentModelIndex>"
generate_qvector_header "QPoint" "::QPoint" "#include <QtCore/QPoint>"
generate_qvector_header "QPointF" "::QPointF" "#include <QtCore/QPointF>" 
generate_qvector_header "QRect" "::QRect" "#include <QtCore/QRect>" 
generate_qvector_header "QRectF" "::QRectF" "#include <QtCore/QRectF>" 
generate_qvector_header "QSize" "::QSize" "#include <QtCore/QSize>"
generate_qvector_header "QSizeF" "::QSizeF" "#include <QtCore/QSizeF>"
generate_qvector_header "QString" "::QString" "#include <QtCore/QString>"
generate_qvector_header "QTime" "::QTime" "#include <QtCore/QTime>"
generate_qvector_header "QUuid" "::QUuid" "#include <QtCore/QUuid>"
generate_qvector_header "QUrl" "::QUrl" "#include <QtCore/QUrl>"
generate_qvector_header "QVariant" "::QVariant" "#include <QtCore/QVariant>"
generate_qvector_header "u8" "::std::uint8_t"
generate_qvector_header "u16" "::std::uint16_t"
generate_qvector_header "u32" "::std::uint32_t"
generate_qvector_header "u64" "::std::uint64_t"
generate_qvector_header "QColor" "::QColor" "#include <QtGui/QColor>"

generate_bridge_primitive "bool"
generate_bridge_primitive "f32"
generate_bridge_primitive "f64"
generate_bridge_primitive "i8"
generate_bridge_primitive "i16"
generate_bridge_primitive "i32"
generate_bridge_primitive "i64"
generate_bridge_qt "QByteArray" "qbytearray"
generate_bridge_qt "QColor" "qcolor"
generate_bridge_qt "QDate" "qdate"
generate_bridge_qt "QDateTime" "qdatetime"
generate_bridge_qt "QLine" "qline"
generate_bridge_qt "QLineF" "qlinef"
generate_bridge_qt "QMargins" "qmargins"
generate_bridge_qt "QMarginsF" "qmarginsf"
generate_bridge_qt "QPersistentModelIndex" "qpersistentmodelindex"
generate_bridge_qt "QPoint" "qpoint"
generate_bridge_qt "QPointF" "qpointf"
generate_bridge_qt "QRect" "qrect"
generate_bridge_qt "QRectF" "qrectf"
generate_bridge_qt "QSize" "qsize"
generate_bridge_qt "QSizeF" "qsizef"
generate_bridge_qt "QString" "qstring"
generate_bridge_qt "QTime" "qtime"
generate_bridge_qt "QUrl" "qurl"
generate_bridge_qt "QUuid" "quuid"
generate_bridge_qt "QVariant" "qvariant"
generate_bridge_primitive "u8"
generate_bridge_primitive "u16"
generate_bridge_primitive "u32"
generate_bridge_primitive "u64"

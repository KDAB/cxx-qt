#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_qlist_header(){
tee "$SCRIPTPATH/../../../include/core/qlist/qlist_$1.h" <<EOF
// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This is an auto-generated file. Do not edit.
//! Edit instead: cxx-qt-lib/src/core/qlist/generate.sh
#pragma once
#include "qlist_private.h"
$3
using QList_$1 = QList<$2>;
EOF
}

function generate_bridge_primitive() {
    tee "$SCRIPTPATH/qlist_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlist_$1.h");
        type QList_$1 = crate::QList<$1>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_$1);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_$1, _: &$1) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_$1"]
        fn construct(_: &QList_$1) -> QList_$1;
        #[rust_name = "qlist_default_$1"]
        fn construct() -> QList_$1;
        #[rust_name = "qlist_drop_$1"]
        fn drop(_: &mut QList_$1);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_$1"]
        fn qlistReserve(_: &mut QList_$1, size: isize);
        #[rust_name = "append_$1"]
        fn qlistAppend(_: &mut QList_$1, _: &$1);
        #[rust_name = "get_unchecked_$1"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qlistGetUnchecked<'a>(set: &'a QList_$1, pos: isize) -> &'a $1;
        #[rust_name = "index_of_$1"]
        fn qlistIndexOf(_: &QList_$1, _: &$1) -> isize;
        #[rust_name = "insert_$1"]
        fn qlistInsert(_: &mut QList_$1, _: isize, _: &$1);
        #[rust_name = "len_$1"]
        fn qlistLen(_: &QList_$1) -> isize;
        #[rust_name = "remove_$1"]
        fn qlistRemove(_: &mut QList_$1, _: isize);
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_$1, size: isize) {
    ffi::reserve_$1(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_$1, value: &$1) {
    ffi::append_$1(v, value);
}

pub(crate) fn clone(v: &ffi::QList_$1) -> ffi::QList_$1 {
    ffi::qlist_clone_$1(v)
}

pub(crate) fn default() -> ffi::QList_$1 {
    ffi::qlist_default_$1()
}

pub(crate) fn drop(v: &mut ffi::QList_$1) {
    ffi::qlist_drop_$1(v);
}

pub(crate) unsafe fn get_unchecked(v: &ffi::QList_$1, pos: isize) -> &$1 {
    ffi::get_unchecked_$1(v, pos)
}

pub(crate) fn index_of(v: &ffi::QList_$1, value: &$1) -> isize {
    ffi::index_of_$1(v, value)
}

pub(crate) fn insert(v: &mut ffi::QList_$1, pos: isize, value: &$1) {
    ffi::insert_$1(v, pos, value);
}

pub(crate) fn len(v: &ffi::QList_$1) -> isize {
    ffi::len_$1(v)
}

pub(crate) fn remove(s: &mut ffi::QList_$1, pos: isize) {
    ffi::remove_$1(s, pos);
}
EOF
    rustfmt "$SCRIPTPATH/qlist_$1.rs"
}

function generate_bridge_qt() {
    tee "$SCRIPTPATH/qlist_$2.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/$2.h");
        type $1 = crate::$1;

        include!("cxx-qt-lib/qlist_$1.h");
        type QList_$1 = crate::QList<$1>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QList_$1);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QList_$1, _: &$1) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qlist_clone_$1"]
        fn construct(_: &QList_$1) -> QList_$1;
        #[rust_name = "qlist_default_$1"]
        fn construct() -> QList_$1;
        #[rust_name = "qlist_drop_$1"]
        fn drop(_: &mut QList_$1);
    }

    #[namespace = "rust::cxxqtlib1::qlist"]
    unsafe extern "C++" {
        #[rust_name = "reserve_$1"]
        fn qlistReserve(_: &mut QList_$1, size: isize);
        #[rust_name = "append_$1"]
        fn qlistAppend(_: &mut QList_$1, _: &$1);
        #[rust_name = "get_unchecked_$1"]
        unsafe fn qlistGetUnchecked(set: &QList_$1, pos: isize) -> &$1;
        #[rust_name = "index_of_$1"]
        fn qlistIndexOf(_: &QList_$1, _: &$1) -> isize;
        #[rust_name = "insert_$1"]
        fn qlistInsert(_: &mut QList_$1, _: isize, _: &$1);
        #[rust_name = "remove_$1"]
        fn qlistRemove(_: &mut QList_$1, _: isize);
        #[rust_name = "len_$1"]
        fn qlistLen(_: &QList_$1) -> isize;
    }
}

pub(crate) fn reserve(v: &mut ffi::QList_$1, size: isize) {
    ffi::reserve_$1(v, size);
}

pub(crate) fn append(v: &mut ffi::QList_$1, value: &ffi::$1) {
    ffi::append_$1(v, value);
}

pub(crate) fn clone(s: &ffi::QList_$1) -> ffi::QList_$1 {
    ffi::qlist_clone_$1(s)
}

pub(crate) fn default() -> ffi::QList_$1 {
    ffi::qlist_default_$1()
}

pub(crate) fn drop(s: &mut ffi::QList_$1) {
    ffi::qlist_drop_$1(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QList_$1, pos: isize) -> &ffi::$1 {
    ffi::get_unchecked_$1(s, pos)
}

pub(crate) fn index_of(v: &ffi::QList_$1, value: &ffi::$1) -> isize {
    ffi::index_of_$1(v, value)
}

pub(crate) fn insert(s: &mut ffi::QList_$1, pos: isize, value: &ffi::$1) {
    ffi::insert_$1(s, pos, value);
}

pub(crate) fn len(s: &ffi::QList_$1) -> isize {
    ffi::len_$1(s)
}

pub(crate) fn remove(s: &mut ffi::QList_$1, pos: isize) {
    ffi::remove_$1(s, pos);
}
EOF
    rustfmt "$SCRIPTPATH/qlist_$2.rs"
}

generate_qlist_header "bool" "bool" 
generate_qlist_header "f32" "float"
generate_qlist_header "f64" "double"
generate_qlist_header "i8" "::std::int8_t"
generate_qlist_header "i16" "::std::int16_t"
generate_qlist_header "i32" "::std::int32_t"
generate_qlist_header "i64" "std::int64_t"
generate_qlist_header "QByteArray" "::QByteArray" "#include <QtCore/QByteArray>"
generate_qlist_header "QDate" "::QDate" "#include <QtCore/QDate>"
generate_qlist_header "QDateTime" "::QDateTime" "#include <QtCore/QDateTime>"
generate_qlist_header "QLine" "::QLine" "#include <QtCore/QLine>"
generate_qlist_header "QLineF" "::QLineF" "#include <QtCore/QLineF>"
generate_qlist_header "QMargins" "::QMargins" "#include <QtCore/QMargins>"
generate_qlist_header "QMarginsF" "::QMarginsF" "#include <QtCore/QMarginsF>"
generate_qlist_header "QPersistentModelIndex" "::QPersistentModelIndex" "#include <QtCore/QPersistentModelIndex>"
generate_qlist_header "QPoint" "::QPoint" "#include <QtCore/QPoint>"
generate_qlist_header "QPointF" "::QPointF" "#include <QtCore/QPointF>" 
generate_qlist_header "QRect" "::QRect" "#include <QtCore/QRect>" 
generate_qlist_header "QRectF" "::QRectF" "#include <QtCore/QRectF>" 
generate_qlist_header "QSize" "::QSize" "#include <QtCore/QSize>"
generate_qlist_header "QSizeF" "::QSizeF" "#include <QtCore/QSizeF>"
generate_qlist_header "QString" "::QString" "#include <QtCore/QString>"
generate_qlist_header "QTime" "::QTime" "#include <QtCore/QTime>"
generate_qlist_header "QUuid" "::QUuid" "#include <QtCore/QUuid>"
generate_qlist_header "QUrl" "::QUrl" "#include <QtCore/QUrl>"
generate_qlist_header "QVariant" "::QVariant" "#include <QtCore/QVariant>"
generate_qlist_header "u8" "::std::uint8_t"
generate_qlist_header "u16" "::std::uint16_t"
generate_qlist_header "u32" "::std::uint32_t"
generate_qlist_header "u64" "::std::uint64_t"
generate_qlist_header "QColor" "::QColor" "#include <QtGui/QColor>"

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
generate_bridge_qt "QLine" "qline"
generate_bridge_qt "QLineF" "qlinef"
generate_bridge_primitive "u8"
generate_bridge_primitive "u16"
generate_bridge_primitive "u32"
generate_bridge_primitive "u64"

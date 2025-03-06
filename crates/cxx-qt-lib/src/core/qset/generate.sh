#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_qset_header(){
tee "$SCRIPTPATH/../../../include/core/qset/qset_$1.h" <<EOF
// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This is an auto-generated file. Do not edit.
//! Edit instead: cxx-qt-lib/src/core/qset/generate.sh
#pragma once
#include "qset_private.h"
$3
using QSet_$1 = QSet<$2>;
EOF
}

function generate_bridge_primitive() {
    tee "$SCRIPTPATH/qset_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset_$1.h");
        type QSet_$1 = crate::QSet<$1>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_$1);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_$1, _: &$1) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_$1, _: &$1) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_$1"]
        fn construct(_: &QSet_$1) -> QSet_$1;
        #[rust_name = "qset_default_$1"]
        fn construct() -> QSet_$1;
        #[rust_name = "qset_drop_$1"]
        fn drop(_: &mut QSet_$1);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_$1"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qsetGetUnchecked<'a>(set: &'a QSet_$1, pos: isize) -> &'a $1;
        #[rust_name = "insert_$1"]
        fn qsetInsert(_: &mut QSet_$1, _: &$1);
        #[rust_name = "len_$1"]
        fn qsetLen(_: &QSet_$1) -> isize;
        #[rust_name = "reserve_$1"]
        fn qsetReserve(_: &mut QSet_$1, size: isize);
    }
}

pub(crate) fn clone(s: &ffi::QSet_$1) -> ffi::QSet_$1 {
    ffi::qset_clone_$1(s)
}

pub(crate) fn default() -> ffi::QSet_$1 {
    ffi::qset_default_$1()
}

pub(crate) fn drop(s: &mut ffi::QSet_$1) {
    ffi::qset_drop_$1(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_$1, pos: isize) -> &$1 {
    ffi::get_unchecked_$1(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_$1, value: &$1) {
    ffi::insert_$1(s, value);
}

pub(crate) fn len(s: &ffi::QSet_$1) -> isize {
    ffi::len_$1(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_$1, size: isize) {
  ffi::reserve_$1(s, size);
}
EOF
    rustfmt "$SCRIPTPATH/qset_$1.rs"
}

function generate_bridge_qt() {
    tee "$SCRIPTPATH/qset_$2.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/$2.h");
        type $1 = crate::$1;

        include!("cxx-qt-lib/qset_$1.h");
        type QSet_$1 = crate::QSet<$1>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_$1);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_$1, _: &$1) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_$1, _: &$1) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qset_clone_$1"]
        fn construct(_: &QSet_$1) -> QSet_$1;
        #[rust_name = "qset_default_$1"]
        fn construct() -> QSet_$1;
        #[rust_name = "qset_drop_$1"]
        fn drop(_: &mut QSet_$1);
    }

    #[namespace = "rust::cxxqtlib1::qset"]
    unsafe extern "C++" {
        #[rust_name = "get_unchecked_$1"]
        unsafe fn qsetGetUnchecked(set: &QSet_$1, pos: isize) -> &$1;
        #[rust_name = "insert_$1"]
        fn qsetInsert(_: &mut QSet_$1, _: &$1);
        #[rust_name = "len_$1"]
        fn qsetLen(_: &QSet_$1) -> isize;
        #[rust_name = "reserve_$1"]
        fn qsetReserve(_: &mut QSet_$1, size: isize);
    }
}

pub(crate) fn clone(s: &ffi::QSet_$1) -> ffi::QSet_$1 {
    ffi::qset_clone_$1(s)
}

pub(crate) fn default() -> ffi::QSet_$1 {
    ffi::qset_default_$1()
}

pub(crate) fn drop(s: &mut ffi::QSet_$1) {
    ffi::qset_drop_$1(s);
}

pub(crate) unsafe fn get_unchecked(s: &ffi::QSet_$1, pos: isize) -> &ffi::$1 {
    ffi::get_unchecked_$1(s, pos)
}

pub(crate) fn insert(s: &mut ffi::QSet_$1, value: &ffi::$1) {
    ffi::insert_$1(s, value);
}

pub(crate) fn len(s: &ffi::QSet_$1) -> isize {
    ffi::len_$1(s)
}

pub(crate) fn reserve(s: &mut ffi::QSet_$1, size: isize) {
  ffi::reserve_$1(s, size);
}
EOF
    rustfmt "$SCRIPTPATH/qset_$2.rs"
}

generate_qset_header "bool" "bool" 
generate_qset_header "f32" "float"
generate_qset_header "f64" "double"
generate_qset_header "i8" "::std::int8_t"
generate_qset_header "i16" "::std::int16_t"
generate_qset_header "i32" "::std::int32_t"
generate_qset_header "i64" "std::int64_t"
generate_qset_header "QByteArray" "::QByteArray" "#include <QtCore/QByteArray>"
generate_qset_header "QDate" "::QDate" "#include <QtCore/QDate>"
generate_qset_header "QDateTime" "::QDateTime" "#include <QtCore/QDateTime>"
generate_qset_header "QPersistentModelIndex" "::QPersistentModelIndex" "#include <QtCore/QPersistentModelIndex>"
generate_qset_header "QString" "::QString" "#include <QtCore/QString>"
generate_qset_header "QTime" "::QTime" "#include <QtCore/QTime>"
generate_qset_header "QUuid" "::QUuid" "#include <QtCore/QUuid>"
generate_qset_header "QUrl" "::QUrl" "#include <QtCore/QUrl>"
generate_qset_header "u8" "::std::uint8_t"
generate_qset_header "u16" "::std::uint16_t"
generate_qset_header "u32" "::std::uint32_t"
generate_qset_header "u64" "::std::uint64_t"

generate_bridge_primitive "bool"
generate_bridge_primitive "f32"
generate_bridge_primitive "f64"
generate_bridge_primitive "i8"
generate_bridge_primitive "i16"
generate_bridge_primitive "i32"
generate_bridge_primitive "i64"
generate_bridge_qt "QByteArray" "qbytearray"
generate_bridge_qt "QDate" "qdate"
generate_bridge_qt "QDateTime" "qdatetime"
generate_bridge_qt "QPersistentModelIndex" "qpersistentmodelindex"
generate_bridge_qt "QString" "qstring"
generate_bridge_qt "QTime" "qtime"
generate_bridge_qt "QUrl" "qurl"
generate_bridge_qt "QUuid" "quuid"
generate_bridge_primitive "u8"
generate_bridge_primitive "u16"
generate_bridge_primitive "u32"
generate_bridge_primitive "u64"

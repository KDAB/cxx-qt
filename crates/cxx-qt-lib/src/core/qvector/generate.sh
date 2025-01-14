#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_bridge_primitive() {
    tee "$SCRIPTPATH/qvector_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvector.h");
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

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/$2.h");
        type $1 = crate::$1;

        include!("cxx-qt-lib/qvector.h");
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

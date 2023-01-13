#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_bridge_primitive() {
    tee "$SCRIPTPATH/qlist_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlist.h");
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

        include!("cxx-qt-lib/qlist.h");
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
generate_bridge_qt "QPoint" "qpoint"
generate_bridge_qt "QPointF" "qpointf"
generate_bridge_qt "QRect" "qrect"
generate_bridge_qt "QRectF" "qrectf"
generate_bridge_qt "QSize" "qsize"
generate_bridge_qt "QSizeF" "qsizef"
generate_bridge_qt "QString" "qstring"
generate_bridge_qt "QTime" "qtime"
generate_bridge_qt "QUrl" "qurl"
generate_bridge_qt "QVariant" "qvariant"
generate_bridge_primitive "u8"
generate_bridge_primitive "u16"
generate_bridge_primitive "u32"
generate_bridge_primitive "u64"

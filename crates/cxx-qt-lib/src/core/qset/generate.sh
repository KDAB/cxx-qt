#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_bridge_primitive() {
    tee "$SCRIPTPATH/qset_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
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

        include!("cxx-qt-lib/qset.h");
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
EOF
    rustfmt "$SCRIPTPATH/qset_$2.rs"
}

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

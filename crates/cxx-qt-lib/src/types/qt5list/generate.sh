#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_bridge_primitive() {
    tee "$SCRIPTPATH/qt5list_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt5list.h");
        type Qt5List_$1 = crate::Qt5List<$1>;
    }

    unsafe extern "C++" {
        /// # Safety
        ///
        /// Calling this method with an out-of-bounds index is undefined behavior
        /// even if the resulting reference is not used.
        #[rust_name = "cxx_get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn at<'a>(self: &'a Qt5List_$1, pos: i32) -> &'a $1;
        #[rust_name = "cxx_append"]
        fn append(self: &mut Qt5List_$1, _: &$1);
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut Qt5List_$1);
        #[rust_name = "cxx_contains"]
        fn contains(self: &Qt5List_$1, _: &$1) -> bool;
        #[rust_name = "cxx_index_of"]
        fn indexOf(self: &Qt5List_$1, _: &$1, from: i32) -> i32;
        #[rust_name = "cxx_insert"]
        fn insert(self: &mut Qt5List_$1, _: i32, _: &$1);
        #[rust_name = "cxx_len"]
        fn length(self: &Qt5List_$1) -> i32;
        #[rust_name = "cxx_remove"]
        fn removeAt(self: &mut Qt5List_$1, _: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qt5list_clone_$1"]
        fn construct(_: &Qt5List_$1) -> Qt5List_$1;
        #[rust_name = "qt5list_default_$1"]
        fn construct() -> Qt5List_$1;
        #[rust_name = "qt5list_drop_$1"]
        fn drop(_: &mut Qt5List_$1);
    }
}

pub(crate) fn clone(v: &ffi::Qt5List_$1) -> ffi::Qt5List_$1 {
    ffi::qt5list_clone_$1(v)
}

pub(crate) fn default() -> ffi::Qt5List_$1 {
    ffi::qt5list_default_$1()
}

pub(crate) fn drop(v: &mut ffi::Qt5List_$1) {
    ffi::qt5list_drop_$1(v);
}
EOF
    rustfmt "$SCRIPTPATH/qt5list_$1.rs"
}

function generate_bridge_qt() {
    tee "$SCRIPTPATH/qt5list_$2.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/$2.h");
        type $1 = crate::$1;

        include!("cxx-qt-lib/qt5list.h");
        type Qt5List_$1 = crate::Qt5List<$1>;
    }

    unsafe extern "C++" {
        /// # Safety
        ///
        /// Calling this method with an out-of-bounds index is undefined behavior
        /// even if the resulting reference is not used.
        #[rust_name = "cxx_get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn at<'a>(self: &'a Qt5List_$1, pos: i32) -> &'a $1;
        #[rust_name = "cxx_append"]
        fn append(self: &mut Qt5List_$1, _: &$1);
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut Qt5List_$1);
        #[rust_name = "cxx_contains"]
        fn contains(self: &Qt5List_$1, _: &$1) -> bool;
        #[rust_name = "cxx_index_of"]
        fn indexOf(self: &Qt5List_$1, _: &$1, from: i32) -> i32;
        #[rust_name = "cxx_insert"]
        fn insert(self: &mut Qt5List_$1, _: i32, _: &$1);
        #[rust_name = "cxx_len"]
        fn length(self: &Qt5List_$1) -> i32;
        #[rust_name = "cxx_remove"]
        fn removeAt(self: &mut Qt5List_$1, _: i32);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qt5list_clone_$1"]
        fn construct(_: &Qt5List_$1) -> Qt5List_$1;
        #[rust_name = "qt5list_default_$1"]
        fn construct() -> Qt5List_$1;
        #[rust_name = "qt5list_drop_$1"]
        fn drop(_: &mut Qt5List_$1);
    }
}

pub(crate) fn clone(s: &ffi::Qt5List_$1) -> ffi::Qt5List_$1 {
    ffi::qt5list_clone_$1(s)
}

pub(crate) fn default() -> ffi::Qt5List_$1 {
    ffi::qt5list_default_$1()
}

pub(crate) fn drop(s: &mut ffi::Qt5List_$1) {
    ffi::qt5list_drop_$1(s);
}
EOF
    rustfmt "$SCRIPTPATH/qt5list_$2.rs"
}

generate_bridge_primitive "bool"
generate_bridge_primitive "f32"
generate_bridge_primitive "f64"
generate_bridge_primitive "i8"
generate_bridge_primitive "i16"
generate_bridge_primitive "i32"
generate_bridge_primitive "i64"
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

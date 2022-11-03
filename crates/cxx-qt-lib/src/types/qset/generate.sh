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

    #[namespace = "rust::cxxqtlib1::qset_$1"]
    unsafe extern "C++" {
        #[rust_name = "clear"]
        fn qset_clear_$1(_: &mut QSet_$1);
        #[rust_name = "clone"]
        fn qset_clone_$1(_: &QSet_$1) -> QSet_$1;
        #[rust_name = "contains"]
        fn qset_contains_$1(_: &QSet_$1, _: &$1) -> bool;
        #[rust_name = "default"]
        fn qset_default_$1() -> QSet_$1;
        #[rust_name = "drop"]
        fn qset_drop_$1(_: &mut QSet_$1);
        #[rust_name = "get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked_$1<'a>(set: &'a QSet_$1, pos: usize) -> &'a $1;
        #[rust_name = "insert"]
        fn qset_insert_$1(_: &mut QSet_$1, _: &$1);
        #[rust_name = "len"]
        fn qset_len_$1(_: &QSet_$1) -> usize;
        #[rust_name = "remove"]
        fn qset_remove_$1(_: &mut QSet_$1, _: &$1) -> bool;
    }
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

    #[namespace = "rust::cxxqtlib1::qset_$1"]
    unsafe extern "C++" {
        #[rust_name = "clear"]
        fn qset_clear_$1(_: &mut QSet_$1);
        #[rust_name = "clone"]
        fn qset_clone_$1(_: &QSet_$1) -> QSet_$1;
        #[rust_name = "contains"]
        fn qset_contains_$1(_: &QSet_$1, _: &$1) -> bool;
        #[rust_name = "default"]
        fn qset_default_$1() -> QSet_$1;
        #[rust_name = "drop"]
        fn qset_drop_$1(_: &mut QSet_$1);
        #[rust_name = "get_unchecked"]
        unsafe fn qset_get_unchecked_$1(set: &QSet_$1, pos: usize) -> &$1;
        #[rust_name = "insert"]
        fn qset_insert_$1(_: &mut QSet_$1, _: &$1);
        #[rust_name = "len"]
        fn qset_len_$1(_: &QSet_$1) -> usize;
        #[rust_name = "remove"]
        fn qset_remove_$1(_: &mut QSet_$1, _: &$1) -> bool;
    }
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
generate_bridge_qt "QDate" "qdate"
generate_bridge_qt "QDateTime" "qdatetime"
generate_bridge_qt "QString" "qstring"
generate_bridge_qt "QTime" "qtime"
generate_bridge_qt "QUrl" "qurl"
generate_bridge_primitive "u8"
generate_bridge_primitive "u16"
generate_bridge_primitive "u32"

// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qset.h");
        type QSet_QString = crate::QSet<QString>;
    }

    #[namespace = "rust::cxxqtlib1::qset_QString"]
    unsafe extern "C++" {
        #[rust_name = "clear"]
        fn qset_clear_QString(_: &mut QSet_QString);
        #[rust_name = "clone"]
        fn qset_clone_QString(_: &QSet_QString) -> QSet_QString;
        #[rust_name = "contains"]
        fn qset_contains_QString(_: &QSet_QString, _: &QString) -> bool;
        #[rust_name = "default"]
        fn qset_default_QString() -> QSet_QString;
        #[rust_name = "drop"]
        fn qset_drop_QString(_: &mut QSet_QString);
        #[rust_name = "get_unchecked"]
        unsafe fn qset_get_unchecked_QString(set: &QSet_QString, pos: usize) -> &QString;
        #[rust_name = "insert"]
        fn qset_insert_QString(_: &mut QSet_QString, _: &QString);
        #[rust_name = "len"]
        fn qset_len_QString(_: &QSet_QString) -> usize;
        #[rust_name = "remove"]
        fn qset_remove_QString(_: &mut QSet_QString, _: &QString) -> bool;
    }
}

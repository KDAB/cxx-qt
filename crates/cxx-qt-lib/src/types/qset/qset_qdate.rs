// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdate.h");
        type QDate = crate::QDate;

        include!("cxx-qt-lib/qset.h");
        type QSet_QDate = crate::QSet<QDate>;
    }

    #[namespace = "rust::cxxqtlib1::qset_QDate"]
    unsafe extern "C++" {
        #[rust_name = "clear"]
        fn qset_clear_QDate(_: &mut QSet_QDate);
        #[rust_name = "clone"]
        fn qset_clone_QDate(_: &QSet_QDate) -> QSet_QDate;
        #[rust_name = "contains"]
        fn qset_contains_QDate(_: &QSet_QDate, _: &QDate) -> bool;
        #[rust_name = "default"]
        fn qset_default_QDate() -> QSet_QDate;
        #[rust_name = "drop"]
        fn qset_drop_QDate(_: &mut QSet_QDate);
        #[rust_name = "get_unchecked"]
        unsafe fn qset_get_unchecked_QDate(set: &QSet_QDate, pos: usize) -> &QDate;
        #[rust_name = "insert"]
        fn qset_insert_QDate(_: &mut QSet_QDate, _: &QDate);
        #[rust_name = "len"]
        fn qset_len_QDate(_: &QSet_QDate) -> usize;
        #[rust_name = "remove"]
        fn qset_remove_QDate(_: &mut QSet_QDate, _: &QDate) -> bool;
    }
}

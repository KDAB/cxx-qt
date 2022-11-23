// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qdatetime.h");
        type QDateTime = crate::QDateTime;

        include!("cxx-qt-lib/qset.h");
        type QSet_QDateTime = crate::QSet<QDateTime>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QDateTime);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QDateTime, _: &QDateTime) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QDateTime, _: &QDateTime) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qset_QDateTime"]
    unsafe extern "C++" {
        #[rust_name = "clone"]
        fn qset_clone_QDateTime(_: &QSet_QDateTime) -> QSet_QDateTime;
        #[rust_name = "default"]
        fn qset_default_QDateTime() -> QSet_QDateTime;
        #[rust_name = "drop"]
        fn qset_drop_QDateTime(_: &mut QSet_QDateTime);
        #[rust_name = "get_unchecked"]
        unsafe fn qset_get_unchecked_QDateTime(set: &QSet_QDateTime, pos: usize) -> &QDateTime;
        #[rust_name = "insert"]
        fn qset_insert_QDateTime(_: &mut QSet_QDateTime, _: &QDateTime);
        #[rust_name = "len"]
        fn qset_len_QDateTime(_: &QSet_QDateTime) -> usize;
    }
}

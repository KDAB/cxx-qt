// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qurl.h");
        type QUrl = crate::QUrl;

        include!("cxx-qt-lib/qset.h");
        type QSet_QUrl = crate::QSet<QUrl>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_QUrl);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_QUrl, _: &QUrl) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_QUrl, _: &QUrl) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qset_QUrl"]
    unsafe extern "C++" {
        #[rust_name = "clone"]
        fn qset_clone_QUrl(_: &QSet_QUrl) -> QSet_QUrl;
        #[rust_name = "default"]
        fn qset_default_QUrl() -> QSet_QUrl;
        #[rust_name = "drop"]
        fn qset_drop_QUrl(_: &mut QSet_QUrl);
        #[rust_name = "get_unchecked"]
        unsafe fn qset_get_unchecked_QUrl(set: &QSet_QUrl, pos: usize) -> &QUrl;
        #[rust_name = "insert"]
        fn qset_insert_QUrl(_: &mut QSet_QUrl, _: &QUrl);
        #[rust_name = "len"]
        fn qset_len_QUrl(_: &QSet_QUrl) -> usize;
    }
}

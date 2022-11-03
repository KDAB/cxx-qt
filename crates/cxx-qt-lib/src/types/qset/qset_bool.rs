// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_bool = crate::QSet<bool>;
    }

    #[namespace = "rust::cxxqtlib1::qset_bool"]
    unsafe extern "C++" {
        #[rust_name = "clear"]
        fn qset_clear_bool(_: &mut QSet_bool);
        #[rust_name = "clone"]
        fn qset_clone_bool(_: &QSet_bool) -> QSet_bool;
        #[rust_name = "contains"]
        fn qset_contains_bool(_: &QSet_bool, _: &bool) -> bool;
        #[rust_name = "default"]
        fn qset_default_bool() -> QSet_bool;
        #[rust_name = "drop"]
        fn qset_drop_bool(_: &mut QSet_bool);
        #[rust_name = "get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked_bool<'a>(set: &'a QSet_bool, pos: usize) -> &'a bool;
        #[rust_name = "insert"]
        fn qset_insert_bool(_: &mut QSet_bool, _: &bool);
        #[rust_name = "len"]
        fn qset_len_bool(_: &QSet_bool) -> usize;
        #[rust_name = "remove"]
        fn qset_remove_bool(_: &mut QSet_bool, _: &bool) -> bool;
    }
}

// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_i8 = crate::QSet<i8>;
    }

    #[namespace = "rust::cxxqtlib1::qset_i8"]
    unsafe extern "C++" {
        #[rust_name = "clear"]
        fn qset_clear_i8(_: &mut QSet_i8);
        #[rust_name = "clone"]
        fn qset_clone_i8(_: &QSet_i8) -> QSet_i8;
        #[rust_name = "contains"]
        fn qset_contains_i8(_: &QSet_i8, _: &i8) -> bool;
        #[rust_name = "default"]
        fn qset_default_i8() -> QSet_i8;
        #[rust_name = "drop"]
        fn qset_drop_i8(_: &mut QSet_i8);
        #[rust_name = "get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked_i8<'a>(set: &'a QSet_i8, pos: usize) -> &'a i8;
        #[rust_name = "insert"]
        fn qset_insert_i8(_: &mut QSet_i8, _: &i8);
        #[rust_name = "len"]
        fn qset_len_i8(_: &QSet_i8) -> usize;
        #[rust_name = "remove"]
        fn qset_remove_i8(_: &mut QSet_i8, _: &i8) -> bool;
    }
}

// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_u8 = crate::QSet<u8>;
    }

    #[namespace = "rust::cxxqtlib1::qset_u8"]
    unsafe extern "C++" {
        #[rust_name = "clear"]
        fn qset_clear_u8(_: &mut QSet_u8);
        #[rust_name = "clone"]
        fn qset_clone_u8(_: &QSet_u8) -> QSet_u8;
        #[rust_name = "contains"]
        fn qset_contains_u8(_: &QSet_u8, _: &u8) -> bool;
        #[rust_name = "default"]
        fn qset_default_u8() -> QSet_u8;
        #[rust_name = "drop"]
        fn qset_drop_u8(_: &mut QSet_u8);
        #[rust_name = "get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked_u8<'a>(set: &'a QSet_u8, pos: usize) -> &'a u8;
        #[rust_name = "insert"]
        fn qset_insert_u8(_: &mut QSet_u8, _: &u8);
        #[rust_name = "len"]
        fn qset_len_u8(_: &QSet_u8) -> usize;
        #[rust_name = "remove"]
        fn qset_remove_u8(_: &mut QSet_u8, _: &u8) -> bool;
    }
}

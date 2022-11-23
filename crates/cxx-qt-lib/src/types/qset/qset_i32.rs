// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_i32 = crate::QSet<i32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_i32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_i32, _: &i32) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_i32, _: &i32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qset_i32"]
    unsafe extern "C++" {
        #[rust_name = "clone"]
        fn qset_clone_i32(_: &QSet_i32) -> QSet_i32;
        #[rust_name = "default"]
        fn qset_default_i32() -> QSet_i32;
        #[rust_name = "drop"]
        fn qset_drop_i32(_: &mut QSet_i32);
        #[rust_name = "get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked_i32<'a>(set: &'a QSet_i32, pos: usize) -> &'a i32;
        #[rust_name = "insert"]
        fn qset_insert_i32(_: &mut QSet_i32, _: &i32);
        #[rust_name = "len"]
        fn qset_len_i32(_: &QSet_i32) -> usize;
    }
}

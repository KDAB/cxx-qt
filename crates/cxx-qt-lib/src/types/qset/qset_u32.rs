// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_u32 = crate::QSet<u32>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_u32);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_u32, _: &u32) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_u32, _: &u32) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qset_u32"]
    unsafe extern "C++" {
        #[rust_name = "clone"]
        fn qset_clone_u32(_: &QSet_u32) -> QSet_u32;
        #[rust_name = "default"]
        fn qset_default_u32() -> QSet_u32;
        #[rust_name = "drop"]
        fn qset_drop_u32(_: &mut QSet_u32);
        #[rust_name = "get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked_u32<'a>(set: &'a QSet_u32, pos: usize) -> &'a u32;
        #[rust_name = "insert"]
        fn qset_insert_u32(_: &mut QSet_u32, _: &u32);
        #[rust_name = "len"]
        fn qset_len_u32(_: &QSet_u32) -> usize;
    }
}

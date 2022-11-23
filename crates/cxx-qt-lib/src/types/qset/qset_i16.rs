// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_i16 = crate::QSet<i16>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QSet_i16);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QSet_i16, _: &i16) -> bool;
        #[rust_name = "cxx_remove"]
        fn remove(self: &mut QSet_i16, _: &i16) -> bool;
    }

    #[namespace = "rust::cxxqtlib1::qset_i16"]
    unsafe extern "C++" {
        #[rust_name = "clone"]
        fn qset_clone_i16(_: &QSet_i16) -> QSet_i16;
        #[rust_name = "default"]
        fn qset_default_i16() -> QSet_i16;
        #[rust_name = "drop"]
        fn qset_drop_i16(_: &mut QSet_i16);
        #[rust_name = "get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked_i16<'a>(set: &'a QSet_i16, pos: usize) -> &'a i16;
        #[rust_name = "insert"]
        fn qset_insert_i16(_: &mut QSet_i16, _: &i16);
        #[rust_name = "len"]
        fn qset_len_i16(_: &QSet_i16) -> usize;
    }
}

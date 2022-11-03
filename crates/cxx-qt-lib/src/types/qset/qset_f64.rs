// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qset.h");
        type QSet_f64 = crate::QSet<f64>;
    }

    #[namespace = "rust::cxxqtlib1::qset_f64"]
    unsafe extern "C++" {
        #[rust_name = "clear"]
        fn qset_clear_f64(_: &mut QSet_f64);
        #[rust_name = "clone"]
        fn qset_clone_f64(_: &QSet_f64) -> QSet_f64;
        #[rust_name = "contains"]
        fn qset_contains_f64(_: &QSet_f64, _: &f64) -> bool;
        #[rust_name = "default"]
        fn qset_default_f64() -> QSet_f64;
        #[rust_name = "drop"]
        fn qset_drop_f64(_: &mut QSet_f64);
        #[rust_name = "get_unchecked"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qset_get_unchecked_f64<'a>(set: &'a QSet_f64, pos: usize) -> &'a f64;
        #[rust_name = "insert"]
        fn qset_insert_f64(_: &mut QSet_f64, _: &f64);
        #[rust_name = "len"]
        fn qset_len_f64(_: &QSet_f64) -> usize;
        #[rust_name = "remove"]
        fn qset_remove_f64(_: &mut QSet_f64, _: &f64) -> bool;
    }
}

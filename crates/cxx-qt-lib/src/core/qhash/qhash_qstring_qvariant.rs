// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = crate::QVariant;

        include!("cxx-qt-lib/qhash_QString_QVariant.h");
        type QHash_QString_QVariant = crate::QHash<super::QHashPair_QString_QVariant>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut QHash_QString_QVariant);
        #[rust_name = "cxx_contains"]
        fn contains(self: &QHash_QString_QVariant, key: &QString) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhash_clone_QString_QVariant"]
        fn construct(_: &QHash_QString_QVariant) -> QHash_QString_QVariant;
        #[rust_name = "qhash_default_QString_QVariant"]
        fn construct() -> QHash_QString_QVariant;
        #[rust_name = "qhash_drop_QString_QVariant"]
        fn drop(_: &mut QHash_QString_QVariant);
    }

    #[namespace = "rust::cxxqtlib1::qhash"]
    unsafe extern "C++" {
        #[rust_name = "get_or_default_QString_QVariant"]
        fn qhashGetOrDefault(_: &QHash_QString_QVariant, key: &QString) -> QVariant;
        #[rust_name = "get_unchecked_key_QString_QVariant"]
        unsafe fn qhashGetUncheckedKey(_: &QHash_QString_QVariant, pos: isize) -> &QString;
        #[rust_name = "get_unchecked_value_QString_QVariant"]
        unsafe fn qhashGetUncheckedValue(_: &QHash_QString_QVariant, pos: isize) -> &QVariant;
        #[rust_name = "insert_QString_QVariant"]
        fn qhashInsert(_: &mut QHash_QString_QVariant, key: &QString, value: &QVariant);
        #[rust_name = "len_QString_QVariant"]
        fn qhashLen(_: &QHash_QString_QVariant) -> isize;
        #[rust_name = "remove_QString_QVariant"]
        fn qhashRemove(_: &mut QHash_QString_QVariant, key: &QString) -> bool;
    }
}

pub(crate) fn clone(hash: &ffi::QHash_QString_QVariant) -> ffi::QHash_QString_QVariant {
    ffi::qhash_clone_QString_QVariant(hash)
}

pub(crate) fn default() -> ffi::QHash_QString_QVariant {
    ffi::qhash_default_QString_QVariant()
}

pub(crate) fn drop(hash: &mut ffi::QHash_QString_QVariant) {
    ffi::qhash_drop_QString_QVariant(hash);
}

pub(crate) fn get_or_default(
    hash: &ffi::QHash_QString_QVariant,
    key: &ffi::QString,
) -> ffi::QVariant {
    ffi::get_or_default_QString_QVariant(hash, key)
}

pub(crate) unsafe fn get_unchecked_key(
    hash: &ffi::QHash_QString_QVariant,
    pos: isize,
) -> &ffi::QString {
    ffi::get_unchecked_key_QString_QVariant(hash, pos)
}

pub(crate) unsafe fn get_unchecked_value(
    hash: &ffi::QHash_QString_QVariant,
    pos: isize,
) -> &ffi::QVariant {
    ffi::get_unchecked_value_QString_QVariant(hash, pos)
}

pub(crate) fn insert(
    hash: &mut ffi::QHash_QString_QVariant,
    key: &ffi::QString,
    value: &ffi::QVariant,
) {
    ffi::insert_QString_QVariant(hash, key, value);
}

pub(crate) fn len(hash: &ffi::QHash_QString_QVariant) -> isize {
    ffi::len_QString_QVariant(hash)
}

pub(crate) fn remove(hash: &mut ffi::QHash_QString_QVariant, key: &ffi::QString) -> bool {
    ffi::remove_QString_QVariant(hash, key)
}

#[allow(non_camel_case_types)]
pub struct QHashPair_QString_QVariant;

unsafe impl ExternType for QHashPair_QString_QVariant {
    type Id = type_id!("QHashPair_QString_QVariant");
    type Kind = cxx::kind::Trivial;
}

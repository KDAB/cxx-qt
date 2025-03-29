#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function tag_ffi() {
  if [[ $1 =~ ^[[:upper:]] ]]; then
    echo "ffi::$1"
  else
    echo "$1"
  fi
}

function generate_qhash_header(){
  local INCLUDE_1=""
  local INCLUDE_2=""

  if [[ -n $4 ]]; then
    INCLUDE_1="#include <$4>"
  fi

  if [[ -n $5 ]]; then
    INCLUDE_2="#include <$5>"
  fi

  tee "$SCRIPTPATH/../../../include/core/qhash/qhash_$1.h" <<EOF
// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This is an auto-generated file. Do not edit.
//! Edit instead: cxx-qt-lib/src/core/qhash/generate.sh
#pragma once
#include "qhash_private.h"
$INCLUDE_1
$INCLUDE_2
using QHash_$1 = QHash<$2, $3>;
EOF
}

function generate_bridge() {
    local K="$2"
    local V="$3"
    local SUFFIX="${K}_${V}"
    local QHASH="QHash_$SUFFIX"
    local QHASHPAIR="QHashPair_$SUFFIX"

    local FK=$K
    local FV=$V
    local INCLUDE_K=""
    local INCLUDE_V=""

    if [[ -n $4 ]]; then
      FK="ffi::$K"
      INCLUDE_K="include!(\"cxx-qt-lib/$4.h\");type $K = crate::$K;"
    fi

    if [[ -n $5 ]]; then
      FV="ffi::$V"
      INCLUDE_V="include!(\"cxx-qt-lib/$5.h\");type $V = crate::$V;"
    fi

    tee "$SCRIPTPATH/qhash_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        $INCLUDE_K $INCLUDE_V

        include!("cxx-qt-lib/qhash_$SUFFIX.h");
        type $QHASH = crate::QHash<super::$QHASHPAIR>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut $QHASH);
        #[rust_name = "cxx_contains"]
        fn contains(self: &$QHASH, key: &$K) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qhash_clone_$SUFFIX"]
        fn construct(_: &$QHASH) -> $QHASH;
        #[rust_name = "qhash_default_$SUFFIX"]
        fn construct() -> $QHASH;
        #[rust_name = "qhash_drop_$SUFFIX"]
        fn drop(_: &mut $QHASH);
    }

    #[namespace = "rust::cxxqtlib1::qhash"]
    unsafe extern "C++" {
        #[rust_name = "get_or_default_$SUFFIX"]
        fn qhashGetOrDefault(_: &$QHASH, key: &$K) -> $V;
        #[rust_name = "get_unchecked_key_$SUFFIX"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qhashGetUncheckedKey<'a>(_: &'a $QHASH, pos: isize) -> &'a $K;
        #[rust_name = "get_unchecked_value_$SUFFIX"]
        unsafe fn qhashGetUncheckedValue(_: &$QHASH, pos: isize) -> &$V;
        #[rust_name = "insert_$SUFFIX"]
        fn qhashInsert(_: &mut $QHASH, key: &$K, value: &$V);
        #[rust_name = "len_$SUFFIX"]
        fn qhashLen(_: &$QHASH) -> isize;
        #[rust_name = "remove_$SUFFIX"]
        fn qhashRemove(_: &mut $QHASH, key: &$K) -> bool;
    }
}

pub(crate) fn clone(hash: &ffi::$QHASH) -> ffi::$QHASH {
    ffi::qhash_clone_$SUFFIX(hash)
}

pub(crate) fn default() -> ffi::$QHASH {
    ffi::qhash_default_$SUFFIX()
}

pub(crate) fn drop(hash: &mut ffi::$QHASH) {
    ffi::qhash_drop_$SUFFIX(hash);
}

pub(crate) fn get_or_default(hash: &ffi::$QHASH, key: &$FK) -> $FV {
    ffi::get_or_default_$SUFFIX(hash, key)
}

pub(crate) unsafe fn get_unchecked_key(hash: &ffi::$QHASH, pos: isize) -> &$FK {
    ffi::get_unchecked_key_$SUFFIX(hash, pos)
}

pub(crate) unsafe fn get_unchecked_value(
    hash: &ffi::$QHASH,
    pos: isize,
) -> &$FV {
    ffi::get_unchecked_value_$SUFFIX(hash, pos)
}

pub(crate) fn insert(hash: &mut ffi::$QHASH, key: &$FK, value: &$FV) {
    ffi::insert_$SUFFIX(hash, key, value);
}

pub(crate) fn len(hash: &ffi::$QHASH) -> isize {
    ffi::len_$SUFFIX(hash)
}

pub(crate) fn remove(hash: &mut ffi::$QHASH, key: &$FK) -> bool {
    ffi::remove_$SUFFIX(hash, key)
}

#[allow(non_camel_case_types)]
pub struct QHashPair_$SUFFIX;

unsafe impl ExternType for QHashPair_$SUFFIX {
    type Id = type_id!("QHashPair_$SUFFIX");
    type Kind = cxx::kind::Trivial;
}
EOF
    rustfmt "$SCRIPTPATH/qhash_$1.rs"
}

generate_qhash_header "i32_QByteArray" "::std::int32_t" "::QByteArray" "" "QtCore/QByteArray"
generate_qhash_header "QString_QVariant" "::QString" "::QVariant" "QtCore/QString" "QtCore/QVariant"

generate_bridge "i32_qbytearray" "i32" "QByteArray" "" "qbytearray"
generate_bridge "qstring_qvariant" "QString" "QVariant" "qstring" "qvariant"

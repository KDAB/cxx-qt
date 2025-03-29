#!/usr/bin/env bash

# SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
# SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0

set -e

SCRIPT=$(realpath "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

function generate_qmap_header(){
  local INCLUDE_1=""
  local INCLUDE_2=""

  if [[ -n $4 ]]; then
    INCLUDE_1="#include <$4>"
  fi

  if [[ -n $5 ]]; then
    INCLUDE_2="#include <$5>"
  fi

  tee "$SCRIPTPATH/../../../include/core/qmap/qmap_$1.h" <<EOF
// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This is an auto-generated file. Do not edit.
//! Edit instead: cxx-qt-lib/src/core/qmap/generate.sh
#pragma once
#include "qmap_private.h"
$INCLUDE_1
$INCLUDE_2
using QMap_$1 = QMap<$2, $3>;
EOF
}

function generate_bridge() {
    local K="$2"
    local V="$3"
    local SUFFIX="${K}_${V}"
    local QMAP="QMap_$SUFFIX"
    local QMAPPAIR="QMapPair_$SUFFIX"

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

    tee "$SCRIPTPATH/qmap_$1.rs" <<EOF
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        $INCLUDE_K $INCLUDE_V

        include!("cxx-qt-lib/qmap_$SUFFIX.h");
        type $QMAP = crate::QMap<super::$QMAPPAIR>;
    }

    unsafe extern "C++" {
        #[rust_name = "cxx_clear"]
        fn clear(self: &mut $QMAP);
        #[rust_name = "cxx_contains"]
        fn contains(self: &$QMAP, key: &$K) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[rust_name = "qmap_clone_$SUFFIX"]
        fn construct(_: &$QMAP) -> $QMAP;
        #[rust_name = "qmap_default_$SUFFIX"]
        fn construct() -> $QMAP;
        #[rust_name = "qmap_drop_$SUFFIX"]
        fn drop(_: &mut $QMAP);
    }

    #[namespace = "rust::cxxqtlib1::qmap"]
    unsafe extern "C++" {
        #[rust_name = "get_or_default_$SUFFIX"]
        fn qmapGetOrDefault(_: &$QMAP, key: &$K) -> $V;
        #[rust_name = "get_unchecked_key_$SUFFIX"]
        #[allow(clippy::needless_lifetimes)]
        unsafe fn qmapGetUncheckedKey<'a>(_: &'a $QMAP, pos: isize) -> &'a $K;
        #[rust_name = "get_unchecked_value_$SUFFIX"]
        unsafe fn qmapGetUncheckedValue(_: &$QMAP, pos: isize) -> &$V;
        #[rust_name = "insert_$SUFFIX"]
        fn qmapInsert(_: &mut $QMAP, key: &$K, value: &$V);
        #[rust_name = "len_$SUFFIX"]
        fn qmapLen(_: &$QMAP) -> isize;
        #[rust_name = "remove_$SUFFIX"]
        fn qmapRemove(_: &mut $QMAP, key: &$K) -> bool;
    }
}

pub(crate) fn clone(map: &ffi::$QMAP) -> ffi::$QMAP {
    ffi::qmap_clone_$SUFFIX(map)
}

pub(crate) fn default() -> ffi::$QMAP {
    ffi::qmap_default_$SUFFIX()
}

pub(crate) fn drop(map: &mut ffi::$QMAP) {
    ffi::qmap_drop_$SUFFIX(map);
}

pub(crate) fn get_or_default(map: &ffi::$QMAP, key: &$FK) -> $FV {
    ffi::get_or_default_$SUFFIX(map, key)
}

pub(crate) unsafe fn get_unchecked_key(map: &ffi::$QMAP, pos: isize) -> &$FK {
    ffi::get_unchecked_key_$SUFFIX(map, pos)
}

pub(crate) unsafe fn get_unchecked_value(
    map: &ffi::$QMAP,
    pos: isize,
) -> &$FV {
    ffi::get_unchecked_value_$SUFFIX(map, pos)
}

pub(crate) fn insert(map: &mut ffi::$QMAP, key: &$FK, value: &$FV) {
    ffi::insert_$SUFFIX(map, key, value);
}

pub(crate) fn len(map: &ffi::$QMAP) -> isize {
    ffi::len_$SUFFIX(map)
}

pub(crate) fn remove(map: &mut ffi::$QMAP, key: &$FK) -> bool {
    ffi::remove_$SUFFIX(map, key)
}

#[allow(non_camel_case_types)]
pub struct QMapPair_$SUFFIX;

unsafe impl ExternType for QMapPair_$SUFFIX {
    type Id = type_id!("QMapPair_$SUFFIX");
    type Kind = cxx::kind::Trivial;
}
EOF
    rustfmt "$SCRIPTPATH/qmap_$1.rs"
}

generate_qmap_header "QString_QVariant" "::QString" "::QVariant" "QtCore/QString" "QtCore/QVariant"

generate_bridge "qstring_qvariant" "QString" "QVariant" "qstring" "qvariant"

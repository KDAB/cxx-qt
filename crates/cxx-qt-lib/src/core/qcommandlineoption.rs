// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcommandlineoption.h");
        type QCommandLineOption = super::QCommandLineOption;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;

        /// Returns the description set for this option.
        fn description(self: &QCommandLineOption) -> QString;

        /// Returns the names set for this option.
        fn names(self: &QCommandLineOption) -> QStringList;

        /// Returns the name of the expected value.
        #[rust_name = "value_name"]
        fn valueName(self: &QCommandLineOption) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qcommandlineoption_init_from_qcommandlineoption"]
        fn construct(commandLineOption: &QCommandLineOption) -> QCommandLineOption;
    }
}

#[repr(C)]
pub struct QCommandLineOption {
    _space: MaybeUninit<usize>,
}

impl Clone for QCommandLineOption {
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        ffi::qcommandlineoption_init_from_qcommandlineoption(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QCommandLineOption {
    type Id = type_id!("QCommandLineOption");
    type Kind = cxx::kind::Trivial;
}

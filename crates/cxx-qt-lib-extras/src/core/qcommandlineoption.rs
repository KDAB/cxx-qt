// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib-extras/core/qcommandlineoption.h");
        type QCommandLineOption = super::QCommandLineOption;
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        /// Returns the default values set for this option.
        #[rust_name = "default_values"]
        fn defaultValues(self: &QCommandLineOption) -> QStringList;

        /// Returns the description set for this option.
        fn description(self: &QCommandLineOption) -> QString;

        /// Returns the names set for this option.
        fn names(self: &QCommandLineOption) -> QStringList;

        /// Sets the default value used for this option to defaultValue.
        #[rust_name = "set_default_value"]
        fn setDefaultValue(self: &mut QCommandLineOption, value: &QString);

        /// Sets the list of default values used for this option to defaultValues.
        #[rust_name = "set_default_values"]
        fn setDefaultValues(self: &mut QCommandLineOption, values: &QStringList);

        /// Sets the description used for this option to description.
        /// It is customary to add a "." at the end of the description.
        #[rust_name = "set_description"]
        fn setDescription(self: &mut QCommandLineOption, description: &QString);

        /// Sets the name of the expected value, for the documentation, to valueName.
        #[rust_name = "set_value_name"]
        fn setValueName(self: &mut QCommandLineOption, valueName: &QString);

        /// Returns the name of the expected value.
        #[rust_name = "value_name"]
        fn valueName(self: &QCommandLineOption) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qcommandlineoption_drop"]
        fn drop(option: &mut QCommandLineOption);

        #[doc(hidden)]
        #[rust_name = "qcommandlineoption_init_from_qcommandlineoption"]
        fn construct(commandLineOption: &QCommandLineOption) -> QCommandLineOption;

        #[doc(hidden)]
        #[rust_name = "qcommandlineoption_init_from_qstring"]
        fn construct(string: &QString) -> QCommandLineOption;

        #[doc(hidden)]
        #[rust_name = "qcommandlineoption_init_from_qstringlist"]
        fn construct(names: &QStringList) -> QCommandLineOption;
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

impl Drop for QCommandLineOption {
    /// Destroys the qcommandlineoption.
    fn drop(&mut self) {
        ffi::qcommandlineoption_drop(self)
    }
}

impl From<&ffi::QString> for QCommandLineOption {
    /// Constructs a command line option object with the name name.
    fn from(name: &ffi::QString) -> Self {
        ffi::qcommandlineoption_init_from_qstring(name)
    }
}

impl From<&ffi::QStringList> for QCommandLineOption {
    /// Constructs a command line option object with the name name.
    fn from(names: &ffi::QStringList) -> Self {
        ffi::qcommandlineoption_init_from_qstringlist(names)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QCommandLineOption {
    type Id = type_id!("QCommandLineOption");
    type Kind = cxx::kind::Trivial;
}

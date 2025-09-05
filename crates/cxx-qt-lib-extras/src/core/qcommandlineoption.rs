// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use cxx_qt_lib::{QString, QStringList};
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

        /// Sets the default value used for this option to `default_value`.
        ///
        /// The default value is used if the user of the application does not specify the option on the command line.
        ///
        /// If `default_value` is empty, the option has no default values.
        #[rust_name = "set_default_value"]
        fn setDefaultValue(self: &mut QCommandLineOption, default_value: &QString);

        /// Sets the list of default values used for this option to `default_values`.
        ///
        /// The default values are used if the user of the application does not specify the option on the command line.
        #[rust_name = "set_default_values"]
        fn setDefaultValues(self: &mut QCommandLineOption, default_values: &QStringList);

        /// Sets the description used for this option to `description`.
        /// It is customary to add a "." at the end of the description.
        ///
        /// The description is used by [QCommandLineParser::showHelp](https://doc.qt.io/qt/qcommandlineparser.html#showHelp)().
        #[rust_name = "set_description"]
        fn setDescription(self: &mut QCommandLineOption, description: &QString);

        /// Sets the name of the expected value, for the documentation, to `value_name`.
        ///
        /// Options without a value assigned have a boolean-like behavior: either the user specifies `–option` or they don't.
        ///
        /// Options with a value assigned need to set a name for the expected value, for the documentation of the option in the help output. An option with names `o` and `output`, and a value name of file will appear as `-o, --output <file>`.
        ///
        /// Call [`QCommandLineParser::value`](crate::QCommandLineParser::value) if you expect the option to be present only once, and [`QCommandLineParser::values`](crate::QCommandLineParser::values) if you expect that option to be present multiple times.
        #[rust_name = "set_value_name"]
        fn setValueName(self: &mut QCommandLineOption, value_name: &QString);

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

/// The `QCommandLineOption` class defines a possible command-line option.
///
/// Qt Documentation: [QCommandLineOption](https://doc.qt.io/qt/qcommandlineoption.html#details)
#[repr(C)]
pub struct QCommandLineOption {
    _space: MaybeUninit<usize>,
}

impl Clone for QCommandLineOption {
    /// Constructs a copy of this `QCommandLineOption`.
    fn clone(&self) -> Self {
        ffi::qcommandlineoption_init_from_qcommandlineoption(self)
    }
}

impl Drop for QCommandLineOption {
    /// Destroys the `QCommandLineOption`.
    fn drop(&mut self) {
        ffi::qcommandlineoption_drop(self);
    }
}

impl From<&QString> for QCommandLineOption {
    /// Constructs a command line option object with the name name.
    fn from(name: &QString) -> Self {
        ffi::qcommandlineoption_init_from_qstring(name)
    }
}

impl From<&QStringList> for QCommandLineOption {
    /// Constructs a command line option object with the name name.
    fn from(names: &QStringList) -> Self {
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

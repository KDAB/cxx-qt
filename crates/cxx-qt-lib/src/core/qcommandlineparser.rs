// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qcommandlineparser.h");
        type QCommandLineParser = super::QCommandLineParser;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = crate::QStringList;

        /// Returns the application description.
        #[rust_name = "application_description"]
        fn applicationDescription(self: &QCommandLineParser) -> QString;

        /// Clears the definitions of additional arguments from the help text.
        #[rust_name = "clear_positional_arguments"]
        fn clearPositionalArguments(self: &mut QCommandLineParser);

        /// Returns a translated error text for the user. This should only be called when parse() returns false.
        #[rust_name = "error_text"]
        fn errorText(self: &QCommandLineParser) -> QString;

        /// Returns a string containing the complete help information.
        #[rust_name = "help_text"]
        fn helpText(self: &QCommandLineParser) -> QString;

        /// Returns a list of option names that were found.
        #[rust_name = "option_names"]
        fn optionNames(self: &QCommandLineParser) -> QStringList;

        /// Parses the command line arguments.
        fn parse(self: &mut QCommandLineParser, arguments: &QStringList) -> bool;

        /// Returns a list of positional arguments.
        #[rust_name = "positional_arguments"]
        fn positionalArguments(self: &QCommandLineParser) -> QStringList;

        /// Processes the command line arguments.
        fn process(self: &mut QCommandLineParser, arguments: &QStringList);

        /// Sets the application description shown by helpText().
        #[rust_name = "set_application_description"]
        fn setApplicationDescription(self: &mut QCommandLineParser, description: &QString);

        /// Displays the version information from QCoreApplication::applicationVersion(), and exits the application.
        #[rust_name = "show_version"]
        fn showVersion(self: &mut QCommandLineParser);

        /// Returns a list of unknown option names.
        #[rust_name = "unknown_option_names"]
        fn unknownOptionNames(self: &QCommandLineParser) -> QStringList;
    }
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[rust_name = "qcommandlineparser_value"]
        fn qcommandlineparserValue(parser: &QCommandLineParser, optionName: &QString) -> QString;

        #[rust_name = "qcommandlineparser_values"]
        fn qcommandlineparserValues(
            parser: &QCommandLineParser,
            optionName: &QString,
        ) -> QStringList;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qcommandlineparser_init_default"]
        fn construct() -> QCommandLineParser;
    }
}

/// QCoreApplication provides the command-line arguments as a simple list of strings.
/// QCommandLineParser provides the ability to define a set of options, parse the command-line arguments, and store which options have actually been used, as well as option values.
#[repr(C)]
pub struct QCommandLineParser {
    _space: MaybeUninit<usize>,
}

impl QCommandLineParser {
    /// Returns the option value found for the given option name optionName, or an empty string if not found.
    pub fn value(&self, option_name: &ffi::QString) -> ffi::QString {
        ffi::qcommandlineparser_value(self, option_name)
    }
    pub fn values(&self, option_name: &ffi::QString) -> ffi::QStringList {
        ffi::qcommandlineparser_values(self, option_name)
    }
}

impl Default for QCommandLineParser {
    /// Constructs a command line parser object.
    fn default() -> Self {
        ffi::qcommandlineparser_init_default()
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QCommandLineParser {
    type Id = type_id!("QCommandLineParser");
    type Kind = cxx::kind::Trivial;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_value() {
        let commandlineparser = ffi::qcommandlineparser_init_default();
        assert!(commandlineparser.error_text().is_empty());
        assert!(commandlineparser.application_description().is_empty());
    }
}

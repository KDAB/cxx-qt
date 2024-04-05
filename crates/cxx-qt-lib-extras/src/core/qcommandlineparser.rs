// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    /// The type of image format available in Qt.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QCommandLineParserOptionsAfterPositionalArgumentsMode {
        /// application argument --opt -t is interpreted as setting the options opt and t,
        /// just like application --opt -t argument would do. This is the default parsing mode.
        /// In order to specify that --opt and -t are positional arguments instead, the user can use --,
        /// as in application argument -- --opt -t.
        ParseAsOptions,
        /// application argument --opt is interpreted as having two positional arguments, argument and --opt.
        /// This mode is useful for executables that aim to launch other executables (e.g. wrappers, debugging tools, etc.)
        /// or that support internal commands followed by options for the command. argument is the name of the command,
        /// and all options occurring after it can be collected and parsed by another command line parser, possibly in another executable.
        ParseAsPositionalArguments,
    }

    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QCommandLineParserSingleDashWordOptionMode {
        ParseAsCompactedShortOptions,
        ParseAsLongOptions,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib-extras/qcommandlineparser.h");
        type QCommandLineParser = super::QCommandLineParser;
        include!("cxx-qt-lib-extras/qcommandlineoption.h");
        type QCommandLineOption = crate::QCommandLineOption;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        /// Adds help options to the command-line parser.
        #[rust_name = "add_help_option"]
        fn addHelpOption(self: &mut QCommandLineParser) -> QCommandLineOption;

        /// Adds the option option to look for while parsing.
        /// Returns true if adding the option was successful; otherwise returns false.
        /// Adding the option fails if there is no name attached to the option,
        /// or the option has a name that clashes with an option name added before.
        #[rust_name = "add_option"]
        fn addOption(self: &mut QCommandLineParser, option: &QCommandLineOption) -> bool;

        /// Adds the -v / --version option, which displays the version string of the application.
        /// This option is handled automatically by QCommandLineParser.
        #[rust_name = "add_version_option"]
        fn addVersionOption(self: &mut QCommandLineParser) -> QCommandLineOption;

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

        /// Sets the parsing mode to parsingMode. This must be called before process() or parse().
        #[rust_name = "set_single_dash_word_option_mode"]
        fn setSingleDashWordOptionMode(
            self: &mut QCommandLineParser,
            singleDashWordOptionMode: QCommandLineParserSingleDashWordOptionMode,
        );

        /// Sets the parsing mode to parsingMode. This must be called before process() or parse().
        #[rust_name = "set_options_after_positional_arguments_mode"]
        fn setOptionsAfterPositionalArgumentsMode(
            self: &mut QCommandLineParser,
            parsingMode: QCommandLineParserOptionsAfterPositionalArgumentsMode,
        );

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

        #[rust_name = "is_set_from_qstring"]
        fn qcommandlineparserIsSetFromQString(parser: &QCommandLineParser, name: &QString) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");
        type QCommandLineParserOptionsAfterPositionalArgumentsMode;
        type QCommandLineParserSingleDashWordOptionMode;

        #[doc(hidden)]
        #[rust_name = "qcommandlineparser_drop"]
        fn drop(parser: &mut QCommandLineParser);

        #[doc(hidden)]
        #[rust_name = "qcommandlineparser_init_default"]
        fn construct() -> QCommandLineParser;
    }
}

/// QCoreApplication provides the command-line arguments as a simple list of strings.
/// QCommandLineParser provides the ability to define a set of options, parse the command-line arguments, and store which options have actually been used, as well as option values.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct QCommandLineParser {
    _space: MaybeUninit<usize>,
}

impl QCommandLineParser {
    /// Returns the option value found for the given option name optionName, or an empty string if not found.
    pub fn value(&self, option_name: &ffi::QString) -> ffi::QString {
        ffi::qcommandlineparser_value(self, option_name)
    }

    /// Returns a list of option values found for the given option name optionName, or an empty list if not found.
    pub fn values(&self, option_name: &ffi::QString) -> ffi::QStringList {
        ffi::qcommandlineparser_values(self, option_name)
    }

    /// Checks whether the option name was passed to the application.
    pub fn is_set(&self, name: &ffi::QString) -> bool {
        ffi::is_set_from_qstring(self, name)
    }
}

impl Default for QCommandLineParser {
    /// Constructs a command line parser object.
    fn default() -> Self {
        ffi::qcommandlineparser_init_default()
    }
}

impl Drop for QCommandLineParser {
    fn drop(&mut self) {
        ffi::qcommandlineparser_drop(self);
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

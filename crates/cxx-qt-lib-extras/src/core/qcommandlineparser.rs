// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use cxx_qt_lib::{QString, QStringList};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    /// This enum describes the way the parser interprets options that occur after positional arguments.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QCommandLineParserOptionsAfterPositionalArgumentsMode {
        /// `application argument --opt -t` is interpreted as setting the options `opt` and `t`,
        /// just like `application --opt -t argument` would do. This is the default parsing mode.
        /// In order to specify that `--opt` and `-t` are positional arguments instead, the user can use `--`,
        /// as in `application argument -- --opt -t`.
        ParseAsOptions,
        /// `application argument --opt` is interpreted as having two positional arguments, `argument` and `--opt`.
        /// This mode is useful for executables that aim to launch other executables (e.g. wrappers, debugging tools, etc.)
        /// or that support internal commands followed by options for the command. argument is the name of the command,
        /// and all options occurring after it can be collected and parsed by another command line parser, possibly in another executable.
        ParseAsPositionalArguments,
    }

    /// This enum describes the way the parser interprets command-line options that use a single dash followed by multiple letters, as `-abc`.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QCommandLineParserSingleDashWordOptionMode {
        /// `-abc` is interpreted as `-a -b -c`, i.e. as three short options that have been compacted on the command-line, if none of the options take a value. If `a` takes a value, then it is interpreted as `-a bc`, i.e. the short option a followed by the value `bc`. This is typically used in tools that behave like compilers, in order to handle options such as `-DDEFINE=VALUE` or `-I/include/path`. This is the default parsing mode. New applications are recommended to use this mode.
        ParseAsCompactedShortOptions,
        /// `-abc` is interpreted as `--abc`, i.e. as the long option named `abc`. This is how Qt's own tools (uic, rcc...) have always been parsing arguments. This mode should be used for preserving compatibility in applications that were parsing arguments in such a way. There is an exception if the `a` option has the [`QCommandLineOption::ShortOptionStyle`] flag set, in which case it is still interpreted as `-a bc`.
        ParseAsLongOptions,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib-extras/core/qcommandlineparser.h");
        type QCommandLineParser = super::QCommandLineParser;
        include!("cxx-qt-lib-extras/core/qcommandlineoption.h");
        type QCommandLineOption = crate::QCommandLineOption;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qstringlist.h");
        type QStringList = cxx_qt_lib::QStringList;

        /// Adds help options to the command-line parser.
        ///
        /// The options specified for this command-line are described by `-h` or `--help`. On Windows, the alternative `-?` is also supported. The option `--help-all` extends that to include generic Qt options, not defined by this command, in the output.
        ///
        /// These options are handled automatically by `QCommandLineParser`.
        #[rust_name = "add_help_option"]
        fn addHelpOption(self: &mut QCommandLineParser) -> QCommandLineOption;

        /// Adds the option `option` to look for while parsing.
        /// Returns `true` if adding the option was successful; otherwise returns `false`.
        ///
        /// Adding the option fails if there is no name attached to the option,
        /// or the option has a name that clashes with an option name added before.
        #[rust_name = "add_option"]
        fn addOption(self: &mut QCommandLineParser, option: &QCommandLineOption) -> bool;

        /// Adds the `-v` / `--version` option, which displays the version string of the application.
        ///
        /// This option is handled automatically by `QCommandLineParser`.
        #[rust_name = "add_version_option"]
        fn addVersionOption(self: &mut QCommandLineParser) -> QCommandLineOption;

        /// Returns the application description.
        #[rust_name = "application_description"]
        fn applicationDescription(self: &QCommandLineParser) -> QString;

        /// Clears the definitions of additional arguments from the help text.
        #[rust_name = "clear_positional_arguments"]
        fn clearPositionalArguments(self: &mut QCommandLineParser);

        /// Returns a translated error text for the user. This should only be called when [`parse`](Self::parse) returns `false`.
        #[rust_name = "error_text"]
        fn errorText(self: &QCommandLineParser) -> QString;

        /// Returns a string containing the complete help information.
        #[rust_name = "help_text"]
        fn helpText(self: &QCommandLineParser) -> QString;

        /// Returns a list of option names that were found.
        ///
        /// This returns a list of all the recognized option names found by the parser, in the order in which they were found. For any long options that were in the form {–option=value}, the value part will have been dropped.
        ///
        /// The names in this list do not include the preceding dash characters. Names may appear more than once in this list if they were encountered more than once by the parser.
        ///
        /// Any entry in the list can be used with [`value`](Self::value) or with [`values`](Self::values) to get any relevant option values.
        #[rust_name = "option_names"]
        fn optionNames(self: &QCommandLineParser) -> QStringList;

        /// Parses the command line arguments.
        ///
        /// Most programs don't need to call this, a simple call to [`process`] is enough.
        ///
        /// This function is more low-level, and only does the parsing. The application will have to take care of the error handling, using [`error_text`] if this function returns `false`. This can be useful for instance to show a graphical error message in graphical programs.
        ///
        /// Calling this function instead of [`process`] can also be useful in order to ignore unknown options temporarily, because more option definitions will be provided later on (depending on one of the arguments), before calling [`process`].
        ///
        /// Don't forget that arguments must start with the name of the executable (ignored, though).
        ///
        /// Returns `false` in case of a parse error (unknown option or missing value); returns `true` otherwise.
        ///
        /// [`process`]: Self::process
        /// [`error_text`]: Self::error_text
        fn parse(self: &mut QCommandLineParser, arguments: &QStringList) -> bool;

        /// Returns a list of positional arguments.
        ///
        /// These are all of the arguments that were not recognized as part of an option.
        #[rust_name = "positional_arguments"]
        fn positionalArguments(self: &QCommandLineParser) -> QStringList;

        /// Processes the command line arguments.
        ///
        /// In addition to parsing the options (like [`parse`](Self::parse)), this function also handles the builtin options and handles errors.
        ///
        /// The builtin options are `--version` if [`add_version_option`](Self::add_version_option) was called and `--help` / `--help-all` if [`add_help_option`](Self::add_help_option) was called.
        ///
        /// When invoking one of these options, or when an error happens (for instance an unknown option was passed), the current process will then stop, using the [exit](https://doc.qt.io/qt/qcoreapplication.html#exit)() function.
        fn process(self: &mut QCommandLineParser, arguments: &QStringList);

        /// Sets the application description shown by [`help_text`](Self::help_text).
        #[rust_name = "set_application_description"]
        fn setApplicationDescription(self: &mut QCommandLineParser, description: &QString);

        /// Sets the parsing mode to `single_dash_word_option_mode`. This must be called before [`process`](Self::process) or [`parse`](Self::parse).
        #[rust_name = "set_single_dash_word_option_mode"]
        fn setSingleDashWordOptionMode(
            self: &mut QCommandLineParser,
            single_dash_word_option_mode: QCommandLineParserSingleDashWordOptionMode,
        );

        /// Sets the parsing mode to `parsing_mode`. This must be called before [`process`](Self::process) or [`parse`](Self::parse).
        #[rust_name = "set_options_after_positional_arguments_mode"]
        fn setOptionsAfterPositionalArgumentsMode(
            self: &mut QCommandLineParser,
            parsing_mode: QCommandLineParserOptionsAfterPositionalArgumentsMode,
        );

        /// Displays the version information from [`QCoreApplication::application_version`](cxx_qt_lib::QCoreApplication::application_version), and exits the application.
        ///
        ///  This is automatically triggered by the `–version` option, but can also be used to display the version when not using [`process`](Self::process). The exit code is set to `EXIT_SUCCESS` (0).
        #[rust_name = "show_version"]
        fn showVersion(self: &mut QCommandLineParser);

        /// Returns a list of unknown option names.
        ///
        /// This list will include both long and short name options that were not recognized. For any long options that were in the form {–option=value}, the value part will have been dropped and only the long name is added.
        ///
        /// The names in this list do not include the preceding dash characters. Names may appear more than once in this list if they were encountered more than once by the parser.
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

/// The QCommandLineParser class provides a means for handling the command line options.
///
/// Qt Documentation: [QCommandLineParser](https://doc.qt.io/qt/qcommandlineparser.html#details)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct QCommandLineParser {
    _space: MaybeUninit<usize>,
}

impl QCommandLineParser {
    /// Returns the option value found for the given option name `option_name`, or an empty string if not found.
    ///
    /// The name provided can be any long or short name of any option that was added with [`add_option`](Self::add_option). All the option names are treated as being equivalent. If the name is not recognized or that option was not present, an empty string is returned.
    ///
    /// For options found by the parser, the last value found for that option is returned. If the option wasn't specified on the command line, the default value is returned.
    ///
    /// If the option does not take a value, a warning is printed, and an empty string is returned.
    pub fn value(&self, option_name: &QString) -> QString {
        ffi::qcommandlineparser_value(self, option_name)
    }

    /// Returns a list of option values found for the given option name `option_name`, or an empty list if not found.
    ///
    /// The name provided can be any long or short name of any option that was added with [`add_option`](Self::add_option). All the options names are treated as being equivalent. If the name is not recognized or that option was not present, an empty list is returned.
    ///
    /// For options found by the parser, the list will contain an entry for each time the option was encountered by the parser. If the option wasn't specified on the command line, the default values are returned.
    ///
    /// An empty list is returned if the option does not take a value.
    pub fn values(&self, option_name: &QString) -> QStringList {
        ffi::qcommandlineparser_values(self, option_name)
    }

    /// Checks whether the option `name` was passed to the application.
    ///
    /// Returns `true` if the option `name` was set, `false` otherwise.
    ///
    /// The name provided can be any long or short name of any option that was added with [`add_option`](Self::add_option). All the options names are treated as being equivalent. If the name is not recognized or that option was not present, `false` is returned.
    pub fn is_set(&self, name: &QString) -> bool {
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

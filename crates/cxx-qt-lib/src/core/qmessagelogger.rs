// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::ffi::CStr;
use std::fmt;
use std::ptr;

use crate::{qt_message_output, QMessageLogContext, QtMsgType};

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qtlogging.h");
        type QMessageLogContext<'a> = crate::QMessageLogContext<'a>;
        type QtMsgType = crate::QtMsgType;
    }

    extern "C++" {
        include!("cxx-qt-lib/qmessagelogger.h");
        type QMessageLogger<'a> = super::QMessageLogger<'a>;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "is_logging_category_enabled"]
        fn isLoggingCategoryEnabled(context: &QMessageLogContext, level: QtMsgType) -> bool;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qmessagelogger_init"]
        unsafe fn construct<'a>(
            file_name: *const c_char,
            line_number: i32,
            function_name: *const c_char,
            category_name: *const c_char,
        ) -> QMessageLogger<'a>;
    }
}

/// The `QMessageLoggger` class generates log messages.
///
/// Qt Documentation: [QMessageLogger](https://doc.qt.io/qt/qmessagelogger.html#details)
///
/// # Using with other libraries
///
/// `QMessageLogger` can be used as a handler for logging libraries. For example, here is an
/// implementation for the [log](https://docs.rs/log/latest/log/) library:
///
/// ```rust,ignore
/// use std::ffi::CString;
/// use cxx_qt_lib::QMessageLogger;
///
/// struct QtLogger;
///
/// impl log::Log for QtLogger {
///     fn log(&self, record: &log::Record) {
///         let file = record
///             .file()
///             .and_then(|file| CString::new(file).ok());
///         let line = record.line().unwrap_or_default().cast_signed();
///         let target = record.target();
///         let category = if target.is_empty() {
///             None
///         } else {
///             CString::new(target).ok()
///         };
///         let logger = QMessageLogger::new(
///             file.as_deref().unwrap_or_default(),
///             line,
///             None,
///             category.as_deref(),
///         );
///         let message = *record.args();
///         match record.level() {
///             log::Level::Error => logger.critical(message),
///             log::Level::Warn => logger.warning(message),
///             log::Level::Info => logger.info(message),
///             log::Level::Debug | log::Level::Trace => logger.debug(message),
///         }
///     }
/// }
/// ```
#[repr(C)]
#[derive(Clone)]
pub struct QMessageLogger<'a> {
    context: QMessageLogContext<'a>,
}

impl<'a> QMessageLogger<'a> {
    /// Constructs a `QMessageLogger` to record `category` messages for `file` at `line` in `function`.
    ///
    /// If `category` is `None`, the default category is used (`c"default"`).
    pub fn new(
        file: &'a CStr,
        line: i32,
        function: Option<&'a CStr>,
        category: Option<&'a CStr>,
    ) -> Self {
        let file = file.as_ptr();
        let function = match function {
            Some(function) => function.as_ptr(),
            None => ptr::null(),
        };
        let category = category.unwrap_or(c"default").as_ptr();
        // SAFETY: All pointers are valid.
        unsafe { ffi::qmessagelogger_init(file, line, function, category) }
    }

    #[inline]
    fn log(&self, level: QtMsgType, message: fmt::Arguments<'_>) {
        if ffi::is_logging_category_enabled(&self.context, level) {
            qt_message_output(level, &self.context, &(message.into()));
        }
    }

    /// Logs a critical message.
    /// Uses the same behavior as [`q_critical!`].
    pub fn critical(&self, message: fmt::Arguments<'_>) {
        self.log(QtMsgType::QtCriticalMsg, message);
    }

    /// Logs a debug message.
    /// Uses the same behavior as [`q_debug!`].
    pub fn debug(&self, message: fmt::Arguments<'_>) {
        self.log(QtMsgType::QtDebugMsg, message);
    }

    /// Logs a fatal message.
    /// Uses the same behavior as [`q_fatal!`].
    pub fn fatal(&self, message: fmt::Arguments<'_>) {
        self.log(QtMsgType::QtFatalMsg, message);
    }

    /// Logs an info message.
    /// Uses the same behavior as [`q_info!`].
    pub fn info(&self, message: fmt::Arguments<'_>) {
        self.log(QtMsgType::QtInfoMsg, message);
    }

    /// Logs a warning message.
    /// Uses the same behavior as [`q_warning!`].
    pub fn warning(&self, message: fmt::Arguments<'_>) {
        self.log(QtMsgType::QtWarningMsg, message);
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the layout is the same.
unsafe impl ExternType for QMessageLogger<'_> {
    type Id = type_id!("QMessageLogger");
    type Kind = cxx::kind::Trivial;
}

/// Constructs a [`QMessageLogger`] for the current [`file!`] and [`line!`], optionally providing
/// a [logging category](https://doc.qt.io/qt-6/qloggingcategory.html).
///
/// # Examples
///
/// ```rust
/// use cxx_qt_lib::q_logger;
///
/// fn somefunc() {
///     // Using the default category
///     let logger = q_logger!();
///     // Using a literal as the category
///     let logger = q_logger!("my_category");
///     // Using a variable as the category
///     let category = c"my_category";
///     let logger = q_logger!(category);
/// }
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {
/// #   cxx_qt::init_crate!(cxx_qt_lib);
/// # }
/// ```
#[macro_export]
macro_rules! q_logger {
    () => {
        $crate::QMessageLogger::new(
            unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes())
            },
            line!() as _,
            None,
            None,
        )
    };
    ($cat:literal) => {
        $crate::QMessageLogger::new(
            unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes())
            },
            line!() as _,
            None,
            Some(unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($cat, "\0").as_bytes())
            }),
        )
    };
    ($cat:expr) => {
        $crate::QMessageLogger::new(
            unsafe {
                std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes())
            },
            line!() as _,
            None,
            Some($cat),
        )
    };
}

/// Calls the Qt message handler with a formatted debug message. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2. This function does nothing if `QT_NO_DEBUG_OUTPUT` was defined during compilation.
///
/// # Examples
///
/// ```rust
/// use cxx_qt_lib::q_debug;
///
/// fn somefunc(x: i32, y: i32) {
///     q_debug!("x: {x}, y: {y}");
///     q_debug!("x: {}, y: {}", x, y);
/// }
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {
/// #   cxx_qt::init_crate!(cxx_qt_lib);
/// # }
/// ```
#[macro_export]
macro_rules! q_debug {
    ($($arg:tt)*) => ($crate::q_logger!().debug(std::format_args!($($arg)*)));
}

/// Calls the Qt message handler with a formatted informational message. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2. This function does nothing if `QT_NO_INFO_OUTPUT` was defined during compilation.
///
/// # Examples
///
/// ```rust
/// use cxx_qt_lib::q_info;
///
/// fn somefunc(x: i32, y: i32) {
///     q_info!("x: {x}, y: {y}");
///     q_info!("x: {}, y: {}", x, y);
/// }
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {
/// #   cxx_qt::init_crate!(cxx_qt_lib);
/// # }
/// ```
#[macro_export]
macro_rules! q_info {
    ($($arg:tt)*) => ($crate::q_logger!().info(std::format_args!($($arg)*)));
}

/// Calls the Qt message handler with a formatted warning message. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2. This function does nothing if `QT_NO_WARNING_OUTPUT` was defined during compilation.
///
/// For debugging purposes, it is sometimes convenient to let the program abort for warning messages. This allows you then to inspect the core dump, or attach a debugger - see also [`q_fatal!`]. To enable this, set the environment variable `QT_FATAL_WARNINGS` to a number `n`. The program terminates then for the `n`-th warning. That is, if the environment variable is set to 1, it will terminate on the first call; if it contains the value 10, it will exit on the 10th call. Any non-numeric value in the environment variable is equivalent to 1.
///
/// # Examples
///
/// ```rust
/// use cxx_qt_lib::q_warning;
///
/// fn somefunc(x: i32, y: i32) {
///     q_warning!("x: {x}, y: {y}");
///     q_warning!("x: {}, y: {}", x, y);
/// }
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {
/// #   cxx_qt::init_crate!(cxx_qt_lib);
/// # }
/// ```
#[macro_export]
macro_rules! q_warning {
    ($($arg:tt)*) => ($crate::q_logger!().warning(std::format_args!($($arg)*)));
}

/// Calls the Qt message handler with a formatted critical message. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2. This function does nothing if `QT_NO_WARNING_OUTPUT` was defined during compilation.
///
/// For debugging purposes, it is sometimes convenient to let the program abort for critical messages. This allows you then to inspect the core dump, or attach a debugger - see also [`q_fatal!`]. To enable this, set the environment variable `QT_FATAL_CRITICALS` to a number `n`. The program terminates then for the `n`-th critical message. That is, if the environment variable is set to 1, it will terminate on the first call; if it contains the value 10, it will exit on the 10th call. Any non-numeric value in the environment variable is equivalent to 1.
///
/// # Examples
///
/// ```rust
/// use cxx_qt_lib::q_critical;
///
/// fn somefunc(x: i32, y: i32) {
///     q_critical!("x: {x}, y: {y}");
///     q_critical!("x: {}, y: {}", x, y);
/// }
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {
/// #   cxx_qt::init_crate!(cxx_qt_lib);
/// # }
/// ```
#[macro_export]
macro_rules! q_critical {
    ($($arg:tt)*) => ($crate::q_logger!().critical(std::format_args!($($arg)*)));
}

/// Calls the Qt message handler with a formatted fatal message. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2.
///
/// If you are using the **default message handler** this function will abort to create a core dump. On Windows, for debug builds, this function will report a `_CRT_ERROR` enabling you to connect a debugger to the application.
///
/// # Examples
///
/// ```rust
/// use cxx_qt_lib::q_fatal;
///
/// fn somefunc(x: i32, y: i32) {
///     q_fatal!("x: {x}, y: {y}");
///     q_fatal!("x: {}, y: {}", x, y);
/// }
/// # // Note that we need a fake main for doc tests to build
/// # fn main() {
/// #   cxx_qt::init_crate!(cxx_qt_lib);
/// # }
/// ```
#[macro_export]
macro_rules! q_fatal {
    ($($arg:tt)*) => ($crate::q_logger!().fatal(std::format_args!($($arg)*)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q_logger_macro() {
        let line = line!();
        let logger = q_logger!();
        assert_eq!(
            (
                logger.context.file().to_str().unwrap(),
                logger.context.line() as u32,
            ),
            (file!(), line + 1),
        );
    }

    #[test]
    fn qmessagelogger_init() {
        let logger = QMessageLogger::new(c"file", 10, Some(c"function"), Some(c"category"));
        assert_eq!(logger.context.file(), c"file");
        assert_eq!(logger.context.line(), 10);
        assert_eq!(logger.context.function(), c"function");
        assert_eq!(logger.context.category(), c"category");
    }
}

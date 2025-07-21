// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::ffi::CStr;

use crate::QString;

#[cxx::bridge]
mod ffi {
    extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qtlogging.h");

        /// Changes the output of the default message handler.
        /// Allows to tweak the output of [`q_debug!`](crate::q_debug!), [`q_info!`](crate::q_info!), [`q_warning!`](crate::q_warning!), [`q_critical!`](crate::q_critical!), and [`q_fatal!`](crate::q_fatal!).
        ///
        /// See the [Qt documentation](https://doc.qt.io/qt/qtlogging.html#qSetMessagePattern) for pattern syntax.
        #[rust_name = "q_set_message_pattern"]
        fn qSetMessagePattern(pattern: &QString);

    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        unsafe fn q_debug(file: *const c_char, line: i32, message: &QString);
        unsafe fn q_info(file: *const c_char, line: i32, message: &QString);
        unsafe fn q_warning(file: *const c_char, line: i32, message: &QString);
        unsafe fn q_critical(file: *const c_char, line: i32, message: &QString);
        unsafe fn q_fatal(file: *const c_char, line: i32, message: &QString);
    }
}

pub use ffi::q_set_message_pattern;

/// Backing function for the [`q_debug!`](crate::q_debug!) macro. See the macro's documentation for more details.
pub fn q_debug(file: &CStr, line: i32, message: &QString) {
    // SAFETY: All strings are zero-terminated.
    unsafe {
        ffi::q_debug(file.as_ptr(), line, message);
    }
}

/// Backing function for the [`q_info!`](crate::q_info!) macro. See the macro's documentation for more details.
pub fn q_info(file: &CStr, line: i32, message: &QString) {
    // SAFETY: All strings are zero-terminated.
    unsafe {
        ffi::q_info(file.as_ptr(), line, message);
    }
}

/// Backing function for the [`q_warning!`](crate::q_warning!) macro. See the macro's documentation for more details.
pub fn q_warning(file: &CStr, line: i32, message: &QString) {
    // SAFETY: All strings are zero-terminated.
    unsafe {
        ffi::q_warning(file.as_ptr(), line, message);
    }
}

/// Backing function for the [`q_critical!`](crate::q_critical!) macro. See the macro's documentation for more details.
pub fn q_critical(file: &CStr, line: i32, message: &QString) {
    // SAFETY: All strings are zero-terminated.
    unsafe {
        ffi::q_critical(file.as_ptr(), line, message);
    }
}

/// Backing function for the [`q_fatal!`](crate::q_fatal!) macro. See the macro's documentation for more details.
pub fn q_fatal(file: &CStr, line: i32, message: &QString) {
    // SAFETY: All strings are zero-terminated.
    unsafe {
        ffi::q_fatal(file.as_ptr(), line, message);
    }
}

/// Calls the Qt message handler with a formatted debug message, using the first argument as the function name for the log context, the second argument as the format string for the log message, and the remaining arguments as arguments to format. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2. This function does nothing if `QT_NO_DEBUG_OUTPUT` was defined during compilation.
///
/// # Examples
///
/// ```rust,ignore
/// use cxx_qt_lib::q_debug;
///
/// fn somefunc(x: i32, y: i32) {
///     q_debug!("x: {x}, y: {y}");
///     q_debug!("x: {}, y: {}", x, y);
/// }
/// ```
#[macro_export]
macro_rules! q_debug {
    ($($arg:tt)*) => ($crate::q_debug(
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
        line!() as i32,
        &$crate::QString::from(std::format_args!($($arg)*))
    ));
}

/// Calls the Qt message handler with a formatted informational message, using the first argument as the function name for the log context, the second argument as the format string for the log message, and the remaining arguments as arguments to format. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2. This function does nothing if `QT_NO_INFO_OUTPUT` was defined during compilation.
///
/// # Examples
///
/// ```rust,ignore
/// use cxx_qt_lib::q_info;
///
/// fn somefunc(x: i32, y: i32) {
///     q_info!("x: {x}, y: {y}");
///     q_info!("x: {}, y: {}", x, y);
/// }
/// ```
#[macro_export]
macro_rules! q_info {
    ($($arg:tt)*) => ($crate::q_info(
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
        line!() as i32,
        &$crate::QString::from(std::format_args!($($arg)*))
    ));
}

/// Calls the Qt message handler with a formatted warning message, using the first argument as the function name for the log context, the second argument as the format string for the log message, and the remaining arguments as arguments to format. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2. This function does nothing if `QT_NO_WARNING_OUTPUT` was defined during compilation.
///
/// For debugging purposes, it is sometimes convenient to let the program abort for warning messages. This allows you then to inspect the core dump, or attach a debugger - see also [`q_fatal`]. To enable this, set the environment variable `QT_FATAL_WARNINGS` to a number `n`. The program terminates then for the `n`-th warning. That is, if the environment variable is set to 1, it will terminate on the first call; if it contains the value 10, it will exit on the 10th call. Any non-numeric value in the environment variable is equivalent to 1.
///
/// # Examples
///
/// ```rust,ignore
/// use cxx_qt_lib::q_warning;
///
/// fn somefunc(x: i32, y: i32) {
///     q_warning!("x: {x}, y: {y}");
///     q_warning!("x: {}, y: {}", x, y);
/// }
/// ```
#[macro_export]
macro_rules! q_warning {
    ($($arg:tt)*) => ($crate::q_warning(
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
        line!() as i32,
        &$crate::QString::from(std::format_args!($($arg)*))
    ));
}

/// Calls the Qt message handler with a critical message, using the first argument as the function name for the log context, the second argument as the format string for the log message, and the remaining arguments as arguments to format. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2. This function does nothing if `QT_NO_WARNING_OUTPUT` was defined during compilation.
///
/// For debugging purposes, it is sometimes convenient to let the program abort for critical messages. This allows you then to inspect the core dump, or attach a debugger - see also [`q_fatal`]. To enable this, set the environment variable `QT_FATAL_CRITICALS` to a number `n`. The program terminates then for the `n`-th critical message. That is, if the environment variable is set to 1, it will terminate on the first call; if it contains the value 10, it will exit on the 10th call. Any non-numeric value in the environment variable is equivalent to 1.
///
/// # Examples
///
/// ```rust,ignore
/// use cxx_qt_lib::q_critical;
///
/// fn somefunc(x: i32, y: i32) {
///     q_critical!("x: {x}, y: {y}");
///     q_critical!("x: {}, y: {}", x, y);
/// }
/// ```
#[macro_export]
macro_rules! q_critical {
    ($($arg:tt)*) => ($crate::q_critical(
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
        line!() as i32,
        &$crate::QString::from(std::format_args!($($arg)*))
    ));
}

///
/// Calls the Qt message handler with a fatal message, using the first argument as the function name for the log context, the second argument as the format string for the log message, and the remaining arguments as arguments to format. If no message handler has been installed, the message is printed to stderr. Under Windows the message is sent to the console, if it is a console application; otherwise, it is sent to the debugger. On QNX, the message is sent to slogger2.
///
/// If you are using the **default message handler** this function will abort to create a core dump. On Windows, for debug builds, this function will report a `_CRT_ERROR` enabling you to connect a debugger to the application.
///
/// # Examples
///
/// ```rust,ignore
/// use cxx_qt_lib::q_fatal;
///
/// fn somefunc(x: i32, y: i32) {
///     q_fatal!("x: {x}, y: {y}");
///     q_fatal!("x: {}, y: {}", x, y);
/// }
/// ```
#[macro_export]
macro_rules! q_fatal {
    ($($arg:tt)*) => ($crate::q_fatal(
        unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(concat!(file!(), "\0").as_bytes()) },
        line!() as i32,
        &$crate::QString::from(std::format_args!($($arg)*))
    ));
}

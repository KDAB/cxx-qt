// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::ffi::c_char;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::size_of;

#[cxx::bridge]
mod ffi {
    /// The level the message is sent to the message handler at.
    #[repr(i32)]
    enum QtMsgType {
        /// A debug message.
        QtDebugMsg = 0,
        /// An info message.
        QtInfoMsg = 4,
        /// A warning message.
        QtWarningMsg = 1,
        /// A fatal message.
        QtFatalMsg = 3,
        /// A critical message.
        QtCriticalMsg = 2,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qtlogging.h");
        type QMessageLogContext<'a> = crate::QMessageLogContext<'a>;
        type QtMsgType;

        /// Outputs a message in the Qt message handler.
        ///
        /// **Warning:** This function is an undocumented internal utility in the Qt library.
        fn qt_message_output(msg_type: QtMsgType, context: &QMessageLogContext, string: &QString);

        /// Generates a formatted string out of the `msg_type`, `context`, `str` arguments.
        ///
        /// This function returns a `QString` that is formatted according to the current message pattern. It can be used by custom message handlers to format output similar to Qt's default message handler.
        ///
        /// The function is thread-safe.
        #[cxx_name = "qFormatLogMessage"]
        fn q_format_log_message(
            msg_type: QtMsgType,
            context: &QMessageLogContext,
            str: &QString,
        ) -> QString;

        /// Changes the output of the default message handler.
        /// See the [Qt documentation](https://doc.qt.io/qt/qtlogging.html#qSetMessagePattern) for full details.
        ///
        /// # Safety
        /// This function is marked as unsafe because it is not guaranteed to be thread-safe.
        #[allow(clippy::missing_safety_doc)]
        #[cxx_name = "qSetMessagePattern"]
        unsafe fn q_set_message_pattern(pattern: &QString);

        #[cxx_name = "qmessagelogcontext_line"]
        #[doc(hidden)]
        fn line(context: &QMessageLogContext) -> i32;

        #[cxx_name = "qmessagelogcontext_file"]
        #[doc(hidden)]
        unsafe fn file(context: &QMessageLogContext) -> *const c_char;

        #[cxx_name = "qmessagelogcontext_function"]
        #[doc(hidden)]
        unsafe fn function(context: &QMessageLogContext) -> *const c_char;

        #[cxx_name = "qmessagelogcontext_category"]
        #[doc(hidden)]
        unsafe fn category(context: &QMessageLogContext) -> *const c_char;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "construct_qmessagelogcontext"]
        unsafe fn construct<'a>(
            file_name: *const c_char,
            line_number: i32,
            function_name: *const c_char,
            category_name: *const c_char,
        ) -> QMessageLogContext<'a>;
    }
}

/// The `QMessageLogContext` class defines the context passed to the Qt message handler.
///
/// Qt Documentation: [QMessageLogContext](https://doc.qt.io/qt/qmessagelogcontext.html#details)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QMessageLogContext<'a> {
    version: i32,
    line: i32,
    file: *const c_char,
    function: *const c_char,
    category: *const c_char,
    _phantom: PhantomData<&'a c_char>,
}

const_assert!(
    size_of::<QMessageLogContext>() == (size_of::<i32>() * 2) + (size_of::<*const c_char>() * 3)
);

impl<'a> QMessageLogContext<'a> {
    pub fn new(
        file: &'a CStr,
        line: i32,
        function: &'a CStr,
        category: &'a CStr,
    ) -> QMessageLogContext<'a> {
        unsafe {
            ffi::construct_qmessagelogcontext(
                file.as_ptr(),
                line,
                function.as_ptr(),
                category.as_ptr(),
            )
        }
    }

    /// The line number given to the message handler.
    pub fn line(&self) -> i32 {
        ffi::line(self)
    }

    /// The file path given to the message handler.
    pub fn file(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(ffi::file(self)) }
    }

    /// The name of the function given to the message handler.
    pub fn function(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(ffi::function(self)) }
    }

    /// The category given to the message handler.
    pub fn category(&self) -> &'a CStr {
        unsafe { CStr::from_ptr(ffi::category(self)) }
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QMessageLogContext is trivial.
unsafe impl ExternType for QMessageLogContext<'_> {
    type Id = type_id!("QMessageLogContext");
    type Kind = cxx::kind::Trivial;
}

use crate::const_assert;
pub use ffi::{q_format_log_message, q_set_message_pattern, qt_message_output, QtMsgType};

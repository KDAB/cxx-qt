// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::ffi::c_char;
use std::ffi::CStr;
use std::marker::PhantomData;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum QtMsgType {
        QtDebugMsg = 0,
        QtInfoMsg = 4,
        QtWarningMsg = 1,
        QtFatalMsg = 3,
        QtCriticalMsg = 2,
        QtSystemMsg = 2
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        include!("cxx-qt-lib/qtlogging.h");
        type QMessageLogContext<'a> = crate::QMessageLogContext<'a>;
        type QtMsgType;

        fn qt_message_output(msgType: QtMsgType, context: &QMessageLogContext, string: &QString);

        #[cxx_name = "qmessagelogcontext_line"]
        fn line(context: &QMessageLogContext) -> i32;

        #[cxx_name = "qmessagelogcontext_set_line"]
        fn set_line(context: &mut QMessageLogContext, line: i32);

        #[cxx_name = "qmessagelogcontext_file"]
        unsafe fn file(context: &QMessageLogContext) -> *const c_char;

        #[cxx_name = "qmessagelogcontext_set_file"]
        unsafe fn set_file(context: &mut QMessageLogContext, file: *const c_char);

        #[cxx_name = "qmessagelogcontext_function"]
        unsafe fn function(context: &QMessageLogContext) -> *const c_char;

        #[cxx_name = "qmessagelogcontext_set_function"]
        unsafe fn set_function(context: &mut QMessageLogContext, function: *const c_char);

        #[cxx_name = "qmessagelogcontext_category"]
        unsafe fn category(context: &QMessageLogContext) -> *const c_char;

        #[cxx_name = "qmessagelogcontext_set_category"]
        unsafe fn set_category(context: &mut QMessageLogContext, category: *const c_char);
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qmessagelogcontext_default"]
        fn construct() -> QMessageLogContext<'static>;
    }
}

#[repr(C)]
pub struct QMessageLogContext<'a> {
    version: i32,
    line: i32,
    file: &'a *const c_char,
    function: &'a *const c_char,
    category: &'a *const c_char
}

impl Default for QMessageLogContext<'_> {
    fn default() -> Self {
        ffi::qmessagelogcontext_default()
    }
}

impl<'a> QMessageLogContext<'a> {
    pub fn line(&self) -> i32 {
        ffi::line(&self)
    }

    pub fn set_line(&mut self, line: i32) {
        ffi::set_line(self, line);
    }

    pub fn file(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(ffi::file(self))
        }
    }

    pub fn set_file(&mut self, file: &'a CStr) {
        unsafe {
            ffi::set_file(self, file.as_ptr());
        }
    }

    pub fn function(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(ffi::function(self))
        }
    }

    pub fn set_function(&mut self, function: &'a CStr) {
        unsafe {
            ffi::set_function(self, function.as_ptr());
        }
    }

    pub fn category(&self) -> &CStr {
        unsafe {
            CStr::from_ptr(ffi::category(self))
        }
    }

    pub fn set_category(&mut self, category: &'a CStr) {
        unsafe {
            ffi::set_category(self, category.as_ptr());
        }
    }
}

// Safety:
//
// Static checks on the C++ side ensure that QMessageLogContext is trivial.
unsafe impl ExternType for QMessageLogContext<'_> {
    type Id = type_id!("QMessageLogContext");
    type Kind = cxx::kind::Trivial;
}

pub use ffi::{
    QtMsgType, qt_message_output
};


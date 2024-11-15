// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Seems to be a Clippy false positive, we need these lifetime declarations
#![allow(clippy::needless_lifetimes)]

use cxx_qt_lib::{QAnyStringView, QString};

#[cxx::bridge]
mod qanystringview_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qanystringview.h");
        type QAnyStringView<'a> = cxx_qt_lib::QAnyStringView<'a>;

        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    extern "Rust" {
        fn construct_qanystringview(str: &str) -> QAnyStringView;
    }

    // This method must be unsafe otherwise we hit
    // must be `unsafe fn` in order to expose explicit lifetimes to C++
    //
    // But then Rust complains about unused unsafe so we need to allow for this
    #[allow(unused_unsafe)]
    extern "Rust" {
        unsafe fn construct_qanystringview_qstring<'a>(str: &'a QString) -> QAnyStringView<'a>;
        unsafe fn clone_qanystringview<'a>(l: &QAnyStringView<'a>) -> QAnyStringView<'a>;
    }
}

fn construct_qanystringview<'a>(str: &'a str) -> QAnyStringView<'a> {
    QAnyStringView::from(str)
}

fn construct_qanystringview_qstring<'a>(str: &'a QString) -> QAnyStringView<'a> {
    QAnyStringView::from(str)
}

fn clone_qanystringview<'a>(l: &QAnyStringView<'a>) -> QAnyStringView<'a> {
    l.clone()
}

// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a custom parent class can be used to inherit from a QQuickItem based object.

/// A CXX-Qt bridge which shows a custom parent class can be used
#[cxx_qt::bridge(cxx_file_stem = "custom_parent_class")]
mod ffi {
    unsafe extern "C++" {
        /// QColor from cxx_qt_lib
        type QColor = cxx_qt_lib::QColor;
        include!("cxx-qt-lib/qcolor.h");

        /// QRectF from cxx_qt_lib
        type QRectF = cxx_qt_lib::QRectF;
        include!("cxx-qt-lib/qrectf.h");

        /// QSizeF from cxx_qt_lib
        type QSizeF = cxx_qt_lib::QSizeF;
        include!("cxx-qt-lib/qsizef.h");
    }

    // Define the API from QPainter that we need
    unsafe extern "C++" {
        /// QPainter from Qt
        type QPainter;
        include!(<QtGui/QPainter>);

        /// QPainter::fillRect from Qt
        #[rust_name = "fill_rect"]
        fn fillRect(self: Pin<&mut QPainter>, rectangle: &QRectF, color: &QColor);
    }

    // Define the API from QtQuick that we need
    unsafe extern "C++" {
        /// Define QQuickItem as a type
        type QQuickItem;
        include!(<QtQuick/QQuickItem>);

        include!(<QtQuick/QQuickPaintedItem>);
    }

    /// A struct which inherits from QQuickPaintedItem
    ///
    /// Which has a parent of the type QQuickItem rather than QObject.
    #[cxx_qt::qobject(
        base = "QQuickPaintedItem",
        qml_uri = "com.kdab.cxx_qt.demo",
        qml_version = "1.0"
    )]
    #[derive(Default)]
    pub struct CustomParentClass {
        #[qproperty]
        color: QColor,
    }

    impl cxx_qt::Constructor<()> for qobject::CustomParentClass {}

    unsafe extern "RustQt" {
        /// Override QQuickPaintedItem::paint to draw two rectangles in Rust using QPainter
        #[qinvokable(cxx_override)]
        unsafe fn paint(self: Pin<&mut qobject::CustomParentClass>, painter: *mut QPainter);

        // Define that we need to inherit size() from the base class
        #[inherit]
        fn size(self: &qobject::CustomParentClass) -> QSizeF;

        // Define that we need to inherit update() from the base class
        #[inherit]
        fn update(self: Pin<&mut qobject::CustomParentClass>);
    }
}

use core::pin::Pin;
use cxx_qt_lib::QRectF;

// TODO: this will change to qobject::RustInvokables once
// https://github.com/KDAB/cxx-qt/issues/559 is done
impl ffi::CustomParentClassQt {
    /// Override QQuickPaintedItem::paint to draw two rectangles in Rust using QPainter
    fn paint(self: Pin<&mut Self>, painter: *mut ffi::QPainter) {
        // We need to convert the *mut QPainter to a Pin<&mut QPainter> so that we can reach the methods
        if let Some(painter) = unsafe { painter.as_mut() } {
            let mut pinned_painter = unsafe { Pin::new_unchecked(painter) };

            // Now pinned painter can be used as normal
            // to render a rectangle with two colours
            let size = self.as_ref().size();
            pinned_painter.as_mut().fill_rect(
                &QRectF::new(0.0, 0.0, size.width() / 2.0, size.height()),
                self.as_ref().color(),
            );
            let darker_color = self.as_ref().color().darker(150);
            pinned_painter.as_mut().fill_rect(
                &QRectF::new(size.width() / 2.0, 0.0, size.width() / 2.0, size.height()),
                &darker_color,
            );
        }
    }
}

impl cxx_qt::Constructor<()> for qobject::CustomParentClass {
    type NewArguments = ();
    type BaseArguments = (*mut ffi::QQuickItem,);
    type InitializeArguments = ();

    fn route_arguments(
        _args: (),
    ) -> (
        Self::NewArguments,
        Self::BaseArguments,
        Self::InitializeArguments,
    ) {
        ((), (core::ptr::null_mut(),), ())
    }

    fn new((): ()) -> CustomParentClass {
        CustomParentClass::default()
    }

    fn initialize(self: core::pin::Pin<&mut Self>, _arguments: Self::InitializeArguments) {
        self.on_color_changed(|qobject| qobject.update()).release();
    }
}

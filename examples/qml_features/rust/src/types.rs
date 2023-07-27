// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! This example shows how a custom type can be used with a QVariant

/// A struct which is a custom type we want to use with a QVariant
// ANCHOR: book_macro_code
#[repr(C)]
pub struct CustomStruct {
    value: i32,
}

unsafe impl cxx::ExternType for CustomStruct {
    type Id = cxx::type_id!("CustomStruct");
    type Kind = cxx::kind::Trivial;
}

// ANCHOR: book_qvariantvalue_impl
impl cxx_qt_lib::QVariantValue for ffi::CustomStruct {
    fn can_convert(variant: &cxx_qt_lib::QVariant) -> bool {
        ffi::qvariant_can_convert_custom_type(variant)
    }

    fn construct(value: &Self) -> cxx_qt_lib::QVariant {
        ffi::qvariant_construct_custom_type(value)
    }

    fn value_or_default(variant: &cxx_qt_lib::QVariant) -> Self {
        ffi::qvariant_value_or_default_custom_type(variant)
    }
}
// ANCHOR_END: book_qvariantvalue_impl

/// A CXX-Qt bridge which shows how a custom type can be used with a QVariant
// ANCHOR: book_cxx_file_stem
#[cxx_qt::bridge(cxx_file_stem = "types")]
pub mod ffi {
    // ANCHOR_END: book_cxx_file_stem
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        /// QPointF from cxx_qt_lib
        type QPointF = cxx_qt_lib::QPointF;
        include!("cxx-qt-lib/qurl.h");
        /// QUrl from cxx_qt_lib
        type QUrl = cxx_qt_lib::QUrl;
        include!("cxx-qt-lib/qvariant.h");
        /// QVariant from cxx_qt_lib
        type QVariant = cxx_qt_lib::QVariant;
    }

    unsafe extern "C++" {
        include!("custom_object.h");
        /// CustomStruct which is a custom C++ type
        type CustomStruct = super::CustomStruct;

        /// Return whether the given QVariant can be converted into a CustomStruct
        #[rust_name = "qvariant_can_convert_custom_type"]
        fn qvariantCanConvertCustomStruct(variant: &QVariant) -> bool;
    }

    // We can reuse the templates from cxx-qt-lib for the construct and value
    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");

        /// Construct a QVariant of CustomStruct
        #[rust_name = "qvariant_construct_custom_type"]
        fn qvariantConstruct(value: &CustomStruct) -> QVariant;
        /// Retrieve the CustomStruct or default from a QVariant
        #[rust_name = "qvariant_value_or_default_custom_type"]
        fn qvariantValueOrDefault(variant: &QVariant) -> CustomStruct;
    }

    unsafe extern "RustQt" {
        #[cxx_qt::qobject(qml_element)]
        #[qproperty(bool, boolean)]
        #[qproperty(QPointF, point)]
        #[qproperty(QUrl, url)]
        #[qproperty(i32, custom_value)]
        type Types = super::TypesRust;

        /// Load the value from a QVariant
        #[qinvokable]
        fn load_from_variant(self: Pin<&mut Types>, variant: &QVariant);

        /// Toggle the boolean Q_PROPERTY
        #[qinvokable]
        fn toggle_boolean(self: Pin<&mut Types>);
    }
}

use core::pin::Pin;
use cxx_qt_lib::{QPointF, QUrl, QVariant};

/// A QObject which shows custom types
pub struct TypesRust {
    boolean: bool,
    point: QPointF,
    url: QUrl,
    custom_value: i32,
}

impl Default for TypesRust {
    fn default() -> Self {
        Self {
            boolean: false,
            point: QPointF::new(1.0, 2.0),
            url: QUrl::from("https://kdab.com"),
            custom_value: 0,
        }
    }
}

impl ffi::Types {
    /// Load the value from a QVariant
    pub fn load_from_variant(self: Pin<&mut Self>, variant: &QVariant) {
        if let Some(boolean) = variant.value::<bool>() {
            self.set_boolean(boolean);
        } else if let Some(point) = variant.value::<QPointF>() {
            self.set_point(point);
        } else if let Some(url) = variant.value::<QUrl>() {
            self.set_url(url);
        } else if let Some(custom) = variant.value::<CustomStruct>() {
            self.set_custom_value(custom.value);
        } else {
            println!("Unknown QVariant type to load from");
        }
    }

    /// Toggle the boolean Q_PROPERTY
    pub fn toggle_boolean(self: Pin<&mut Self>) {
        let new_boolean = !self.as_ref().boolean();
        self.set_boolean(new_boolean);
    }
}
// ANCHOR_END: book_macro_code

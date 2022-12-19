// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

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

    fn value(variant: &cxx_qt_lib::QVariant) -> Self {
        ffi::qvariant_value_custom_type(variant)
    }
}
// ANCHOR_END: book_qvariantvalue_impl

// ANCHOR: book_cxx_file_stem
#[cxx_qt::bridge(cxx_file_stem = "types")]
mod ffi {
    // ANCHOR_END: book_cxx_file_stem
    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPointF = cxx_qt_lib::QPointF;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    unsafe extern "C++" {
        include!("custom_object.h");
        type CustomStruct = super::CustomStruct;

        #[rust_name = "qvariant_can_convert_custom_type"]
        fn qvariantCanConvertCustomStruct(variant: &QVariant) -> bool;
    }

    // We can reuse the templates from cxx-qt-lib for the construct and value
    #[namespace = "rust::cxxqtlib1::qvariant"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qvariant.h");

        #[rust_name = "qvariant_construct_custom_type"]
        fn qvariantConstruct(value: &CustomStruct) -> QVariant;
        #[rust_name = "qvariant_value_custom_type"]
        fn qvariantValue(variant: &QVariant) -> CustomStruct;
    }

    #[cxx_qt::qobject]
    pub struct Types {
        #[qproperty]
        boolean: bool,
        #[qproperty]
        point: QPointF,
        #[qproperty]
        url: QUrl,
        #[qproperty]
        custom_value: i32,
    }

    impl Default for Types {
        fn default() -> Self {
            Self {
                boolean: false,
                point: QPointF::new(1.0, 2.0),
                url: QUrl::from("https://kdab.com"),
                custom_value: 0,
            }
        }
    }

    impl qobject::Types {
        #[qinvokable]
        pub fn load_from_variant(self: Pin<&mut Self>, variant: &QVariant) {
            if let Some(boolean) = variant.try_value::<bool>() {
                self.set_boolean(boolean);
            } else if let Some(point) = variant.try_value::<QPointF>() {
                self.set_point(point);
            } else if let Some(url) = variant.try_value::<QUrl>() {
                self.set_url(url);
            } else if let Some(custom) = variant.try_value::<CustomStruct>() {
                self.set_custom_value(custom.value);
            } else {
                println!("Unknown QVariant type to load from");
            }
        }

        #[qinvokable]
        pub fn toggle_boolean(self: Pin<&mut Self>) {
            let new_boolean = !self.as_ref().boolean();
            self.set_boolean(new_boolean);
        }
    }
}
// ANCHOR_END: book_macro_code

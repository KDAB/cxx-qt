// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
// ANCHOR: book_cxx_file_stem
#[cxx_qt::bridge(cxx_file_stem = "types")]
mod ffi {
    // ANCHOR_END: book_cxx_file_stem
    use cxx_qt_lib::QVariantValue;

    unsafe extern "C++" {
        include!("cxx-qt-lib/qpoint.h");
        type QPointF = cxx_qt_lib::QPointF;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
        include!("cxx-qt-lib/qvariant.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    // TODO: should we show how to do custom types?
    #[cxx_qt::qobject]
    pub struct Types {
        #[qproperty]
        boolean: bool,
        #[qproperty]
        point: QPointF,
        #[qproperty]
        url: QUrl,
    }

    impl Default for Types {
        fn default() -> Self {
            Self {
                boolean: false,
                point: QPointF::new(1.0, 2.0),
                url: QUrl::from("https://kdab.com"),
            }
        }
    }

    impl qobject::Types {
        #[qinvokable]
        pub fn load_from_variant(self: Pin<&mut Self>, variant: &QVariant) {
            match variant.value() {
                QVariantValue::Bool(boolean) => self.set_boolean(boolean),
                QVariantValue::QPointF(point) => self.set_point(point),
                QVariantValue::QUrl(url) => self.set_url(url),
                _ => println!("Unknown QVariant type to load from"),
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

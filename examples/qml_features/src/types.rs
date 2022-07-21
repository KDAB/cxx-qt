// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
mod types {
    use cxx_qt_lib::QVariantValue;

    #[namespace = ""]
    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    pub struct Data {
        variant: UniquePtr<QVariant>,
    }

    impl Default for Data {
        fn default() -> Self {
            Data {
                variant: QVariant::from(1_i32),
            }
        }
    }

    #[derive(Default)]
    pub struct RustObj;

    impl RustObj {
        #[invokable]
        pub fn test_variant_property(&self, cpp: &mut CppObj) {
            match cpp.variant().value() {
                QVariantValue::Bool(b) => {
                    cpp.set_variant(QVariant::from(!b).as_ref().unwrap());
                }
                QVariantValue::I32(i) => {
                    cpp.set_variant(QVariant::from(i * 2).as_ref().unwrap());
                }
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[invokable]
        pub fn test_variant_invokable(&self, variant: &QVariant) -> UniquePtr<QVariant> {
            match variant.value() {
                QVariantValue::Bool(b) => QVariant::from(!b),
                QVariantValue::I32(i) => QVariant::from(i * 2),
                _ => panic!("Incorrect variant type!"),
            }
        }
    }
}
// ANCHOR_END: book_macro_code

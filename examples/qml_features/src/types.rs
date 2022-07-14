// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
mod types {
    extern "Qt" {
        use cxx_qt_lib::{QVariant, QVariantValue};

        pub struct Data {
            variant: QVariant,
        }

        impl Default for Data {
            fn default() -> Self {
                Data {
                    variant: QVariant::from(1_i32),
                }
            }
        }

        #[derive(Default)]
        struct RustObj;

        impl RustObj {
            #[invokable]
            fn test_variant_property(&self, cpp: &mut CppObj) {
                match cpp.variant().value() {
                    QVariantValue::Bool(b) => {
                        cpp.set_variant(QVariant::from(!b));
                    }
                    QVariantValue::I32(i) => {
                        cpp.set_variant(QVariant::from(i * 2));
                    }
                    _ => panic!("Incorrect variant type!"),
                }
            }

            #[invokable]
            fn test_variant_invokable(&self, variant: &QVariant) -> QVariant {
                match variant.value() {
                    QVariantValue::Bool(b) => QVariant::from(!b),
                    QVariantValue::I32(i) => QVariant::from(i * 2),
                    _ => panic!("Incorrect variant type!"),
                }
            }
        }
    }
}
// ANCHOR_END: book_macro_code

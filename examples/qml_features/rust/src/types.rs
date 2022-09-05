// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_macro_code
#[cxx_qt::bridge]
mod ffi {
    use cxx_qt_lib::QVariantValue;

    unsafe extern "C++" {
        include!("cxx-qt-lib/include/qt_types.h");
        type QVariant = cxx_qt_lib::QVariant;
    }

    #[cxx_qt::qobject]
    pub struct Types {
        #[qproperty]
        variant: UniquePtr<QVariant>,
    }

    impl Default for Types {
        fn default() -> Self {
            Self {
                variant: QVariant::from(1_i32),
            }
        }
    }

    impl cxx_qt::QObject<Types> {
        #[qinvokable]
        pub fn test_variant_property(mut self: Pin<&mut Self>) {
            match self.get_variant().value() {
                QVariantValue::Bool(b) => {
                    self.as_mut().set_variant(QVariant::from(!b));
                }
                QVariantValue::I32(i) => {
                    self.as_mut().set_variant(QVariant::from(i * 2));
                }
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[qinvokable]
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

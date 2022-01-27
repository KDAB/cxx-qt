// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx_qt::make_qobject;

// ANCHOR: book_macro_code
#[make_qobject]
mod types {
    use cxx_qt_lib::{QVariant, Variant, VariantValue};

    pub struct Data {
        variant: Variant,
    }

    impl Default for Data {
        fn default() -> Self {
            Data {
                variant: Variant::from(1_i32),
            }
        }
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_variant_property(&self, cpp: &mut CppObj) {
            match cpp.variant().value() {
                VariantValue::Bool(b) => {
                    cpp.set_variant(Variant::from(!b));
                }
                VariantValue::I32(i) => {
                    cpp.set_variant(Variant::from(i * 2));
                }
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[invokable]
        fn test_variant_invokable(&self, variant: &QVariant) -> Variant {
            match variant.to_rust().value() {
                VariantValue::Bool(b) => Variant::from(!b),
                VariantValue::I32(i) => Variant::from(i * 2),
                _ => panic!("Incorrect variant type!"),
            }
        }
    }
}
// ANCHOR_END: book_macro_code

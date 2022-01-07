// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::make_qobject;

#[make_qobject]
mod mock_qt_types {
    use cxx_qt_lib::{let_qvariant, QPointF, QVariant, Variant, VariantImpl};

    pub struct Data {
        pointf: QPointF,
        variant: Variant,
    }

    impl Default for Data {
        fn default() -> Self {
            Data {
                pointf: QPointF::new(1.0, 3.0),
                variant: Variant::from_i32(1),
            }
        }
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_pointf_property(&self, cpp: &mut CppObj) {
            let mut point = *cpp.pointf();
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 3.0);
            cpp.set_pointf(&point);
        }

        #[invokable]
        fn test_pointf_invokable(&self, point: &QPointF) -> QPointF {
            let mut point = *point;
            point.set_x(point.x() * 2.0);
            point.set_y(point.y() * 3.0);
            point
        }

        #[invokable]
        fn test_variant_property(&self, cpp: &mut CppObj) {
            match *cpp.variant().to_rust() {
                VariantImpl::Bool(b) => {
                    let new_variant = Variant::from_bool(!b);
                    let_qvariant!(new_qvariant = &new_variant);
                    cpp.set_variant(&new_qvariant);
                }
                VariantImpl::I8(i) => {
                    let new_variant = Variant::from_i8(i * 2);
                    let_qvariant!(new_qvariant = &new_variant);
                    cpp.set_variant(&new_qvariant);
                }
                VariantImpl::I16(i) => {
                    let new_variant = Variant::from_i16(i * 2);
                    let_qvariant!(new_qvariant = &new_variant);
                    cpp.set_variant(&new_qvariant);
                }
                VariantImpl::I32(i) => {
                    let new_variant = Variant::from_i32(i * 2);
                    let_qvariant!(new_qvariant = &new_variant);
                    cpp.set_variant(&new_qvariant);
                }
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[invokable]
        fn test_variant_invokable(&self, variant: &QVariant) -> Variant {
            match *variant.to_rust() {
                VariantImpl::Bool(b) => Variant::from_bool(!b),
                VariantImpl::I8(i) => Variant::from_i8(i * 2),
                VariantImpl::I16(i) => Variant::from_i16(i * 2),
                VariantImpl::I32(i) => Variant::from_i32(i * 2),
                _ => panic!("Incorrect variant type!"),
            }
        }
    }
}

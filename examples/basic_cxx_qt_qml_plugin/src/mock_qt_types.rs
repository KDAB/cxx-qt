// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::make_qobject;

#[make_qobject]
mod mock_qt_types {
    use cxx_qt_lib::{QPoint, QPointF, QSize, QSizeF, Variant, VariantImpl};

    pub struct Data {
        point: QPoint,
        pointf: QPointF,
        size: QSize,
        sizef: QSizeF,
        variant: Variant,
    }

    impl Default for Data {
        fn default() -> Self {
            Data {
                point: QPoint::new(1, 3),
                pointf: QPointF::new(1.0, 3.0),
                size: QSize::new(1, 3),
                sizef: QSizeF::new(1.0, 3.0),
                variant: Variant::from_i32(1),
            }
        }
    }

    #[derive(Default)]
    struct RustObj;

    impl RustObj {
        #[invokable]
        fn test_point_property(&self, cpp: &mut CppObj) {
            let mut point = *cpp.point();
            point.set_x(point.x() * 2);
            point.set_y(point.y() * 3);
            cpp.set_point(&point);
        }

        #[invokable]
        fn test_point_invokable(&self, point: &QPoint) -> QPoint {
            let mut point = *point;
            point.set_x(point.x() * 2);
            point.set_y(point.y() * 3);
            point
        }

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
        fn test_size_property(&self, cpp: &mut CppObj) {
            let mut size = *cpp.size();
            size.set_width(size.width() * 2);
            size.set_height(size.height() * 3);
            cpp.set_size(&size);
        }

        #[invokable]
        fn test_size_invokable(&self, size: &QSize) -> QSize {
            let mut size = *size;
            size.set_width(size.width() * 2);
            size.set_height(size.height() * 3);
            size
        }

        #[invokable]
        fn test_sizef_property(&self, cpp: &mut CppObj) {
            let mut size = *cpp.sizef();
            size.set_width(size.width() * 2.0);
            size.set_height(size.height() * 3.0);
            cpp.set_sizef(&size);
        }

        #[invokable]
        fn test_sizef_invokable(&self, size: &QSizeF) -> QSizeF {
            let mut size = *size;
            size.set_width(size.width() * 2.0);
            size.set_height(size.height() * 3.0);
            size
        }

        #[invokable]
        fn test_variant_property(&self, cpp: &mut CppObj) {
            match *cpp.variant() {
                VariantImpl::Bool(b) => cpp.set_variant(&Variant::from_bool(!b)),
                VariantImpl::F32(f) => cpp.set_variant(&Variant::from_f32(f * 2.0)),
                VariantImpl::F64(d) => cpp.set_variant(&Variant::from_f64(d * 2.0)),
                VariantImpl::I8(i) => cpp.set_variant(&Variant::from_i8(i * 2)),
                VariantImpl::I16(i) => cpp.set_variant(&Variant::from_i16(i * 2)),
                VariantImpl::I32(i) => cpp.set_variant(&Variant::from_i32(i * 2)),
                VariantImpl::U8(i) => cpp.set_variant(&Variant::from_u8(i * 2)),
                VariantImpl::U16(i) => cpp.set_variant(&Variant::from_u16(i * 2)),
                VariantImpl::U32(i) => cpp.set_variant(&Variant::from_u32(i * 2)),
                _ => panic!("Incorrect variant type!"),
            }
        }

        #[invokable]
        fn test_variant_invokable(&self, variant: &Variant) -> Variant {
            match **variant {
                VariantImpl::Bool(b) => Variant::from_bool(!b),
                VariantImpl::F32(f) => Variant::from_f32(f * 2.0),
                VariantImpl::F64(d) => Variant::from_f64(d * 2.0),
                VariantImpl::I8(i) => Variant::from_i8(i * 2),
                VariantImpl::I16(i) => Variant::from_i16(i * 2),
                VariantImpl::I32(i) => Variant::from_i32(i * 2),
                VariantImpl::U8(i) => Variant::from_u8(i * 2),
                VariantImpl::U16(i) => Variant::from_u16(i * 2),
                VariantImpl::U32(i) => Variant::from_u32(i * 2),
                _ => panic!("Incorrect variant type!"),
            }
        }
    }
}

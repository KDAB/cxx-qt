// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx_qt::make_qobject;

#[make_qobject]
mod mock_qt_types {
    use cxx_qt_lib::{
        let_qcolor, let_qvariant, Color, ColorImpl, QColor, QPoint, QPointF, QRect, QRectF, QSize,
        QSizeF, QVariant, Variant, VariantImpl,
    };

    pub struct Data {
        color: Color,
        point: QPoint,
        pointf: QPointF,
        rect: QRect,
        rectf: QRectF,
        size: QSize,
        sizef: QSizeF,
        variant: Variant,
    }

    impl Default for Data {
        fn default() -> Self {
            Data {
                color: Color::from_argb(255, 255, 0, 0),
                point: QPoint::new(1, 3),
                pointf: QPointF::new(1.0, 3.0),
                rect: QRect::new(1, 2, 3, 4),
                rectf: QRectF::new(1.0, 2.0, 3.0, 4.0),
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
        fn test_color_property(&self, cpp: &mut CppObj) {
            match *cpp.color().to_rust() {
                ColorImpl::ARGB { .. } => {
                    let new_color = Color::from_argb(255, 0, 0, 255);
                    let_qcolor!(new_qcolor = &new_color);
                    cpp.set_color(&new_qcolor);
                }
                _ => panic!("Incorrect color type!"),
            }
        }

        #[invokable]
        fn test_color_invokable(&self, color: &QColor) -> Color {
            match *color.to_rust() {
                ColorImpl::ARGB { .. } => Color::from_argb(255, 0, 255, 0),
                _ => panic!("Incorrect color type!"),
            }
        }

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
        fn test_rect_property(&self, cpp: &mut CppObj) {
            let mut rect = *cpp.rect();
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2);
            rect.set_y(rect.y() * 3);
            rect.set_width(width * 4);
            rect.set_height(height * 5);
            cpp.set_rect(&rect);
        }

        #[invokable]
        fn test_rect_invokable(&self, rect: &QRect) -> QRect {
            let mut rect = *rect;
            // Copy width and height, otherwise when we adjust the x and y it affects the width and height
            let (width, height) = (rect.width(), rect.height());
            rect.set_x(rect.x() * 2);
            rect.set_y(rect.x() * 3);
            rect.set_width(width * 4);
            rect.set_height(height * 5);
            rect
        }

        #[invokable]
        fn test_rectf_property(&self, cpp: &mut CppObj) {
            let mut rect = *cpp.rectf();
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.y() * 3.0);
            rect.set_width(rect.width() * 4.0);
            rect.set_height(rect.height() * 5.0);
            cpp.set_rectf(&rect);
        }

        #[invokable]
        fn test_rectf_invokable(&self, rect: &QRectF) -> QRectF {
            let mut rect = *rect;
            rect.set_x(rect.x() * 2.0);
            rect.set_y(rect.x() * 3.0);
            rect.set_width(rect.width() * 4.0);
            rect.set_height(rect.height() * 5.0);
            rect
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
            match *cpp.variant().to_rust() {
                VariantImpl::Bool(b) => {
                    let new_variant = Variant::from_bool(!b);
                    let_qvariant!(new_qvariant = &new_variant);
                    cpp.set_variant(&new_qvariant);
                }
                VariantImpl::F32(f) => {
                    let new_variant = Variant::from_f32(f * 2.0);
                    let_qvariant!(new_qvariant = &new_variant);
                    cpp.set_variant(&new_qvariant);
                }
                VariantImpl::F64(d) => {
                    let new_variant = Variant::from_f64(d * 2.0);
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
                VariantImpl::U8(i) => {
                    let new_variant = Variant::from_u8(i * 2);
                    let_qvariant!(new_qvariant = &new_variant);
                    cpp.set_variant(&new_qvariant);
                }
                VariantImpl::U16(i) => {
                    let new_variant = Variant::from_u16(i * 2);
                    let_qvariant!(new_qvariant = &new_variant);
                    cpp.set_variant(&new_qvariant);
                }
                VariantImpl::U32(i) => {
                    let new_variant = Variant::from_u32(i * 2);
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

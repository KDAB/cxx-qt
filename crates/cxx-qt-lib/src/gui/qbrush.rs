// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type BrushStyle = crate::BrushStyle;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qbrush.h");
        type QBrush = super::QBrush;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;
        include!("cxx-qt-lib/qimage.h");
        type QImage = crate::QImage;

        /// Returns true if the brush is fully opaque otherwise false
        #[rust_name = "is_opaque"]
        fn isOpaque(self: &QBrush) -> bool;

        /// Returns the brush color.
        fn color(self: &QBrush) -> &QColor;

        /// Sets the brush style to style.
        #[rust_name = "set_style"]
        fn setStyle(self: &mut QBrush, style: BrushStyle);

        /// Sets the brush color to the given color.
        #[rust_name = "set_color"]
        fn setColor(self: &mut QBrush, color: &QColor);

        /// Sets the brush image to image. The style is set to Qt::TexturePattern.
        #[rust_name = "set_texture_image"]
        fn setTextureImage(self: &mut QBrush, image: &QImage);

        /// Returns the brush style.
        fn style(self: &QBrush) -> BrushStyle;

        /// Returns the custom brush pattern, or a null image if no custom brush pattern has been set.
        #[rust_name = "texture_image"]
        fn textureImage(self: &QBrush) -> QImage;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qbrush_init_default"]
        fn construct() -> QBrush;

        #[doc(hidden)]
        #[rust_name = "qbrush_drop"]
        fn drop(brush: &mut QBrush);

        #[doc(hidden)]
        #[rust_name = "qbrush_clone"]
        fn construct(brush: &QBrush) -> QBrush;
    }
}

#[repr(C)]
pub struct QBrush {
    _cspec: MaybeUninit<usize>,
}

impl Default for QBrush {
    /// Constructs a default black brush with the style Qt::NoBrush (i.e. this brush will not fill shapes).
    fn default() -> Self {
        ffi::qbrush_init_default()
    }
}

impl Drop for QBrush {
    fn drop(&mut self) {
        ffi::qbrush_drop(self);
    }
}

impl Clone for QBrush {
    fn clone(&self) -> Self {
        ffi::qbrush_clone(self)
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QBrush {
    type Id = type_id!("QBrush");
    type Kind = cxx::kind::Trivial;
}

// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::mem::MaybeUninit;

#[cxx::bridge]
mod ffi {
    #[namespace = "Qt"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type TransformationMode = crate::TransformationMode;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qimage.h");
        type QImage = super::QImage;
        include!("cxx-qt-lib/qsize.h");
        type QSize = crate::QSize;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;

        /// Returns true if all the colors in the image are shades of gray
        #[rust_name = "all_gray"]
        fn allGray(self: &QImage) -> bool;

        /// Returns a sub-area of the image as a new image.
        fn copy(self: &QImage, rect: &QRect) -> QImage;

        /// Returns the size of the color table for the image.
        #[rust_name = "color_count"]
        fn colorCount(self: &QImage) -> i32;

        /// Returns the depth of the image.
        fn depth(self: &QImage) -> i32;

        /// Returns the number of pixels that fit horizontally in a physical meter.
        #[rust_name = "dots_per_meterx"]
        fn dotsPerMeterX(self: &QImage) -> i32;

        /// Returns the number of pixels that fit vertically in a physical meter.
        #[rust_name = "dots_per_metery"]
        fn dotsPerMeterY(self: &QImage) -> i32;

        /// Fills the entire image with the given color.
        fn fill(self: &mut QImage, color: &QColor);

        /// Whether the QImage is null.
        ///
        /// This means that the QImage has all parameters set to zero and no allocated data.
        #[rust_name = "is_null"]
        fn isNull(self: &QImage) -> bool;

        /// For 32-bit images, this function is equivalent to allGray().
        /// For color indexed images, this function returns true if color(i) is QRgb(i, i, i)
        /// for all indexes of the color table; otherwise returns false.
        #[rust_name = "is_gray_scale"]
        fn isGrayscale(self: &QImage) -> bool;

        /// Returns true if the image has a format that respects the alpha channel, otherwise returns false.
        #[rust_name = "has_alpha_channel"]
        fn hasAlphaChannel(self: &QImage) -> bool;

        /// Returns the height of the image.
        fn height(self: &QImage) -> i32;

        /// Returns the enclosing rectangle (0, 0, width(), height()) of the image.
        fn rect(self: &QImage) -> QRect;

        /// Returns a scaled copy of the image. The returned image is scaled to the given height using the specified transformation mode.
        #[rust_name = "scaled_to_height"]
        fn scaledToHeight(self: &QImage, width: i32, mode: TransformationMode) -> QImage;

        /// Returns a scaled copy of the image. The returned image is scaled to the given width using the specified transformation mode.
        #[rust_name = "scaled_to_width"]
        fn scaledToWidth(self: &QImage, width: i32, mode: TransformationMode) -> QImage;

        /// Resizes the color table to contain colorCount entries.
        #[rust_name = "set_color_count"]
        fn setColorCount(self: &mut QImage, colorCount: i32);

        /// Sets the alpha channel of this image to the given alphaChannel.
        #[rust_name = "set_alpha_channel"]
        fn setAlphaChannel(self: &mut QImage, alphaChannel: &QImage);

        /// Sets the number of pixels by which the image is intended to be offset by when positioning relative to other images, to offset.
        #[rust_name = "set_offset"]
        fn setOffset(self: &mut QImage, point: &QPoint);

        /// Sets the pixel color at (x, y) to color.
        #[rust_name = "set_pixel_color"]
        fn setPixelColor(self: &mut QImage, x: i32, y: i32, color: &QColor);

        /// Returns the size of the image.
        fn size(self: &QImage) -> QSize;

        /// Swaps image other with this image. This operation is very fast and never fails.
        fn swap(self: &mut QImage, other: &mut QImage);

        /// Returns the number of pixels by which the image is intended to be offset by when positioning relative to other images.
        fn offset(self: &QImage) -> QPoint;

        /// Returns the color of the pixel at coordinates (x, y) as a QColor.
        #[rust_name = "pixel_color"]
        fn pixelColor(self: &QImage, x: i32, y: i32) -> QColor;

        /// Returns the pixel index at (x, y).
        #[rust_name = "pixel_index"]
        fn pixelIndex(self: &QImage, x: i32, y: i32) -> i32;

        /// Returns true if pos is a valid coordinate pair within the image.
        fn valid(self: &QImage, x: i32, y: i32) -> bool;

        /// Returns the width of the image.
        fn width(self: &QImage) -> i32;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "qimage_drop"]
        fn drop(image: &mut QImage);

        #[doc(hidden)]
        #[rust_name = "qimage_init_from_data"]
        fn qimageInitFromData(data: &[u8], format: &str) -> QImage;

        #[doc(hidden)]
        #[rust_name = "qimage_cache_key"]
        fn qimageCacheKey(image: &QImage) -> i64;
    }
}
/*
#[cfg(qt_version_major = "6")]
#[cxx::bridge]
mod ffi_qt_6 {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qimage.h");
        type QImage = super::QImage;

        /// Mirrors of the image in the horizontal and/or the vertical direction depending on whether horizontal and vertical are set to true or false.
        // #[cfg(qt_version_major = "6")]
        fn mirror(self: &mut QImage, horizontal: bool, vertical: bool);
    }
}
*/
/// > ⚠ **Warning**: The QImage API in CXX-Qt-lib is not yet complete and subject to change.
///
/// This struct is the Rust representation of the [`QImage`](https://doc.qt.io/qt-6/qimage.html)
/// class.
///
/// It provides a way to store and manipulate images in a hardware-independent manner.
#[repr(C)]
pub struct QImage {
    // Static checks on the C++ side ensure this is true.
    // See qcolor.cpp
    #[cfg(qt_version_major = "5")]
    _data: MaybeUninit<[usize; 4]>,
    #[cfg(qt_version_major = "6")]
    _data: MaybeUninit<[usize; 3]>,
}

// Safety:
//
// Static checks on the C++ side to ensure the size & alignment is the same.
unsafe impl ExternType for QImage {
    type Id = type_id!("QImage");
    type Kind = cxx::kind::Trivial;
}

impl Drop for QImage {
    fn drop(&mut self) {
        ffi::qimage_drop(self);
    }
}

impl QImage {
    /// Convert raw image data to a [`QImage`].
    ///
    /// The data must be in the given `format`.
    /// See [`QImageReader::supportedImageFormats()`](https://doc.qt.io/qt-6/qimagereader.html#supportedImageFormats) for the list of supported formats.
    ///
    /// If no `format` is provided, the format will be quessed from the image header.
    pub fn from_data(data: &[u8], format: Option<&str>) -> Option<Self> {
        let image = ffi::qimage_init_from_data(data, format.unwrap_or(""));

        if !image.is_null() {
            Some(image)
        } else {
            None
        }
    }
    /// Returns a number that identifies the contents of this QImage object.
    pub fn cache_key(&self) -> i64 {
        ffi::qimage_cache_key(self)
    }
}

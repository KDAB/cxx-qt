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
        type AspectRatioMode = crate::AspectRatioMode;
    }

    /// The type of image format available in Qt.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QImageInvertMode {
        InvertRgb,
        InvertRgba,
    }

    /// The type of image format available in Qt.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QImageFormat {
        Format_Invalid,
        Format_Mono,
        Format_MonoLSB,
        Format_Indexed8,
        Format_RGB32,
        Format_ARGB32,
        Format_ARGB32_Premultiplied,
        Format_RGB16,
        Format_ARGB8565_Premultiplied,
        Format_RGB666,
        Format_ARGB6666_Premultiplied,
        Format_RGB555,
        Format_ARGB8555_Premultiplied,
        Format_RGB888,
        Format_RGB444,
        Format_ARGB4444_Premultiplied,
        Format_RGBX8888,
        Format_RGBA8888,
        Format_RGBA8888_Premultiplied,
        Format_BGR30,
        Format_A2BGR30_Premultiplied,
        Format_RGB30,
        Format_A2RGB30_Premultiplied,
        Format_Alpha8,
        Format_Grayscale8,
        Format_RGBX64,
        Format_RGBA64,
        Format_RGBA64_Premultiplied,
        Format_Grayscale16,
        Format_BGR888,
        /* Qt 6.2
        Format_RGBX16FPx4,
        Format_RGBA16FPx4,
        Format_RGBA16FPx4_Premultiplied,
        Format_RGBX32FPx4,
        Format_RGBA32FPx4,
        Format_RGBA32FPx4_Premultiplied,
        */
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
        include!("cxx-qt-lib/qsizef.h");
        #[allow(dead_code)]
        type QSizeF = crate::QSizeF;

        /// Returns true if all the colors in the image are shades of gray
        #[rust_name = "all_gray"]
        fn allGray(self: &QImage) -> bool;

        /// Returns the number of bit planes in the image.
        #[rust_name = "bit_plane_count"]
        fn bitPlaneCount(self: &QImage) -> i32;

        /// Returns a sub-area of the image as a new image.
        fn copy(self: &QImage, rect: &QRect) -> QImage;

        /// Creates and returns a 1-bpp heuristic mask for this image.
        #[rust_name = "create_heuristic_mask"]
        fn createHeuristicMask(self: &QImage, clipTight: bool) -> QImage;

        /// Returns the size of the color table for the image.
        #[rust_name = "color_count"]
        fn colorCount(self: &QImage) -> i32;

        /// Returns the depth of the image.
        fn depth(self: &QImage) -> i32;

        /// Returns the size of the image in device independent pixels.
        /// This value should be used when using the image size in user interface size calculations.
        /// The return value is equivalent to image.size() / image.devicePixelRatio().
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_2))]
        #[rust_name = "device_independent_size"]
        fn deviceIndependentSize(self: &QImage) -> QSizeF;

        /// Returns the number of pixels that fit horizontally in a physical meter.
        #[rust_name = "dots_per_meter_x"]
        fn dotsPerMeterX(self: &QImage) -> i32;

        /// Returns the number of pixels that fit vertically in a physical meter.
        #[rust_name = "dots_per_meter_y"]
        fn dotsPerMeterY(self: &QImage) -> i32;

        /// Fills the entire image with the given color.
        fn fill(self: &mut QImage, color: &QColor);

        /// Returns the format of the image.
        fn format(self: &QImage) -> QImageFormat;

        /// Inverts all pixel values in the image.
        #[rust_name = "invert_pixels"]
        fn invertPixels(self: &mut QImage, mode: QImageInvertMode);

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

        /// Mirrors of the image in the horizontal and/or the vertical direction depending on whether horizontal and vertical are set to true or false.
        #[cfg(cxxqt_qt_version_at_least_6)]
        fn mirror(self: &mut QImage, horizontal: bool, vertical: bool);

        /// Swaps the values of the red and blue components of all pixels, effectively converting an RGB image to an BGR image.
        #[cfg(cxxqt_qt_version_at_least_6)]
        #[rust_name = "rgb_swap"]
        fn rgbSwap(self: &mut QImage);

        /// Returns the enclosing rectangle (0, 0, width(), height()) of the image.
        fn rect(self: &QImage) -> QRect;

        /// Returns a copy of the image scaled to a rectangle with the given width and height according to the given aspectRatioMode and transformMode.
        fn scaled(
            self: &QImage,
            width: i32,
            height: i32,
            aspectRatioMode: AspectRatioMode,
            transformMode: TransformationMode,
        ) -> QImage;

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

        /// Sets the number of pixels that fit horizontally in a physical meter, to x.
        #[rust_name = "set_dots_per_meter_x"]
        fn setDotsPerMeterX(self: &mut QImage, x: i32);

        /// Sets the number of pixels that fit vertically in a physical meter, to y.
        #[rust_name = "set_dots_per_meter_y"]
        fn setDotsPerMeterY(self: &mut QImage, y: i32);

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

        /// Changes the format of the image to format without changing the data. Only works between formats of the same depth.
        #[rust_name = "reinterpret_as_format"]
        fn reinterpretAsFormat(self: &mut QImage, format: QImageFormat) -> bool;

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
        type QImageFormat;
        type QImageInvertMode;

        #[doc(hidden)]
        #[rust_name = "qimage_init_default"]
        fn construct() -> QImage;

        #[doc(hidden)]
        #[rust_name = "qimage_init_from_width_and_height_and_image_format"]
        fn construct(width: i32, height: i32, format: QImageFormat) -> QImage;

        #[doc(hidden)]
        #[rust_name = "qimage_drop"]
        fn drop(image: &mut QImage);

        #[doc(hidden)]
        #[rust_name = "qimage_init_from_data"]
        fn qimageInitFromData(data: &[u8], format: &str) -> QImage;

        #[doc(hidden)]
        #[rust_name = "qimage_cache_key"]
        fn qimageCacheKey(image: &QImage) -> i64;

        #[doc(hidden)]
        #[rust_name = "qimage_eq"]
        fn operatorEq(a: &QImage, b: &QImage) -> bool;
    }
}

pub use ffi::{QImageFormat, QImageInvertMode};

/// This struct is the Rust representation of the [`QImage`](https://doc.qt.io/qt-6/qimage.html)
/// class.
///
/// It provides a way to store and manipulate images in a hardware-independent manner.
#[repr(C)]
pub struct QImage {
    // Static checks on the C++ side ensure this is true.
    // See qcolor.cpp
    #[cfg(cxxqt_qt_version_major = "5")]
    _data: MaybeUninit<[usize; 4]>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _data: MaybeUninit<[usize; 3]>,
}

impl Clone for QImage {
    /// Constructs a copy of other.
    fn clone(&self) -> Self {
        self.copy(&self.rect())
    }
}

impl Default for QImage {
    /// Constructs a null image.
    fn default() -> Self {
        ffi::qimage_init_default()
    }
}

impl PartialEq for QImage {
    fn eq(&self, other: &Self) -> bool {
        ffi::qimage_eq(self, other)
    }
}

impl Eq for QImage {}

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

    /// Construct a Rust QImage from a given width, height, and QImage Format
    pub fn from_width_height_and_format(
        width: i32,
        height: i32,
        format: ffi::QImageFormat,
    ) -> Self {
        ffi::qimage_init_from_width_and_height_and_image_format(width, height, format)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_default_values() {
        let default_image = QImage::default();
        assert!(default_image.all_gray());
        assert!(default_image.is_null());
        assert_eq!(default_image.width(), 0);
        assert_eq!(default_image.height(), 0);
        assert_eq!(default_image.depth(), 0);
        assert!(default_image.size().is_null());
    }

    #[test]
    fn test_create_qimage_from_format() {
        let qimage = QImage::from_width_height_and_format(50, 70, ffi::QImageFormat::Format_Mono);
        assert_eq!(qimage.width(), 50);
        assert_eq!(qimage.height(), 70);
        assert!(!qimage.is_null());
        assert_eq!(qimage.format(), ffi::QImageFormat::Format_Mono);
    }

    #[test]
    fn test_copy() {
        let qimage = QImage::from_width_height_and_format(50, 70, ffi::QImageFormat::Format_Mono);
        let qimage2 = qimage.copy(&qimage.rect());
        assert_eq!(qimage.width(), qimage2.width());
        assert_eq!(qimage.height(), qimage2.height());
        assert_eq!(qimage.format(), qimage2.format());
    }
}

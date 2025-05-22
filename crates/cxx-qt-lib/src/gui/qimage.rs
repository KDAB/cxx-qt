// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use cxx::{type_id, ExternType};
use std::fmt;
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
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
        include!("cxx-qt-lib/qrect.h");
        type QRect = crate::QRect;
        include!("cxx-qt-lib/qcolor.h");
        type QColor = crate::QColor;
        include!("cxx-qt-lib/qpoint.h");
        type QPoint = crate::QPoint;
        include!("cxx-qt-lib/qsizef.h");
        #[allow(dead_code)]
        type QSizeF = crate::QSizeF;
        type QImageCleanupFunction = super::QImageCleanupFunction;
        type uchar;

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
        type c_void = crate::c_void;

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

        #[doc(hidden)]
        #[rust_name = "qimage_to_debug_qstring"]
        fn toDebugQString(image: &QImage) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    extern "C++" {
        #[doc(hidden)]
        #[rust_name = "qimage_init_from_raw_parts_mut"]
        unsafe fn construct(
            data: *mut uchar,
            width: i32,
            height: i32,
            format: QImageFormat,
            cleanup_function: QImageCleanupFunction,
            cleanup_info: *mut c_void,
        ) -> QImage;

        #[doc(hidden)]
        #[rust_name = "qimage_init_from_raw_parts"]
        unsafe fn construct(
            data: *const uchar,
            width: i32,
            height: i32,
            format: QImageFormat,
            cleanup_function: QImageCleanupFunction,
            cleanup_info: *mut c_void,
        ) -> QImage;
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
    _painters: MaybeUninit<u16>,
    #[cfg(cxxqt_qt_version_major = "5")]
    _pointers: MaybeUninit<[usize; 3]>,
    #[cfg(cxxqt_qt_version_major = "6")]
    _pointers: MaybeUninit<[usize; 2]>,
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

impl fmt::Debug for QImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ffi::qimage_to_debug_qstring(self).fmt(f)
    }
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

// Static assertions on the C++ side assert that this type is equal to:
// void(*)(void*)
#[repr(transparent)]
struct QImageCleanupFunction(extern "C" fn(*mut ffi::c_void));

unsafe impl ExternType for QImageCleanupFunction {
    type Id = type_id!("QImageCleanupFunction");

    type Kind = cxx::kind::Trivial;
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

    /// Convert raw image data to a [`QImage`].
    ///
    /// If no `format` is provided, the format will be quessed from the image header,
    /// and if the format still cannot be guessed, the invalid QImage will be returned
    pub fn from_data_or_default(data: &[u8], format: Option<&str>) -> Self {
        ffi::qimage_init_from_data(data, format.unwrap_or(""))
    }

    /// Constructs a QImage from an existing memory buffer.
    ///
    /// # Safety
    /// For details on safety see the [Qt documentation](https://doc.qt.io/qt-6/qimage.html#QImage-7)
    pub unsafe fn from_raw_parts(
        data: *const ffi::uchar,
        width: i32,
        height: i32,
        format: QImageFormat,
        cleanup_function: extern "C" fn(x: *mut ffi::c_void),
        cleanup_info: *mut ffi::c_void,
    ) -> Self {
        ffi::qimage_init_from_raw_parts(
            data,
            width,
            height,
            format,
            QImageCleanupFunction(cleanup_function),
            cleanup_info,
        )
    }

    /// Constructs a QImage from an existing mutable memory buffer.
    ///
    /// # Safety
    /// For details on safety see the [Qt documentation](https://doc.qt.io/qt-6/qimage.html#QImage-8)
    pub unsafe fn from_raw_parts_mut(
        data: *mut ffi::uchar,
        width: i32,
        height: i32,
        format: QImageFormat,
        cleanup_function: extern "C" fn(x: *mut ffi::c_void),
        cleanup_info: *mut ffi::c_void,
    ) -> Self {
        ffi::qimage_init_from_raw_parts_mut(
            data,
            width,
            height,
            format,
            QImageCleanupFunction(cleanup_function),
            cleanup_info,
        )
    }

    /// Constructs a QImage from the raw data inside a `Vec<u8>`.
    ///
    /// # Safety
    /// This function is unsafe because it requires that the data matches the given QImageFormat,
    /// width and height.
    ///
    /// It is a convenience function around [Self::from_raw_parts_mut].
    pub unsafe fn from_raw_bytes(
        data: Vec<u8>,
        width: i32,
        height: i32,
        format: QImageFormat,
    ) -> Self {
        extern "C" fn delete_boxed_vec(boxed_vec: *mut ffi::c_void) {
            // Safety: This is safe to do, as we only call this function from the destructor of a QImage
            // that is created by the call below.
            // In this case the *mut ffi::c_void is actually a `*mut Vec<u8>` that was created by
            // Box::into_raw(), so can be re-created by Box::from_raw().
            // QImage also guarantees that this is only called once when the last copy is destroyed.
            let the_box: Box<Vec<u8>> = unsafe { Box::from_raw(boxed_vec as *mut Vec<u8>) };
            drop(the_box);
        }
        let data = Box::new(data);
        let bytes = data.as_ptr() as *mut ffi::uchar;
        let raw_box = Box::into_raw(data) as *mut ffi::c_void;
        QImage::from_raw_parts_mut(bytes, width, height, format, delete_boxed_vec, raw_box)
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

#[cfg(any(feature = "image-v0-24", feature = "image-v0-25"))]
macro_rules! from_image {
    ($crt:ident) => {
        impl From<$crt::RgbaImage> for QImage {
            fn from(image: $crt::RgbaImage) -> QImage {
                let width = image.width() as i32;
                let height = image.height() as i32;
                // SAFETY: The RgbaImage has the same format as RGBA8888 and the number of
                // pixels is correct for the images width and height, which is guaranteed by
                // RgbaImage.
                unsafe {
                    QImage::from_raw_bytes(
                        image.into_raw(),
                        width,
                        height,
                        QImageFormat::Format_RGBA8888,
                    )
                }
            }
        }
    };
}

#[cfg(feature = "image-v0-24")]
from_image!(image_v0_24);

#[cfg(feature = "image-v0-25")]
from_image!(image_v0_25);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QColor;

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

    #[test]
    fn test_from_raw_bytes() {
        let bytes: Vec<u8> = vec![
            0x01, 0x02, 0x03, 0x04, // Pixel 1
            0x05, 0x06, 0x07, 0x08, // Pixel 2
        ];

        let qimage = unsafe { QImage::from_raw_bytes(bytes, 2, 1, QImageFormat::Format_RGBA8888) };
        assert_eq!(qimage.width(), 2);
        assert_eq!(qimage.height(), 1);
        assert_eq!(
            qimage.pixel_color(0, 0),
            QColor::from_rgba(0x01, 0x02, 0x03, 0x04),
        );
        assert_eq!(
            qimage.pixel_color(1, 0),
            QColor::from_rgba(0x05, 0x06, 0x07, 0x08),
        );
    }

    #[cfg(any(feature = "image-v0-24", feature = "image-v0-25"))]
    macro_rules! test_image_crate {
        ($crt:ident: $test_name:ident) => {
            #[test]
            fn $test_name() {
                use $crt::{Rgba, RgbaImage};

                // Create a sample RgbaImage
                let width = 2;
                let height = 2;
                let mut rgba_image = RgbaImage::new(width, height);
                rgba_image.put_pixel(0, 0, Rgba([255, 0, 0, 255])); // Red pixel
                rgba_image.put_pixel(1, 0, Rgba([0, 255, 0, 255])); // Green pixel
                rgba_image.put_pixel(0, 1, Rgba([0, 0, 255, 255])); // Blue pixel
                rgba_image.put_pixel(1, 1, Rgba([255, 255, 0, 255])); // Yellow pixel

                // Convert RgbaImage to QImage
                let qimage: QImage = rgba_image.into();

                // Verify the conversion
                assert_eq!(qimage.width(), width as i32);
                assert_eq!(qimage.height(), height as i32);

                // Verify the pixel data
                assert_eq!(qimage.pixel_color(0, 0), QColor::from_rgba(255, 0, 0, 255)); // Red pixel
                assert_eq!(qimage.pixel_color(1, 0), QColor::from_rgba(0, 255, 0, 255)); // Green pixel
                assert_eq!(qimage.pixel_color(0, 1), QColor::from_rgba(0, 0, 255, 255)); // Blue pixel
                assert_eq!(
                    qimage.pixel_color(1, 1),
                    QColor::from_rgba(255, 255, 0, 255)
                ); // Yellow pixel
            }
        };
    }

    #[cfg(feature = "image-v0-24")]
    test_image_crate!(image_v0_24: test_image_v0_24_to_qimage);

    #[cfg(feature = "image-v0-25")]
    test_image_crate!(image_v0_25: test_image_v0_25_to_qimage);
}

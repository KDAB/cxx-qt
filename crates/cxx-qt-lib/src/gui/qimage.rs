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
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_9))]
        type Orientations = crate::Orientations;
    }

    /// This enum type is used to describe how pixel values should be inverted in the [`QImage::invert_pixels`] function.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QImageInvertMode {
        /// Invert only the RGB values and leave the alpha channel unchanged.
        InvertRgb,
        /// Invert all channels, including the alpha channel.
        InvertRgba,
    }

    /// The type of image format available in Qt.
    #[repr(i32)]
    #[namespace = "rust::cxxqtlib1"]
    #[derive(Debug)]
    enum QImageFormat {
        /// The image is invalid.
        Format_Invalid,
        /// The image is stored using 1-bit per pixel. Bytes are packed with the most significant bit (MSB) first.
        Format_Mono,
        /// The image is stored using 1-bit per pixel. Bytes are packed with the less significant bit (LSB) first.
        Format_MonoLSB,
        /// The image is stored using 8-bit indexes into a colormap.
        Format_Indexed8,
        /// The image is stored using a 32-bit RGB format (0xffRRGGBB).
        Format_RGB32,
        /// The image is stored using a 32-bit ARGB format (0xAARRGGBB).
        Format_ARGB32,
        /// The image is stored using a premultiplied 32-bit ARGB format (0xAARRGGBB), i.e. the red, green, and blue channels are multiplied by the alpha component divided by 255. (If RR, GG, or BB has a higher value than the alpha channel, the results are undefined.) Certain operations (such as image composition using alpha blending) are faster using premultiplied ARGB32 than with plain ARGB32.
        Format_ARGB32_Premultiplied,
        /// The image is stored using a 16-bit RGB format (5-6-5).
        Format_RGB16,
        /// The image is stored using a premultiplied 24-bit ARGB format (8-5-6-5).
        Format_ARGB8565_Premultiplied,
        /// The image is stored using a 24-bit RGB format (6-6-6). The unused most significant bits is always zero.
        Format_RGB666,
        /// The image is stored using a premultiplied 24-bit ARGB format (6-6-6-6).
        Format_ARGB6666_Premultiplied,
        /// The image is stored using a 16-bit RGB format (5-5-5). The unused most significant bit is always zero.
        Format_RGB555,
        /// The image is stored using a premultiplied 24-bit ARGB format (8-5-5-5).
        Format_ARGB8555_Premultiplied,
        /// The image is stored using a 24-bit RGB format (8-8-8).
        Format_RGB888,
        /// The image is stored using a 16-bit RGB format (4-4-4). The unused bits are always zero.
        Format_RGB444,
        /// The image is stored using a premultiplied 16-bit ARGB format (4-4-4-4).
        Format_ARGB4444_Premultiplied,
        /// The image is stored using a 32-bit byte-ordered RGB(x) format (8-8-8-8). This is the same as the Format_RGBA8888 except alpha must always be 255.
        Format_RGBX8888,
        /// The image is stored using a 32-bit byte-ordered RGBA format (8-8-8-8). Unlike ARGB32 this is a byte-ordered format, which means the 32bit encoding differs between big endian and little endian architectures, being respectively (0xRRGGBBAA) and (0xAABBGGRR). The order of the colors is the same on any architecture if read as bytes 0xRR,0xGG,0xBB,0xAA.
        Format_RGBA8888,
        /// The image is stored using a premultiplied 32-bit byte-ordered RGBA format (8-8-8-8).
        Format_RGBA8888_Premultiplied,
        /// The image is stored using a 32-bit BGR format (x-10-10-10).
        Format_BGR30,
        /// The image is stored using a 32-bit premultiplied ABGR format (2-10-10-10).
        Format_A2BGR30_Premultiplied,
        /// The image is stored using a 32-bit RGB format (x-10-10-10).
        Format_RGB30,
        /// The image is stored using a 32-bit premultiplied ARGB format (2-10-10-10).
        Format_A2RGB30_Premultiplied,
        /// The image is stored using an 8-bit alpha only format.
        Format_Alpha8,
        /// The image is stored using an 8-bit grayscale format.
        Format_Grayscale8,
        /// The image is stored using a 64-bit halfword-ordered RGB(x) format (16-16-16-16). This is the same as the Format_RGBA64 except alpha must always be 65535.
        Format_RGBX64,
        /// The image is stored using a 64-bit halfword-ordered RGBA format (16-16-16-16).
        Format_RGBA64,
        /// The image is stored using a premultiplied 64-bit halfword-ordered RGBA format (16-16-16-16).
        Format_RGBA64_Premultiplied,
        /// The image is stored using an 16-bit grayscale format.
        Format_Grayscale16,
        /// The image is stored using a 24-bit BGR format.
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

        /// Returns `true` if all the colors in the image are shades of gray (i.e. their red, green and blue components are equal); otherwise `false`.
        ///
        /// Note that this function is slow for images without color table.
        #[rust_name = "all_gray"]
        fn allGray(self: &QImage) -> bool;

        /// Returns the number of bit planes in the image.
        ///
        /// The number of bit planes is the number of bits of color and transparency information for each pixel. This is different from (i.e. smaller than) the depth when the image format contains unused bits.
        #[rust_name = "bit_plane_count"]
        fn bitPlaneCount(self: &QImage) -> i32;

        /// Returns a sub-area of the image as a new image.
        ///
        /// The returned image is copied from the position (`rectangle.x()`, `rectangle.y()`) in this image, and will always have the size of the given `rectangle`.
        ///
        /// In areas beyond this image, pixels are set to 0. For 32-bit RGB images, this means black; for 32-bit ARGB images, this means transparent black; for 8-bit images, this means the color with index 0 in the color table which can be anything; for 1-bit images, this means Qt::color0.
        ///
        /// If the given `rectangle` is a null rectangle the entire image is copied.
        fn copy(self: &QImage, rectangle: &QRect) -> QImage;

        /// Creates and returns a 1-bpp heuristic mask for this image.
        ///
        /// The function works by selecting a color from one of the corners, then chipping away pixels of that color starting at all the edges. The four corners vote for which color is to be masked away. In case of a draw (this generally means that this function is not applicable to the image), the result is arbitrary.
        ///
        /// The returned image has little-endian bit order (i.e. the image's format is [`QImageFormat::Format_MonoLSB`]), which you can convert to big-endian ([`QImageFormat::Format_Mono`]) using the [convertToFormat](https://doc.qt.io/qt/qimage.html#convertToFormat)() function.
        ///
        /// If `clip_tight` is `true` the mask is just large enough to cover the pixels; otherwise, the mask is larger than the data pixels.
        ///
        /// Note that this function disregards the alpha buffer.
        #[rust_name = "create_heuristic_mask"]
        fn createHeuristicMask(self: &QImage, clip_tight: bool) -> QImage;

        /// Returns the size of the color table for the image.
        ///
        /// Notice that this function returns 0 for 32-bpp images because these images do not use color tables, but instead encode pixel values as ARGB quadruplets.
        #[rust_name = "color_count"]
        fn colorCount(self: &QImage) -> i32;

        /// Returns the depth of the image.
        ///
        /// The image depth is the number of bits used to store a single pixel, also called bits per pixel (bpp).
        ///
        /// The supported depths are 1, 8, 16, 24, 32 and 64.
        fn depth(self: &QImage) -> i32;

        /// Returns the size of the image in device independent pixels.
        /// This value should be used when using the image size in user interface size calculations.
        /// The return value is equivalent to [`size`](Self::size) / [devicePixelRatio](https://doc.qt.io/qt/qimage.html#devicePixelRatio)().
        ///
        /// This function was introduced in Qt 6.2.
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_2))]
        #[rust_name = "device_independent_size"]
        fn deviceIndependentSize(self: &QImage) -> QSizeF;

        /// Returns the number of pixels that fit horizontally in a physical meter. Together with [`dots_per_meter_y`](Self::dots_per_meter_y), this number defines the intended scale and aspect ratio of the image.
        #[rust_name = "dots_per_meter_x"]
        fn dotsPerMeterX(self: &QImage) -> i32;

        /// Returns the number of pixels that fit vertically in a physical meter. Together with [`dots_per_meter_x`](Self::dots_per_meter_x), this number defines the intended scale and aspect ratio of the image.
        #[rust_name = "dots_per_meter_y"]
        fn dotsPerMeterY(self: &QImage) -> i32;

        /// Fills the entire image with the given `color`.
        ///
        /// If the depth of this image is 1, only the lowest bit is used. If you say fill(0), fill(2), etc., the image is filled with 0s. If you say fill(1), fill(3), etc., the image is filled with 1s. If the depth is 8, the lowest 8 bits are used and if the depth is 16 the lowest 16 bits are used.
        ///
        /// If the image depth is higher than 32bit the result is undefined.
        fn fill(self: &mut QImage, color: &QColor);

        /// Flips or mirrors the image in the horizontal and/or the vertical direction depending on orient.
        ///
        /// This function was introduced in Qt 6.9.
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_9))]
        fn flip(self: &mut QImage, orient: Orientations);

        /// Returns the format of the image.
        fn format(self: &QImage) -> QImageFormat;

        /// Inverts all pixel values in the image.
        ///
        /// The given invert mode only have a meaning when the image's depth is 32. If the mode is [`QImageInvertMode::InvertRgb`], the alpha channel is left unchanged. If the mode is [`QImageInvertMode::InvertRgba`], the alpha bits are also inverted.
        ///
        /// Inverting an 8-bit image means to replace all pixels using color index `i` with a pixel using color index 255 minus `i`. The same is the case for a 1-bit image. Note that the color table is `not` changed.
        ///
        /// If the image has a premultiplied alpha channel, the image is first converted to an unpremultiplied image format to be inverted and then converted back.
        #[rust_name = "invert_pixels"]
        fn invertPixels(self: &mut QImage, mode: QImageInvertMode);

        /// Returns `true` if it is a null image, otherwise returns `false`.
        ///
        /// A null image has all parameters set to zero and no allocated data.
        #[rust_name = "is_null"]
        fn isNull(self: &QImage) -> bool;

        /// For 32-bit images, this function is equivalent to [`all_gray`](Self::all_gray).
        /// For color indexed images, this function returns `true` if color(i) is QRgb(i, i, i)
        /// for all indexes of the color table; otherwise returns `false`.
        #[rust_name = "is_gray_scale"]
        fn isGrayscale(self: &QImage) -> bool;

        /// Returns `true` if the image has a format that respects the alpha channel, otherwise returns `false`.
        #[rust_name = "has_alpha_channel"]
        fn hasAlphaChannel(self: &QImage) -> bool;

        /// Returns the height of the image.
        fn height(self: &QImage) -> i32;

        /// Mirrors of the image in the horizontal and/or the vertical direction depending on whether `horizontal` and `vertical` are set to `true` or `false`.
        ///
        /// This function was introduced in Qt 6.0.
        /// This function is scheduled for deprecation in version 6.13.
        #[cfg(all(
            cxxqt_qt_version_at_least_6,
            not(cxxqt_qt_version_at_least_6_10),
            not(cxxqt_qt_version_at_least_7)
        ))]
        fn mirror(self: &mut QImage, horizontal: bool, vertical: bool);

        /// Swaps the values of the red and blue components of all pixels, effectively converting an RGB image to an BGR image.
        ///
        /// This function was introduced in Qt 6.0.
        #[cfg(cxxqt_qt_version_at_least_6)]
        #[rust_name = "rgb_swap"]
        fn rgbSwap(self: &mut QImage);

        /// Returns the enclosing rectangle (`0`, `0`, `width()`, `height()`) of the image.
        fn rect(self: &QImage) -> QRect;

        /// Returns a copy of the image scaled to a rectangle with the given `width` and `height` according to the given `aspect_ratio_mode` and `transform_mode`.
        fn scaled(
            self: &QImage,
            width: i32,
            height: i32,
            aspect_ratio_mode: AspectRatioMode,
            transform_mode: TransformationMode,
        ) -> QImage;

        /// Returns a scaled copy of the image. The returned image is scaled to the given `height` using the specified transformation `mode`.
        ///
        /// This function automatically calculates the width of the image so that the ratio of the image is preserved.
        ///
        /// If the given `height` is 0 or negative, a null image is returned.
        #[rust_name = "scaled_to_height"]
        fn scaledToHeight(self: &QImage, height: i32, mode: TransformationMode) -> QImage;

        /// Returns a scaled copy of the image. The returned image is scaled to the given `width` using the specified transformation `mode`.
        ///
        /// This function automatically calculates the height of the image so that the ratio of the image is preserved.
        ///
        /// If the given `width` is 0 or negative, a null image is returned.
        #[rust_name = "scaled_to_width"]
        fn scaledToWidth(self: &QImage, width: i32, mode: TransformationMode) -> QImage;

        /// Resizes the color table to contain `color_count` entries.
        ///
        /// If the color table is expanded, all the extra colors will be set to transparent.
        ///
        /// When the image is used, the color table must be large enough to have entries for all the pixel/index values present in the image, otherwise the results are undefined.
        #[rust_name = "set_color_count"]
        fn setColorCount(self: &mut QImage, color_count: i32);

        /// Sets the alpha channel of this image to the given `alpha_channel`.
        ///
        /// If `alpha_channel` is an 8 bit alpha image, the alpha values are used directly. Otherwise, `alpha_channel` is converted to 8 bit grayscale and the intensity of the pixel values is used.
        ///
        /// If the image already has an alpha channel, the existing alpha channel is multiplied with the new one. If the image doesn't have an alpha channel it will be converted to a format that does.
        #[rust_name = "set_alpha_channel"]
        fn setAlphaChannel(self: &mut QImage, alpha_channel: &QImage);

        /// Sets the number of pixels that fit horizontally in a physical meter, to `x`.
        #[rust_name = "set_dots_per_meter_x"]
        fn setDotsPerMeterX(self: &mut QImage, x: i32);

        /// Sets the number of pixels that fit vertically in a physical meter, to `y`.
        #[rust_name = "set_dots_per_meter_y"]
        fn setDotsPerMeterY(self: &mut QImage, y: i32);

        /// Sets the number of pixels by which the image is intended to be offset by when positioning relative to other images, to `offset`.
        #[rust_name = "set_offset"]
        fn setOffset(self: &mut QImage, offset: &QPoint);

        /// Sets the pixel color at (`x`, `y`) to `color`.
        ///
        /// If (`x`, `y`) is not a valid coordinate pair in the image, or the image's format is either monochrome or paletted, the result is undefined.
        #[rust_name = "set_pixel_color"]
        fn setPixelColor(self: &mut QImage, x: i32, y: i32, color: &QColor);

        /// Returns the size of the image.
        fn size(self: &QImage) -> QSize;

        /// Swaps image `other` with this image. This operation is very fast and never fails.
        fn swap(self: &mut QImage, other: &mut QImage);

        /// Changes the format of the image to `format` without changing the data. Only works between formats of the same depth.
        ///
        /// Returns `true` if successful.
        ///
        /// This function can be used to change images with alpha-channels to their corresponding opaque formats if the data is known to be opaque-only, or to change the format of a given image buffer before overwriting it with new data.
        ///
        /// **Warning:** The function does not check if the image data is valid in the new format and will still return true if the depths are compatible. Operations on an image with invalid data are undefined.
        ///
        /// **Warning:** If the image is not detached, this will cause the data to be copied.
        #[rust_name = "reinterpret_as_format"]
        fn reinterpretAsFormat(self: &mut QImage, format: QImageFormat) -> bool;

        /// Returns the number of pixels by which the image is intended to be offset by when positioning relative to other images.
        fn offset(self: &QImage) -> QPoint;

        /// Returns the color of the pixel at coordinates (`x`, `y`) as a `QColor`.
        ///
        /// If the position (`x`, `y`) is not valid, an invalid `QColor` is returned.
        #[rust_name = "pixel_color"]
        fn pixelColor(self: &QImage, x: i32, y: i32) -> QColor;

        /// Returns the pixel index at (`x`, `y`).
        ///
        /// If the position (`x`, `y`) is not valid, or if the image is not a paletted image ([`depth`](Self::depth) > 8), the results are undefined.
        #[rust_name = "pixel_index"]
        fn pixelIndex(self: &QImage, x: i32, y: i32) -> i32;

        /// Returns `true` if (`x`, `y`) is a valid coordinate pair within the image; otherwise returns `false`.
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

/// The `QImage` class provides a hardware-independent image representation that allows direct access to the pixel data, and can be used as a paint device.
///
/// Qt Documentation: [QImage](https://doc.qt.io/qt/qimage.html#details)
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
    /// Convert raw image data to a `QImage`.
    ///
    /// The data must be in the given `format`.
    /// See [QImageReader::supportedImageFormats](https://doc.qt.io/qt/qimagereader.html#supportedImageFormats)() for the list of supported formats.
    ///
    /// If `format` is `None`, the format will be quessed from the image header.
    pub fn from_data(data: &[u8], format: Option<&str>) -> Option<Self> {
        let image = ffi::qimage_init_from_data(data, format.unwrap_or(""));
        if image.is_null() {
            None
        } else {
            Some(image)
        }
    }

    /// Constructs a `QImage` from an existing memory buffer.
    ///
    /// # Safety
    /// For details on safety see the [Qt documentation](https://doc.qt.io/qt/qimage.html#QImage-7)
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

    /// Constructs a `QImage` from an existing mutable memory buffer.
    ///
    /// # Safety
    /// For details on safety see the [Qt documentation](https://doc.qt.io/qt/qimage.html#QImage-8)
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

    /// Constructs a `QImage` from the raw data inside a `Vec<u8>`.
    ///
    /// # Safety
    /// This function is unsafe because it requires that the data matches the given `QImageFormat`,
    /// `width` and `height`.
    ///
    /// It is a convenience function around [`from_raw_parts_mut`](Self::from_raw_parts_mut).
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

    /// Returns a number that identifies the contents of this `QImage` object. Distinct `QImage` objects can only have the same key if they refer to the same contents.
    ///
    /// The key will change when the image is altered.
    pub fn cache_key(&self) -> i64 {
        ffi::qimage_cache_key(self)
    }

    /// Construct a `QImage` from a given `width`, `height`, and image `format`.
    pub fn from_width_height_and_format(width: i32, height: i32, format: QImageFormat) -> Self {
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
        let qimage = QImage::from_width_height_and_format(50, 70, QImageFormat::Format_Mono);
        assert_eq!(qimage.width(), 50);
        assert_eq!(qimage.height(), 70);
        assert!(!qimage.is_null());
        assert_eq!(qimage.format(), QImageFormat::Format_Mono);
    }

    #[test]
    fn test_copy() {
        let qimage = QImage::from_width_height_and_format(50, 70, QImageFormat::Format_Mono);
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

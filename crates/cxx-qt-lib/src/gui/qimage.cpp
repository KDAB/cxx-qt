// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qimage.h"
#include <cxx-qt-lib/assertion_utils.h>
#include <string>

// A QImage inherits from QPaintDevice.

// QPaintDevice in Qt5 contains two things:
// 1. ushort painters; (due to the following field this has padding to make it
// 64-bit long)
// 2. QPaintDevicePrivate *reserved;
// Then QImage adds an additional field:
// 3. QImageData *d;
// For a total of 3 pointers in length.
// Because of the added v-table, it's a total of 4 pointers in size.
#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QImage, {
  ::std::uint16_t a0;
  ::std::size_t a1;
  ::std::size_t a2;
  ::std::size_t a3;
});
#else
// In Qt6 the QPaintDevice doesn't contain the `reserved` pointer, making it 1
// pointer smaller
assert_alignment_and_size(QImage, {
  ::std::uint16_t a0;
  ::std::size_t a1;
  ::std::size_t a2;
});
#endif

static_assert(std::is_same_v<QImageCleanupFunction, void (*)(void*)>);

namespace rust {
namespace cxxqtlib1 {

QImage
qimageInitFromData(const rust::Slice<std::uint8_t const> data, rust::Str format)
{
  std::string formatString(format);
  return QImage::fromData(static_cast<const unsigned char*>(data.data()),
                          static_cast<int>(data.size()),
                          formatString.empty() ? nullptr : formatString.data());
}

QImage
qimageInitFromRawParts(const std::uint8_t* data,
                       int width,
                       int height,
                       QImageFormat format,
                       QImageCleanupFunction cleanupFunction,
                       void* cleanupInfo)
{
  return QImage(static_cast<const unsigned char*>(data),
                width,
                height,
                format,
                cleanupFunction,
                cleanupInfo);
}

QImage
qimageInitFromRawParts(std::uint8_t* data,
                       int width,
                       int height,
                       QImageFormat format,
                       QImageCleanupFunction cleanupFunction,
                       void* cleanupInfo)
{
  return QImage(static_cast<unsigned char*>(data),
                width,
                height,
                format,
                cleanupFunction,
                cleanupInfo);
}

::std::int64_t
qimageCacheKey(const QImage& image)
{
  return static_cast<::std::int64_t>(image.cacheKey());
}
}
}

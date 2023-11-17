// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#ifdef CXX_QT_GUI_FEATURE

#include "cxx-qt-lib/qimage.h"
#include "../assertion_utils.h"
#include <string>

// A QImage inherits from QPaintDevice.

#if (QT_VERSION < QT_VERSION_CHECK(6, 0, 0))
// QPaintDevice in Qt5 contains two things:
// 1. ushort painters; (due to the following field this has padding to make it
// 64-bit long)
// 2. QPaintDevicePrivate *reserved;
// Then QImage adds an additional field:
// 3. QImageData *d;
// For a total of 3 pointers in length.
// Because of the added v-table, it's a total of 4 pointers in size.
assert_alignment_and_size(QImage,
                          alignof(::std::size_t),
                          sizeof(::std::size_t) * 4);
#else
// In Qt6 the QPaintDevice doesn't contain the `reserved` pointer, making it 1
// pointer smaller
assert_alignment_and_size(QImage,
                          alignof(::std::size_t),
                          sizeof(::std::size_t) * 3);
#endif

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

}
}

#endif

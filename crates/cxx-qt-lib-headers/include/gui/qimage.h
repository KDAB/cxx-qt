// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_GUI_FEATURE

#include <QtGui/QImage>

#include "rust/cxx.h"

#include <cstdint>

namespace rust {

// QImage has a move constructor, however it is basically trivial.
template<>
struct IsRelocatable<QImage> : ::std::true_type
{
};

namespace cxxqtlib1 {
using QImageFormat = QImage::Format;

QImage
qimageInitFromData(const rust::Slice<std::uint8_t const> data,
                   rust::Str format);

} // namespace cxxqtlib1
} // namespace rust
#endif

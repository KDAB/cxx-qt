// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QImage>

#include "rust/cxx.h"

#include <cstdint>

namespace rust {

// QImage has a move constructor, however it is basically trivial.
template<>
struct IsRelocatable<QImage> : ::std::true_type
{};

namespace cxxqtlib1 {
using QImageFormat = QImage::Format;
using QImageInvertMode = QImage::InvertMode;

QImage
qimageInitFromData(const rust::Slice<std::uint8_t const> data,
                   rust::Str format);

QImage
qimageInitFromRawParts(const std::uint8_t* data,
                       int width,
                       int height,
                       QImageFormat format,
                       QImageCleanupFunction cleanupFunction,
                       void* cleanupInfo);

QImage
qimageInitFromRawParts(std::uint8_t* data,
                       int width,
                       int height,
                       QImageFormat format,
                       QImageCleanupFunction cleanupFunction,
                       void* cleanupInfo);

::std::int64_t
qimageCacheKey(const QImage& image);

} // namespace cxxqtlib1
} // namespace rust

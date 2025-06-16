// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qpixmap.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QPixmap>
qpixmapNew(int width, int height)
{
    return ::std::make_unique<QPixmap>(width, height);
}

} // namespace cxxqtlib1
} // namespace rust
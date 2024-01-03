// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_GUI_FEATURE

#include <cinttypes>
#include <memory>

#include <QtGui/QPainter>

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QPainter>
qpainterInitDefault();

}
}
#endif

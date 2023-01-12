// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#ifdef CXX_QT_GUI_FEATURE
#include "cxx-qt-lib/qqmlengine.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QQmlEngine>
qqmlengineNew()
{
  return ::std::make_unique<QQmlEngine>();
}

}
}
#endif

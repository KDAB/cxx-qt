// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#ifdef CXX_QT_GUI_FEATURE
#include "cxx-qt-lib/qqmlapplicationengine.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QQmlApplicationEngine>
qqmlapplicationengineNew()
{
  return ::std::make_unique<QQmlApplicationEngine>();
}

}
}
#endif

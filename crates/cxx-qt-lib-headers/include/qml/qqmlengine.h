// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_QML_FEATURE

#include <memory>

#include <QtQml/QQmlEngine>

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QQmlEngine>
qqmlengineNew();

}
}

#endif

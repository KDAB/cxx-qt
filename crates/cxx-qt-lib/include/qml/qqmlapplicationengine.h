// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_QML_FEATURE

#include <memory>

#include <QtQml/QQmlApplicationEngine>
#include <QtQml/QQmlEngine>

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QQmlApplicationEngine>
qqmlapplicationengineNew();

QQmlEngine&
qqmlapplicationengineAsQQmlEngine(QQmlApplicationEngine&);

#if (QT_VERSION >= QT_VERSION_CHECK(6, 5, 0))
void*
qqmlapplicationengineSingletonInstance(QQmlApplicationEngine& engine,
                                       QAnyStringView uri,
                                       QAnyStringView typeName);
#endif

}
}

#endif

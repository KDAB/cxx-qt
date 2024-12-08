// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qqmlapplicationengine.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QQmlApplicationEngine>
qqmlapplicationengineNew()
{
  return ::std::make_unique<QQmlApplicationEngine>();
}

QQmlEngine&
qqmlapplicationengineAsQQmlEngine(QQmlApplicationEngine& engine)
{
  return static_cast<QQmlEngine&>(engine);
}

#if (QT_VERSION >= QT_VERSION_CHECK(6, 5, 0))
void*
qqmlapplicationengineSingletonInstance(QQmlApplicationEngine& engine,
                                       QAnyStringView uri,
                                       QAnyStringView typeName)
{
  return reinterpret_cast<void*>(
    engine.singletonInstance<QObject*>(uri, typeName));
}
#endif

}
}

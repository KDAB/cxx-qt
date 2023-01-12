// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Be Wilson <be.wilson@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_cargo_cpp_includes
#include <QtQml/QQmlEngine>

#include "cxx-qt-gen/my_object.cxxqt.h"

// ANCHOR_END: book_cargo_cpp_includes

// ANCHOR: book_cargo_run_cpp
void
register_types()
{
  qmlRegisterType<MyObject>("com.kdab.cxx_qt.demo", 1, 0, "MyObject");
}
// ANCHOR_END: book_cargo_run_qml

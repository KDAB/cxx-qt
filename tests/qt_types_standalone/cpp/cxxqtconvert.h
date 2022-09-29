// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtGui/QColor>
#include <QtTest/QTest>

#include "cxx-qt-lib/include/convert.h"

class CxxQtConvertTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  // This test just ensures that it compiles
  void cxx_qt_convert_compiles()
  {
    // T -> R
    QColor ret1a = rust::cxxqtlib1::cxx_qt_convert<QColor, QColor>{}(QColor());
    auto colorRef = QColor();
    QColor& ret1b =
      rust::cxxqtlib1::cxx_qt_convert<QColor&, QColor&>{}(colorRef);
    const QColor& ret1c =
      rust::cxxqtlib1::cxx_qt_convert<const QColor&, const QColor&>{}(QColor());

    // std::unique_ptr<T> -> R
    QColor ret2 =
      rust::cxxqtlib1::cxx_qt_convert<QColor, std::unique_ptr<QColor>>{}(
        std::make_unique<QColor>(QColor()));

    // const std::unique_ptr<T>& -> const R&
    const QColor& ret3 =
      rust::cxxqtlib1::cxx_qt_convert<const QColor&,
                                      const std::unique_ptr<QColor>&>{}(
        std::make_unique<QColor>(QColor()));

    // const T& -> std::unique_ptr<T>
    std::unique_ptr<QColor> ret4 =
      rust::cxxqtlib1::cxx_qt_convert<std::unique_ptr<QColor>, const QColor&>{}(
        QColor());
  }
};

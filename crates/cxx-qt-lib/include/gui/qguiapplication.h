// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <memory>

#include <QtGui/QFont>
#include <QtGui/QGuiApplication>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QGuiApplication>
qguiapplicationNew(const QVector<QByteArray>& args);

inline void (*qguiapplicationSetFont)(const QFont&) = QGuiApplication::setFont;

inline QFont (*qguiapplicationFont)() = QGuiApplication::font;

inline void (*qguiapplicationSetDesktopFileName)(const QString&) =
  QGuiApplication::setDesktopFileName;

inline QString (*qguiapplicationDesktopFileName)() =
  QGuiApplication::desktopFileName;

inline Qt::KeyboardModifiers (*qguiapplicationKeyboardModifiers)() =
  QGuiApplication::keyboardModifiers;

inline Qt::MouseButtons (*qguiapplicationMouseButtons)() =
  QGuiApplication::mouseButtons;

inline Qt::KeyboardModifiers (*qguiapplicationQueryKeyboardModifiers)() =
  QGuiApplication::queryKeyboardModifiers;

}
}

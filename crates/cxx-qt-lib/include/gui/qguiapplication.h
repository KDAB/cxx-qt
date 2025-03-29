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

void (*qguiapplicationSetFont)(const QFont&) = QGuiApplication::setFont;

QFont (*qguiapplicationFont)() = QGuiApplication::font;

void (*qguiapplicationSetDesktopFileName)(const QString&) =
  QGuiApplication::setDesktopFileName;

QString (*qguiapplicationDesktopFileName)() = QGuiApplication::desktopFileName;

Qt::KeyboardModifiers (*qguiapplicationKeyboardModifiers)() =
  QGuiApplication::keyboardModifiers;

Qt::MouseButtons (*qguiapplicationMouseButtons)() =
  QGuiApplication::mouseButtons;

Qt::KeyboardModifiers (*qguiapplicationQueryKeyboardModifiers)() =
  QGuiApplication::queryKeyboardModifiers;

}
}

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

void
qguiapplicationAddLibraryPath(const QString& path);

QString
qguiapplicationApplicationName();

QString
qguiapplicationApplicationVersion();

::std::int32_t
qguiapplicationExec();

QStringList
qguiapplicationLibraryPaths();

QString
qguiapplicationOrganizationDomain();

QString
qguiapplicationOrganizationName();

void
qguiapplicationSetApplicationName(const QString& name);

void
qguiapplicationSetApplicationVersion(const QString& version);

void
qguiapplicationSetLibraryPaths(const QStringList& paths);

void
qguiapplicationSetOrganizationDomain(const QString& domain);

void
qguiapplicationSetOrganizationName(const QString& name);

void
qguiapplicationRemoveLibraryPath(const QString& path);

void
qguiapplicationSetFont(const QFont& font);

QFont
qguiapplicationFont();

void
qguiapplicationSetDesktopFileName(const QString& name);

QString
qguiapplicationDesktopFileName();

Qt::KeyboardModifiers
qguiapplicationKeyboardModifiers();

Qt::MouseButtons
qguiapplicationMouseButtons();

Qt::KeyboardModifiers
qguiapplicationQueryKeyboardModifiers();

}
}

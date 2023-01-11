// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#ifdef CXX_QT_GUI_FEATURE
#include <cstdint>
#include <memory>

#include <QtCore/QByteArray>
#include <QtCore/QString>
#include <QtCore/QStringList>
#include <QtCore/QVector>
#include <QtGui/QGuiApplication>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QGuiApplication>
qguiapplicationNew(const QVector<QByteArray>& args);

void
qguiapplicationAddLibraryPath(QGuiApplication& app, const QString& path);
QString
qguiapplicationApplicationName(const QGuiApplication& app);
QString
qguiapplicationApplicationVersion(const QGuiApplication& app);
::std::int32_t
qguiapplicationExec(QGuiApplication& app);
QStringList
qguiapplicationLibraryPaths(const QGuiApplication& app);
QString
qguiapplicationOrganizationDomain(const QGuiApplication& app);
QString
qguiapplicationOrganizationName(const QGuiApplication& app);
void
qguiapplicationSetApplicationName(QGuiApplication& app, const QString& name);
void
qguiapplicationSetApplicationVersion(QGuiApplication& app,
                                     const QString& version);
void
qguiapplicationSetLibraryPaths(QGuiApplication& app, const QStringList& paths);
void
qguiapplicationSetOrganizationDomain(QGuiApplication& app,
                                     const QString& domain);
void
qguiapplicationSetOrganizationName(QGuiApplication& app, const QString& name);

}
}

#endif

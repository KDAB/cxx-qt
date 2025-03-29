// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <memory>

#include <QtGui/QFont>
#include <QtWidgets/QApplication>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QApplication>
qapplicationNew(const QVector<QByteArray>& args);

void
qapplicationAddLibraryPath(const QString& path);

QString
qapplicationApplicationName();

QString
qapplicationApplicationVersion();

::std::int32_t
qapplicationExec();

QStringList
qapplicationLibraryPaths();

QString
qapplicationOrganizationDomain();

QString
qapplicationOrganizationName();

void
qapplicationSetApplicationName(const QString& name);

void
qapplicationSetApplicationVersion(const QString& version);

void
qapplicationSetLibraryPaths(const QStringList& paths);

void
qapplicationSetOrganizationDomain(const QString& domain);

void
qapplicationSetOrganizationName(const QString& name);

void
qapplicationRemoveLibraryPath(const QString& path);

void
qapplicationSetFont(const QFont& font);

QFont
qapplicationFont();

void
qapplicationSetDesktopFileName(const QString& name);

QString
qapplicationDesktopFileName();

}
}

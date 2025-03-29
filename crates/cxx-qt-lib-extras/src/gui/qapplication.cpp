// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib-extras/qapplication.h"

#include "cxx-qt-lib/qcoreapplication.h"

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QApplication>
qapplicationNew(const QVector<QByteArray>& args)
{
  // Ensure that our QVector has the same lifetime as the QApplication
  // by storing it inside a QObject that has QApplication as it's parent
  auto argsData = new ApplicationArgsData(args);
  // Note that QApplication uses a reference to an int for the size here
  // so we need to ensure that reference remains valid
  auto ptr =
    ::std::make_unique<QApplication>(argsData->size(), argsData->data());
  Q_ASSERT(ptr != nullptr);
  argsData->setParent(ptr.get());

  return ptr;
}

void
qapplicationAddLibraryPath(const QString& path)
{
  QApplication::addLibraryPath(path);
}

QString
qapplicationApplicationName()
{
  return QApplication::applicationName();
}

QString
qapplicationApplicationVersion()
{
  return QApplication::applicationVersion();
}

::std::int32_t
qapplicationExec()
{
  return static_cast<::std::int32_t>(QApplication::exec());
}

QStringList
qapplicationLibraryPaths()
{
  return QApplication::libraryPaths();
}

QString
qapplicationOrganizationDomain()
{
  return QApplication::organizationDomain();
}

QString
qapplicationOrganizationName()
{
  return QApplication::organizationName();
}

void
qapplicationSetApplicationName(const QString& name)
{
  QApplication::setApplicationName(name);
}

void
qapplicationSetApplicationVersion(const QString& version)
{
  QApplication::setApplicationVersion(version);
}

void
qapplicationSetLibraryPaths(const QStringList& paths)
{
  QApplication::setLibraryPaths(paths);
}

void
qapplicationSetOrganizationDomain(const QString& domain)
{
  QApplication::setOrganizationDomain(domain);
}

void
qapplicationSetOrganizationName(const QString& name)
{
  QApplication::setOrganizationName(name);
}

void
qapplicationRemoveLibraryPath(const QString& path)
{
  QApplication::removeLibraryPath(path);
}

void
qapplicationSetFont(const QFont& font)
{
  QApplication::setFont(font);
}

QFont
qapplicationFont()
{
  return QApplication::font();
}

void
qapplicationSetDesktopFileName(const QString& name)
{
  QApplication::setDesktopFileName(name);
}

QString
qapplicationDesktopFileName()
{
  return QApplication::desktopFileName();
}

}
}

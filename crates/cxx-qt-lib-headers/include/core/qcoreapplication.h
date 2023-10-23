// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <cstdint>
#include <memory>
#include <vector>

#include <QtCore/QByteArray>
#include <QtCore/QCoreApplication>
#include <QtCore/QStringList>
#include <QtCore/QVector>

namespace rust {
namespace cxxqtlib1 {

class ApplicationArgsData : public QObject
{
public:
  explicit ApplicationArgsData(const QVector<QByteArray>& args,
                               QObject* parent = nullptr);

  char** data();
  int& size();

private:
  QVector<QByteArray> m_ownedVector;
  int m_size = 0;
  std::vector<char*> m_vector;
};

::std::unique_ptr<QCoreApplication>
qcoreapplicationNew(const QVector<QByteArray>& args);

template<typename T>
void
qapplicationAddLibraryPath(T& app, const QString& path)
{
  app.addLibraryPath(path);
}

template<typename T>
QString
qapplicationApplicationName(const T& app)
{
  return app.applicationName();
}

template<typename T>
QString
qapplicationApplicationVersion(const T& app)
{
  return app.applicationVersion();
}

template<typename T>
::std::int32_t
qapplicationExec(T& app)
{
  return static_cast<::std::int32_t>(app.exec());
}

template<typename T>
QStringList
qapplicationLibraryPaths(const T& app)
{
  return app.libraryPaths();
}

template<typename T>
QString
qapplicationOrganizationDomain(const T& app)
{
  return app.organizationDomain();
}

template<typename T>
QString
qapplicationOrganizationName(const T& app)
{
  return app.organizationName();
}

template<typename T>
void
qapplicationSetApplicationName(T& app, const QString& name)
{
  app.setApplicationName(name);
}

template<typename T>
void
qapplicationSetApplicationVersion(T& app, const QString& version)
{
  app.setApplicationVersion(version);
}

template<typename T>
void
qapplicationSetLibraryPaths(T& app, const QStringList& paths)
{
  app.setLibraryPaths(paths);
}

template<typename T>
void
qapplicationSetOrganizationDomain(T& app, const QString& domain)
{
  app.setOrganizationDomain(domain);
}

template<typename T>
void
qapplicationSetOrganizationName(T& app, const QString& name)
{
  app.setOrganizationName(name);
}

}
}

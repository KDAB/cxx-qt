// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#ifdef CXX_QT_GUI_FEATURE
#include "cxx-qt-lib/qguiapplication.h"

#include <vector>

#include <QtCore/QObject>

namespace {

class ArgsData : public QObject
{
public:
  explicit ArgsData(const QVector<QByteArray>& args, QObject* parent = nullptr)
    : QObject(parent)
    , m_ownedVector(args)
  {
    // Construct our vector of char*
    for (auto& bytes : m_ownedVector) {
      m_vector.emplace_back(bytes.data());
      m_size += 1;
    }
  }

  char** data() { return m_vector.data(); }
  int& size() { return m_size; }

private:
  QVector<QByteArray> m_ownedVector;
  int m_size = 0;
  std::vector<char*> m_vector;
};

}

namespace rust {
namespace cxxqtlib1 {

::std::unique_ptr<QGuiApplication>
qguiapplicationNew(const QVector<QByteArray>& args)
{
  // Ensure that our QVector has the same lifetime as the QGuiApplication
  // by storing it inside a QObject that has QGuiApplication as it's parent
  auto argsData = new ArgsData(args);
  // Note that QGuiApplication uses a reference to an int for the size here
  // so we need to ensure that reference remains valid
  auto ptr =
    ::std::make_unique<QGuiApplication>(argsData->size(), argsData->data());
  Q_ASSERT(ptr != nullptr);
  argsData->setParent(ptr.get());

  return ptr;
}

void
qguiapplicationAddLibraryPath(QGuiApplication& app, const QString& path)
{
  app.addLibraryPath(path);
}

QString
qguiapplicationApplicationName(const QGuiApplication& app)
{
  return app.applicationName();
}

QString
qguiapplicationApplicationVersion(const QGuiApplication& app)
{
  return app.applicationVersion();
}

::std::int32_t
qguiapplicationExec(QGuiApplication& app)
{
  return static_cast<::std::int32_t>(app.exec());
}

QStringList
qguiapplicationLibraryPaths(const QGuiApplication& app)
{
  return app.libraryPaths();
}

QString
qguiapplicationOrganizationDomain(const QGuiApplication& app)
{
  return app.organizationDomain();
}

QString
qguiapplicationOrganizationName(const QGuiApplication& app)
{
  return app.organizationName();
}

void
qguiapplicationSetApplicationName(QGuiApplication& app, const QString& name)
{
  app.setApplicationName(name);
}

void
qguiapplicationSetApplicationVersion(QGuiApplication& app,
                                     const QString& version)
{
  app.setApplicationVersion(version);
}

void
qguiapplicationSetLibraryPaths(QGuiApplication& app, const QStringList& paths)
{
  app.setLibraryPaths(paths);
}

void
qguiapplicationSetOrganizationDomain(QGuiApplication& app,
                                     const QString& domain)
{
  app.setOrganizationDomain(domain);
}

void
qguiapplicationSetOrganizationName(QGuiApplication& app, const QString& name)
{
  app.setOrganizationName(name);
}

}
}
#endif

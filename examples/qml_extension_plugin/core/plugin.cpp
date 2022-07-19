// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// ANCHOR: book_qml_plugin
#include <QQmlEngine>
#include <QQmlExtensionPlugin>

#include "cxx-qt-gen/include/my_object.cxxqt.h"

class CoreQmlpluginPlugin : public QQmlExtensionPlugin
{
  Q_OBJECT
  Q_PLUGIN_METADATA(IID QQmlExtensionInterface_iid)

public:
  void registerTypes(const char* uri) override
  {
    qmlRegisterType<cxx_qt::my_object::MyObject>(uri, 1, 0, "MyObject");
  }
};

#include "plugin.moc"
// ANCHOR_END: book_qml_plugin

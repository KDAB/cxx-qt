// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QModelIndex>
#include <QtCore/QStringListModel>
#include <QtTest/QTest>

#include "cxx-qt-gen/qmodelindex_cxx.cxx.h"

class QModelIndexTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto i = construct_qmodelindex();
    QCOMPARE(i.isValid(), false);
  }

  void read()
  {
    const auto model =
      QStringListModel(QStringList() << QStringLiteral("kdab"));
    QVERIFY(read_qmodelindex(model.index(0)));
  }

  void clone()
  {
    const auto model =
      QStringListModel(QStringList() << QStringLiteral("kdab"));
    const auto c = clone_qmodelindex(model.index(0));
    QCOMPARE(c.isValid(), true);
    QCOMPARE(c.row(), 0);
  }
};

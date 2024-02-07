// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QPersistentModelIndex>
#include <QtGui/QStandardItem>
#include <QtGui/QStandardItemModel>
#include <QtTest/QTest>

#include "cxx-qt-gen/qpersistentmodelindex.cxx.h"

class QPersistentModelIndexTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto i = construct_qpersistentmodelindex();
    QCOMPARE(i.isValid(), false);
  }

  void read()
  {
    auto model = QStandardItemModel();
    model.appendRow(new QStandardItem(QStringLiteral("kdab")));
    const auto persistentIndex = QPersistentModelIndex(model.index(0, 0));
    QVERIFY(read_qpersistentmodelindex(persistentIndex, 0));

    // Insert a row before and check that the persistent index moves
    model.insertRow(0, new QStandardItem(QStringLiteral("qt")));
    QVERIFY(read_qpersistentmodelindex(persistentIndex, 1));
  }

  void clone()
  {
    auto model = QStandardItemModel();
    model.appendRow(new QStandardItem(QStringLiteral("kdab")));
    const auto c =
      clone_qpersistentmodelindex(QPersistentModelIndex(model.index(0, 0)));
    QCOMPARE(c.isValid(), true);
    QCOMPARE(c.row(), 0);

    // Insert a row before and check that the persistent index moves
    model.insertRow(0, new QStandardItem(QStringLiteral("qt")));
    QCOMPARE(c.isValid(), true);
    QCOMPARE(c.row(), 1);
  }
};

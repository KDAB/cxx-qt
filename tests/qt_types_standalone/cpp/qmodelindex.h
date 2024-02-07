// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QAbstractListModel>
#include <QtCore/QModelIndex>
#include <QtCore/QStringListModel>
#include <QtTest/QTest>

#include "cxx-qt-gen/qmodelindex.cxx.h"

// We subclass from QAbstractListModel to have a valid model to use for
// access to createIndex();
class QModelIndexTest : public QAbstractListModel
{
  Q_OBJECT

public:
  int rowCount(const QModelIndex& parent = QModelIndex()) const override
  {
    return 0;
  }

  QVariant data(const QModelIndex& index,
                int role = Qt::DisplayRole) const override
  {
    return QVariant();
  }

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

  void internalPointer()
  {
    const auto index = createIndex(0, 0, (void*)&my_data);

    auto pointer = internal_pointer_qmodelindex(index);
    QCOMPARE((int*)pointer, &my_data);
  }

  void internalId()
  {
    const auto index = createIndex(0, 0, (quintptr)1234);

    auto id = internal_id_qmodelindex(index);
    QCOMPARE(id, 1234);
  }

private:
  int my_data = 42;
};

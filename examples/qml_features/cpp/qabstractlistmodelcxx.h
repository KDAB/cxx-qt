// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QAbstractListModel>

#include "rust/cxx.h"

class QAbstractListModelCXX : public QAbstractListModel
{
public:
  explicit QAbstractListModelCXX(QObject* parent = nullptr)
    : QAbstractListModel(parent)
  {
  }

  // Can't define in CXX as they are protected
  // so crate public methods that are proxied
  void beginInsertRows(int first, int last)
  {
    QAbstractItemModel::beginInsertRows(QModelIndex(), first, last);
  }

  void endInsertRows() { QAbstractItemModel::endInsertRows(); }

  void beginRemoveRows(int first, int last)
  {
    QAbstractItemModel::beginRemoveRows(QModelIndex(), first, last);
  }

  void endRemoveRows() { QAbstractItemModel::endRemoveRows(); }

  void beginResetModel() { QAbstractItemModel::beginResetModel(); }

  void endResetModel() { QAbstractItemModel::endResetModel(); }
};

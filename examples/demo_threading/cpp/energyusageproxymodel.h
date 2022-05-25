// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <optional>

#include <QAbstractListModel>
#include <QVector>

#include "cxx-qt-gen/energy_usage.cxxqt.h"

class EnergyUsageProxyModel : public QAbstractListModel
{
  Q_OBJECT

  Q_PROPERTY(cxx_qt::energy_usage::EnergyUsage* sourceModel READ sourceModel
               WRITE setSourceModel NOTIFY sourceModelChanged)
public:
  enum EnergyRoles
  {
    Uuid = Qt::UserRole + 1,
    Power,
  };
  Q_ENUM(EnergyRoles)

  explicit EnergyUsageProxyModel(QObject* parent = nullptr);

  cxx_qt::energy_usage::EnergyUsage* sourceModel() const;

  // QAbstractListModel
  QHash<int, QByteArray> roleNames() const override;
  int rowCount(const QModelIndex& parent = QModelIndex()) const override;
  QVariant data(const QModelIndex& index,
                int role = Qt::DisplayRole) const override;
  QModelIndex index(int row,
                    int column = 0,
                    const QModelIndex& parent = QModelIndex()) const override;

  std::optional<int> indexOf(const QString& uuid) const;

public Q_SLOTS:
  void setSourceModel(cxx_qt::energy_usage::EnergyUsage* energyUsage);

private Q_SLOTS:
  void onSensorAdded(const QString& uuid);
  void onSensorChanged(const QString& uuid);
  void onSensorRemoved(const QString& uuid);

Q_SIGNALS:
  void sourceModelChanged();

private:
  cxx_qt::energy_usage::EnergyUsage* m_sourceModel = nullptr;
  QVector<QString> m_uuids;
};

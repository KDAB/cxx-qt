// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Leon Matthes <leon.matthes@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "energyusageproxymodel.h"

EnergyUsageProxyModel::EnergyUsageProxyModel(QObject* parent)
  : QAbstractListModel(parent)
{
}

cxx_qt::energy_usage::EnergyUsage*
EnergyUsageProxyModel::sourceModel() const
{
  return m_sourceModel;
}

void
EnergyUsageProxyModel::setSourceModel(
  cxx_qt::energy_usage::EnergyUsage* sourceModel)
{
  if (m_sourceModel != sourceModel) {
    if (m_sourceModel) {
      m_sourceModel->disconnect(this);
    }

    m_sourceModel = sourceModel;

    if (m_sourceModel) {
      connect(m_sourceModel,
              &cxx_qt::energy_usage::EnergyUsage::sensorAdded,
              this,
              &EnergyUsageProxyModel::onSensorAdded);
      connect(m_sourceModel,
              &cxx_qt::energy_usage::EnergyUsage::sensorChanged,
              this,
              &EnergyUsageProxyModel::onSensorChanged);
      connect(m_sourceModel,
              &cxx_qt::energy_usage::EnergyUsage::sensorRemoved,
              this,
              &EnergyUsageProxyModel::onSensorRemoved);
    }

    Q_EMIT sourceModelChanged();
  }
}

QHash<int, QByteArray>
EnergyUsageProxyModel::roleNames() const
{
  static const QHash<int, QByteArray> roles{
    { EnergyRoles::Uuid, QByteArrayLiteral("uuid") },
    { EnergyRoles::Power, QByteArrayLiteral("power") },
  };
  return roles;
}

int
EnergyUsageProxyModel::rowCount(const QModelIndex& parent) const
{
  return m_uuids.size();
}

QVariant
EnergyUsageProxyModel::data(const QModelIndex& index, int role) const
{

  if (!hasIndex(index.row(), index.column(), index.parent()))
    return {};

  auto& uuid = m_uuids[index.row()];
  switch (role) {
    case EnergyRoles::Uuid:
      return uuid;
    case EnergyRoles::Power: {
      if (m_sourceModel) {
        return m_sourceModel->sensorPower(uuid);
      } else {
        return {};
      }
    }
    default:
      return {};
  }
}

QModelIndex
EnergyUsageProxyModel::index(int row,
                             int column,
                             const QModelIndex& parent) const
{

  if (parent.isValid())
    return {};

  return createIndex(row, column);
}

std::optional<int>
EnergyUsageProxyModel::indexOf(const QString& uuid) const
{
  const auto index = m_uuids.indexOf(uuid);
  if (index < 0) {
    return std::nullopt;
  } else {
    return index;
  }
}

void
EnergyUsageProxyModel::onSensorAdded(const QString& uuid)
{
  beginInsertRows(QModelIndex(), m_uuids.size(), m_uuids.size());
  m_uuids.append(uuid);
  endInsertRows();
}

void
EnergyUsageProxyModel::onSensorChanged(const QString& uuid)
{
  const auto uuidIndex = indexOf(uuid);
  if (!uuidIndex.has_value()) {
    return;
  }
  const auto row = index(uuidIndex.value());
  Q_EMIT dataChanged(row, row, { EnergyRoles::Power });
}

void
EnergyUsageProxyModel::onSensorRemoved(const QString& uuid)
{
  const auto index = indexOf(uuid);
  if (!index.has_value()) {
    return;
  }
  beginRemoveRows(QModelIndex(), index.value(), index.value());
  m_uuids.removeAt(index.value());
  endRemoveRows();
}

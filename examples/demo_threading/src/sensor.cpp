// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "sensor.h"

Sensor::Sensor(QObject* parent)
  : QObject(parent)
{
  connect(this, &Sensor::uuidChanged, this, &Sensor::findUuid);
  connect(this, &Sensor::modelChanged, this, &Sensor::findUuid);
}

void
Sensor::findUuid()
{
  if (m_model) {
    m_index = m_model->indexOf(m_uuid);
  }

  Q_EMIT onChanged();
  Q_EMIT powerChanged();
}

EnergyUsageProxyModel*
Sensor::model() const
{
  return m_model;
}

bool
Sensor::isOn() const
{
  return m_index.has_value();
}

double
Sensor::power() const
{
  if (m_model && m_index) {
    return m_model
      ->data(m_model->index(m_index.value()), EnergyUsageProxyModel::Power)
      .toDouble();
  } else {
    return 0.0;
  }
}

void
Sensor::onModelDataChanged(const QModelIndex& topLeft,
                           const QModelIndex& bottomRight,
                           const QVector<int>& roles)
{
  if (m_index >= topLeft.row() && m_index <= bottomRight.row() &&
      roles.contains(EnergyUsageProxyModel::Power)) {
    Q_EMIT powerChanged();
  }
}

void
Sensor::setModel(EnergyUsageProxyModel* model)
{
  if (m_model != model) {
    if (m_model) {
      m_model->disconnect(this);
    }

    m_model = model;

    if (m_model) {
      connect(m_model,
              &QAbstractListModel::dataChanged,
              this,
              &Sensor::onModelDataChanged);
      connect(
        m_model, &QAbstractListModel::rowsInserted, this, &Sensor::findUuid);
      connect(
        m_model, &QAbstractListModel::rowsRemoved, this, &Sensor::findUuid);
      connect(
        m_model, &QAbstractListModel::modelReset, this, &Sensor::findUuid);
    }

    Q_EMIT modelChanged();
  }
}

void
Sensor::setUuid(const QString& uuid)
{
  if (m_uuid != uuid) {
    m_uuid = uuid;

    Q_EMIT uuidChanged();
  }
}

QString
Sensor::uuid() const
{
  return m_uuid;
}

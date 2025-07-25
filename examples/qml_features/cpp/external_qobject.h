// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QObject>

#include <cstdint>

class ExternalQObject : public QObject
{
  Q_OBJECT

public:
  explicit ExternalQObject(QObject* parent = nullptr);

  Q_INVOKABLE void trigger(::std::uint32_t amount);

Q_SIGNALS:
  void triggered();
  void triggeredPrivateSignal(QPrivateSignal);
  void triggeredConstSignal() const;
};

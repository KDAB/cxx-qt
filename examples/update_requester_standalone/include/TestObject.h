// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include "rust/cxx_qt.h"

class TestObject : public CxxQObject
{
  Q_OBJECT

public:
  TestObject(QObject* parent = nullptr)
    : CxxQObject(parent)
  {}
  ~TestObject() = default;

private:
  Q_INVOKABLE void requestUpdate()
  {
    runOnGUIThread([&]() { updateState(); });
  }
  void updateState() { Q_EMIT updateStateRequested(); }

Q_SIGNALS:
  void updateStateRequested();
};

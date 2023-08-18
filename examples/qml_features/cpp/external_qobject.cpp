// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "external_qobject.h"

ExternalQObject::ExternalQObject(QObject* parent)
  : QObject(parent)
{
}

void
ExternalQObject::trigger(::std::uint32_t amount)
{
  for (::std::uint32_t i = 0; i < amount; i++) {
    Q_EMIT triggered();
    Q_EMIT triggeredPrivateSignal(QPrivateSignal());
  }
}

// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <QtWidgets/QApplication>

#include <memory>

::std::unique_ptr<QApplication>
qapplicationNew(const QVector<QByteArray>& args);

template<typename T>
::std::int32_t
qapplicationExec(T& app)
{
  return static_cast<::std::int32_t>(app.exec());
}

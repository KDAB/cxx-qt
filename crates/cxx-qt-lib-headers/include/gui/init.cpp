// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include <cxx-qt-lib/qlist.h>
#include <cxx-qt-lib/qvector.h>

static const int register_QList_QColor =
  qRegisterMetaType<::QList_QColor>("QList_QColor");
static const int register_QVector_QColor =
  qRegisterMetaType<::QVector_QColor>("QVector_QColor");

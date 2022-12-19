// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QObject>
#include <QVariant>

struct CustomStruct
{
  int value;
};
Q_DECLARE_METATYPE(CustomStruct);

bool
qvariantCanConvertCustomStruct(const QVariant& variant);

class CustomObject : public QObject
{
  Q_OBJECT

  Q_PROPERTY(int value MEMBER m_value)
public:
  CustomObject(QObject* parent = nullptr)
    : QObject(parent)
    , m_value(0)
  {
  }

  Q_INVOKABLE CustomStruct asStruct() const { return CustomStruct{ m_value }; }

private:
  int m_value;
};

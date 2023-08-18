// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QObject>
#include <QtCore/QVariant>

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
  explicit CustomObject(QObject* parent = nullptr);

  Q_INVOKABLE CustomStruct asStruct() const;

private:
  int m_value;
};

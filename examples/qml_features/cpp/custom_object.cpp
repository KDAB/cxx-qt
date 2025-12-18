// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "qml_features/custom_object.h"

bool
qvariantCanConvertCustomStruct(const QVariant& variant)
{
  return variant.canConvert<CustomStruct>();
}

CustomObject::CustomObject(QObject* parent)
  : QObject(parent)
  , m_value(0)
{
}

CustomStruct
CustomObject::asStruct() const
{
  return CustomStruct{ m_value };
}

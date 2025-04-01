// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once
#include <QtCore/QObject>
#include <type_traits>

namespace rust::cxxqt1 {

template<typename Sub, typename Base>
const Base*
upcastPtr(const Sub* sub)
{
  static_assert(std::is_base_of_v<Base, Sub>);
  return static_cast<const Base*>(sub);
}

// Main downcasting function, for non-QObject types
template<typename Sub,
         typename Base,
         std::enable_if_t<!std::is_base_of_v<QObject, Sub>, bool> = true>
const Sub*
downcastPtr(const Base* base)
{
  static_assert(std::is_base_of_v<Base, Sub>);
  static_assert(std::is_polymorphic_v<Sub>,
                "Downcasting requires a polymorphic type (e.g. a type with at "
                "least one virtual method)!");
  return dynamic_cast<const Sub*>(base);
}

// Downcasting function for QObject types, enabled via SFINAE
template<typename Sub,
         typename Base,
         std::enable_if_t<std::is_base_of_v<QObject, Sub>, bool> = true>
const Sub*
downcastPtr(const Base* base)
{
  static_assert(std::is_base_of_v<Base, Sub>);
  return qobject_cast<const Sub*>(base);
}

// Warning: This function is highly unsafe, use with caution!
// It is only safe to use if you are 100% sure that the Sub and Base types are
// of the same size and that all Base instances are also valid Sub instances!
//
// This is mostly the case for types like QStringList, that are just an
// extension of their parent types and don't add any data members.
template<typename Sub, typename Base>
const Sub*
downcastPtrStatic(const Base* base)
{
  static_assert(std::is_base_of_v<Base, Sub>);
  static_assert(sizeof(Base) == sizeof(Sub));
  return static_cast<const Sub*>(base);
}

}

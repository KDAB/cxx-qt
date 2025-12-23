// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "cxx-qt-lib/qmetatype.h"

#include <cxx-qt-lib/assertion_utils.h>

#include <QtCore/QMarginsF>

assert_alignment_and_size(QMetaType, { ::std::size_t a0; });

static_assert(::std::is_trivially_copy_assignable<QMetaType>::value);
static_assert(::std::is_trivially_copy_constructible<QMetaType>::value);
static_assert(::std::is_trivially_destructible<QMetaType>::value);
static_assert(::std::is_move_constructible<QMetaType>::value);
static_assert(QTypeInfo<QMetaType>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {
int
qmetatypeId(const QMetaType& metaType)
{
  return metaType.id();
}

bool
qmetatypeIsRegistered(const QMetaType& metaType)
{
  return metaType.isRegistered();
}

bool
qmetatypeIsValid(const QMetaType& metaType)
{
  return metaType.isValid();
}

QMetaType
qmetatypeFromName(::rust::Slice<const ::std::uint8_t> typeName)
{
  return QMetaType::fromName(
    QByteArrayView(typeName.data(), typeName.length()));
}

}
}

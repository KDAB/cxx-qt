// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qstringlist.h"

#include "assertion_utils.h"

// The layout has changed between Qt 5 and Qt 6
//
// Qt5 QStringList has one pointer as a member
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qlist.h?h=v5.15.6-lts-lgpl#n157
//
// Qt6 QStringList has one member, which contains two pointers and a size_t
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qlist.h?h=v6.2.4#n110
// DataPointer is then a QArrayDataPointer<QString>
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/tools/qarraydatapointer.h?h=v6.2.4#n390
#if (QT_VERSION >= QT_VERSION_CHECK(6, 0, 0))
assert_alignment_and_size(QStringList,
                          alignof(::std::size_t),
                          sizeof(::std::size_t[3]));
#else
assert_alignment_and_size(QStringList,
                          alignof(::std::size_t),
                          sizeof(::std::size_t));
#endif

static_assert(!::std::is_trivially_copy_assignable<QStringList>::value);
static_assert(!::std::is_trivially_copy_constructible<QStringList>::value);
static_assert(!::std::is_trivially_destructible<QStringList>::value);

static_assert(QTypeInfo<QStringList>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

bool
qstringlistContains(const QStringList& list, const QString& string)
{
  return list.contains(string);
}

QStringList
qstringlistFromQListQString(const QList<QString>& list)
{
  return QStringList(list);
}

QList<QString>
qstringlistAsQListQString(const QStringList& list)
{
  // Cast to a QList then copy it
  const auto list_cast = static_cast<QList<QString>>(list);
  return QList<QString>(list_cast);
}

}
}

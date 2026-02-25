// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qanystringview.h"

#include <cxx-qt-lib/assertion_utils.h>

// QAnyStringView has two members.
// A union of (void*, char*, char_16*) and a size_t.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/text/qanystringview.h
assert_alignment_and_size(QAnyStringView, {
  ::std::size_t a0;
  ::std::size_t a1;
});

static_assert(::std::is_trivially_copy_assignable<QAnyStringView>::value);
static_assert(::std::is_trivially_copy_constructible<QAnyStringView>::value);

static_assert(::std::is_trivially_destructible<QAnyStringView>::value);

static_assert(QTypeInfo<QAnyStringView>::isRelocatable);

namespace rust {
namespace cxxqtlib1 {

QAnyStringView
qanystringviewInitFromRustString(::rust::Str string)
{
  return QAnyStringView(string.data(), string.size());
}

::rust::isize
qanystringviewLen(const QAnyStringView& string)
{
  return static_cast<::rust::isize>(string.size());
}

}
}

// clang-format off
// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Laurent Montel <laurent.montel@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib-extras/include/qelapsedtimer.h"

#include <cxx-qt-lib/assertion_utils.h>

#include <cstdint>

// QElapsedTimer has two "int64" members
// Rust represents these as 2 64-bit integers.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qelapsedtimer.h#n57
//
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/kernel/qelapsedtimer.h?h=v6.2.4#n89
assert_alignment_and_size(QElapsedTimer, {
  ::std::int64_t a0;
  ::std::int64_t a1;
});

static_assert(::std::is_trivially_copyable<QElapsedTimer>::value,
              "QElapsedTimer must be trivially copyable!");

namespace rust {
namespace cxxqtlib1 {

::std::int64_t
qelapsedtimerRestart(QElapsedTimer& elapsedTimer)
{
  return static_cast<::std::int64_t>(elapsedTimer.restart());
}

}
}

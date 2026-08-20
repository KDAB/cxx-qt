// clang-format off
// SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qmessagelogger.h"

#include <cxx-qt-lib/assertion_utils.h>

#include <QtCore/QLoggingCategory>

assert_alignment_and_size(QMessageLogger, { QMessageLogContext context; });

static_assert(::std::is_trivially_destructible<QMessageLogger>::value);

namespace rust {
namespace cxxqtlib1 {
bool
isLoggingCategoryEnabled(const QMessageLogContext& context,
                         QtMsgType enableForLevel)
{
#if defined(QT_NO_DEBUG_OUTPUT)
  if (enableForLevel == QtMsgType::QtDebugMsg) {
    return false;
  }
#endif
#if defined(QT_NO_INFO_OUTPUT)
  if (enableForLevel == QtMsgType::QtInfoMsg) {
    return false;
  }
#endif
#if defined(QT_NO_WARNING_OUTPUT)
  if (enableForLevel == QtMsgType::QtWarningMsg) {
    return false;
  }
#endif
  if (context.category == nullptr || strcmp(context.category, "default") == 0) {
    if (QLoggingCategory* defaultCategory =
          QLoggingCategory::defaultCategory()) {
      return defaultCategory->isEnabled(enableForLevel);
    }
  }
  return QLoggingCategory(context.category).isEnabled(enableForLevel);
}
}
}

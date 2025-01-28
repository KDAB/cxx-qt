// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Nicolas Fella <nicolas.fella@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QLocale>

#include "rust/cxx.h"

namespace rust {

template<>
struct IsRelocatable<QLocale> : ::std::true_type
{};

namespace cxxqtlib1 {

using QLocaleTagSeparator = QLocale::TagSeparator;
using QLocaleCurrencySymbolFormat = QLocale::CurrencySymbolFormat;
using QLocaleFormatType = QLocale::FormatType;
using QLocaleLanguage = QLocale::Language;
using QLocaleMeasurementSystem = QLocale::MeasurementSystem;

}
}

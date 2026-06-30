// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QAnyStringView>
#include <QtCore/QByteArray>
#include <QtCore/QString>

#include "rust/cxx.h"

namespace rust {
namespace cxxqtlib1 {

QAnyStringView
qanystringviewInitFromRustString(::rust::Str string);

::rust::isize
qanystringviewLen(const QAnyStringView& string);

}
}

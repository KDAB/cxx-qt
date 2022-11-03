// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QSet>

#include <QtCore/QDate>
#include <QtCore/QDateTime>
#include <QtCore/QString>
#include <QtCore/QTime>
#include <QtCore/QUrl>

#include "rust/cxx.h"

// This has static asserts in the cpp file to ensure this is valid.
template<typename T>
struct rust::IsRelocatable<QSet<T>> : std::true_type
{
};

// Note on the Rust side we rename the methods to "clear" or "len" etc
// so we need to put each set in it's own namespace otherwise the generated
// CXX code collides.
#define CXX_QT_QSET_METHODS(typeName, name)                                    \
  using QSet_##name = QSet<typeName>;                                          \
                                                                               \
  namespace rust {                                                             \
  namespace cxxqtlib1 {                                                        \
  namespace qset_##name                                                        \
  {                                                                            \
    void qset_clear_##name(QSet_##name& s) noexcept;                           \
    QSet_##name qset_clone_##name(const QSet_##name& s) noexcept;              \
    bool qset_contains_##name(const QSet_##name& s,                            \
                              const typeName& value) noexcept;                 \
    QSet_##name qset_default_##name() noexcept;                                \
    void qset_drop_##name(QSet_##name& s) noexcept;                            \
    const typeName& qset_get_unchecked_##name(const QSet_##name& s,            \
                                              ::std::size_t pos) noexcept;     \
    void qset_insert_##name(QSet_##name& s, const typeName& value) noexcept;   \
    std::size_t qset_len_##name(const QSet_##name& s) noexcept;                \
    bool qset_remove_##name(QSet_##name& s, const typeName& value) noexcept;   \
  }                                                                            \
  }                                                                            \
  }

CXX_QT_QSET_METHODS(bool, bool);
CXX_QT_QSET_METHODS(float, f32);
CXX_QT_QSET_METHODS(double, f64);
CXX_QT_QSET_METHODS(::qint8, i8);
CXX_QT_QSET_METHODS(::qint16, i16);
CXX_QT_QSET_METHODS(::qint32, i32);
CXX_QT_QSET_METHODS(::QDate, QDate);
CXX_QT_QSET_METHODS(::QDateTime, QDateTime);
CXX_QT_QSET_METHODS(::QString, QString);
CXX_QT_QSET_METHODS(::QTime, QTime);
CXX_QT_QSET_METHODS(::QUrl, QUrl);
CXX_QT_QSET_METHODS(::quint8, u8);
CXX_QT_QSET_METHODS(::quint16, u16);
CXX_QT_QSET_METHODS(::quint32, u32);

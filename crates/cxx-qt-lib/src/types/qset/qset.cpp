// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qset.h"

#include "../assertion_utils.h"

#define CXX_QT_QSET_ASSERTS(typeName, name)                                    \
  assert_alignment_and_size(                                                   \
    QSet_##name, alignof(std::size_t), sizeof(std::size_t));                   \
                                                                               \
  static_assert(!std::is_trivially_copy_assignable<QSet_##name>::value);       \
  static_assert(!std::is_trivially_copy_constructible<QSet_##name>::value);    \
  static_assert(!std::is_trivially_destructible<QSet_##name>::value);          \
                                                                               \
  static_assert(QTypeInfo<QSet_##name>::isRelocatable);                        \
                                                                               \
  static_assert(std::is_copy_assignable<typeName>::value);                     \
  static_assert(std::is_copy_constructible<typeName>::value);

#define CXX_QT_QSET_METHODS_IMPL(typeName, name)                               \
  CXX_QT_QSET_ASSERTS(typeName, name);                                         \
                                                                               \
  namespace rust {                                                             \
  namespace cxxqtlib1 {                                                        \
  namespace qset_##name                                                        \
  {                                                                            \
    void qset_clear_##name(QSet_##name& s) noexcept                            \
    {                                                                          \
      s.clear();                                                               \
    }                                                                          \
                                                                               \
    QSet_##name qset_clone_##name(const QSet_##name& s) noexcept               \
    {                                                                          \
      return QSet(s);                                                          \
    }                                                                          \
                                                                               \
    bool qset_contains_##name(const QSet_##name& s,                            \
                              const typeName& value) noexcept                  \
    {                                                                          \
      return s.contains(value);                                                \
    }                                                                          \
                                                                               \
    QSet_##name qset_default_##name() noexcept                                 \
    {                                                                          \
      return QSet_##name();                                                    \
    }                                                                          \
                                                                               \
    void qset_drop_##name(QSet_##name& s) noexcept                             \
    {                                                                          \
      s.~QSet_##name();                                                        \
    }                                                                          \
                                                                               \
    const typeName& qset_get_unchecked_##name(const QSet_##name& s,            \
                                              ::std::size_t pos) noexcept      \
    {                                                                          \
      Q_ASSERT(pos < static_cast<::std::size_t>(s.size()));                    \
      auto it = s.cbegin();                                                    \
      std::advance(it, pos);                                                   \
      return *it;                                                              \
    }                                                                          \
                                                                               \
    void qset_insert_##name(QSet_##name& s, const typeName& value) noexcept    \
    {                                                                          \
      s.insert(value);                                                         \
    }                                                                          \
                                                                               \
    ::std::size_t qset_len_##name(const QSet_##name& s) noexcept               \
    {                                                                          \
      return static_cast<::std::size_t>(s.size());                             \
    }                                                                          \
                                                                               \
    bool qset_remove_##name(QSet_##name& s, const typeName& value) noexcept    \
    {                                                                          \
      return s.remove(value);                                                  \
    }                                                                          \
  }                                                                            \
  }                                                                            \
  }

CXX_QT_QSET_METHODS_IMPL(bool, bool);
CXX_QT_QSET_METHODS_IMPL(float, f32);
CXX_QT_QSET_METHODS_IMPL(double, f64);
CXX_QT_QSET_METHODS_IMPL(::qint8, i8);
CXX_QT_QSET_METHODS_IMPL(::qint16, i16);
CXX_QT_QSET_METHODS_IMPL(::qint32, i32);
CXX_QT_QSET_METHODS_IMPL(::QDate, QDate);
CXX_QT_QSET_METHODS_IMPL(::QDateTime, QDateTime);
CXX_QT_QSET_METHODS_IMPL(::QString, QString);
CXX_QT_QSET_METHODS_IMPL(::QTime, QTime);
CXX_QT_QSET_METHODS_IMPL(::QUrl, QUrl);
CXX_QT_QSET_METHODS_IMPL(::quint8, u8);
CXX_QT_QSET_METHODS_IMPL(::quint16, u16);
CXX_QT_QSET_METHODS_IMPL(::quint32, u32);

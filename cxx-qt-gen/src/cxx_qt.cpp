// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#ifndef NO_QT

#include "rust/cxx_qt.h"

#include <QPointer>

// TODO: for UpdateRequester also use funky C function names with lots of $
// signs like CXX to prevent possible symbol collisions.

using QPtr = QPointer<CxxQObject>;

// UpdateRequester is simply a wrapper around QPtr which allows for Rust code to
// post an event to a specific CxxQObject.
//
// We use QPointer as this allows us to detect when our pointer no longer points
// to a valid QObject as a result of the object having been deleted by C++ code
// for whatever reason.
//
// As Rust does not understand how QPointer works, we give it a QPtr* and
// provide the below C functions to operate on it. This QPtr is intended to be
// owned by a Rust object and is not supposed to ever be shared elsewhere as the
// Rust object takes control of deleting the memory behind the pointer.
//
// The reason that a QPtr* is used instead of constructing a QPtr directly into
// Rust allocated memory of the correct size is to prevent the perils that can
// result from Rust trying to move said memory. If we only give a pointer to
// Rust though, it is free to move that in memory as much as it likes.

extern "C"
{
  QPtr* cxx_qt_update_requester_new(CxxQObject* qobject_ptr) noexcept
  {
    Q_ASSERT(qobject_ptr != nullptr);
    return new QPtr(qobject_ptr);
  }

  void cxx_qt_update_requester_drop(QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);
    delete self;
  }

  bool cxx_qt_update_requester_request_update(const QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);

    if (self->isNull())
      return false;

    self->data()->requestUpdate();
    return true;
  }

  QPtr* cxx_qt_update_requester_clone(const QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);
    return new QPtr(*self);
  }
}

namespace {

// We do these checks to ensure that we can safely store a QString
// inside a block of memory that Rust thinks contains as usize.
//
// We assume that std::size_t is the same size as Rust's usize.
// cxx.cc has some asserts to ensure that is true.

static_assert(alignof(QString) <= alignof(std::size_t),
              "unexpectedly large QString alignment");

static_assert(sizeof(QString) <= sizeof(std::size_t),
              "unexpectedly large QString size");

// We also assume that C++ char and Rust u8 are the same

static_assert(sizeof(char) == sizeof(std::uint8_t));

} // namespace

extern "C"
{
  void cxxqt1$qstring$init(QString* self,
                           const char* ptr,
                           std::size_t len) noexcept
  {
    new (self) QString();
    *self = QString::fromUtf8(ptr, len);
  }

  void cxxqt1$qstring$to_rust_string(const QString& qt,
                                     rust::String& rust) noexcept
  {
    static_assert(sizeof(char16_t) == sizeof(QChar));
    rust = rust::String(reinterpret_cast<const char16_t*>(qt.constData()),
                        qt.size());
  }

  void cxxqt1$qstring$drop(QString* self) noexcept { self->~QString(); }
}

#endif // NO_QT
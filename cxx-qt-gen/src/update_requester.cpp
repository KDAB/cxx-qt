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
  QPtr* cxx_qt_update_requester_new(CxxQObject* qobject_ptr)
  {
    Q_ASSERT(qobject_ptr != nullptr);
    return new QPtr(qobject_ptr);
  }

  void cxx_qt_update_requester_drop(QPtr* self)
  {
    Q_ASSERT(self != nullptr);
    delete self;
  }

  bool cxx_qt_update_requester_request_update(const QPtr* self)
  {
    Q_ASSERT(self != nullptr);

    if (self->isNull())
      return false;

    self->data()->requestUpdate();
    return true;
  }

  QPtr* cxx_qt_update_requester_clone(const QPtr* self)
  {
    Q_ASSERT(self != nullptr);
    return new QPtr(*self);
  }
}

#endif // NO_QT
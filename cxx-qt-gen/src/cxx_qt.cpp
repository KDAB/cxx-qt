// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#ifndef NO_QT

#include "rust/cxx_qt.h"

#include <QDate>
#include <QDateTime>
#include <QMetaObject>
#include <QPointF>
#include <QPointer>
#include <QRect>
#include <QRectF>
#include <QSize>
#include <QSizeF>
#include <QTime>
#include <QUrl>
#include <QVariant>
#include <QtGui/QColor>

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

using QPtr = QPointer<CxxQObject>;

extern "C"
{
  QPtr* cxxqt1$update_requester$new(CxxQObject* qobject_ptr) noexcept
  {
    Q_ASSERT(qobject_ptr != nullptr);
    return new QPtr(qobject_ptr);
  }

  void cxxqt1$update_requester$drop(QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);
    delete self;
  }

  bool cxxqt1$update_requester$request_update(const QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);

    if (self->isNull())
      return false;

    const auto ret = QMetaObject::invokeMethod(
      self->data(), "requestUpdate", Qt::DirectConnection);

    if (!ret) {
      qWarning() << Q_FUNC_INFO
                 << "Tried to call requestUpdate on object without "
                    "UpdateRequestHandler trait.";
    }

    return ret;
  }

  QPtr* cxxqt1$update_requester$clone(const QPtr* self) noexcept
  {
    Q_ASSERT(self != nullptr);
    return new QPtr(*self);
  }
}

static const QEvent::Type
createEventType(int hint)
{
  auto eventId = QEvent::registerEventType(hint);
  Q_ASSERT(eventId > -1);
  return static_cast<QEvent::Type>(eventId);
}

const QEvent::Type CxxQObject::ProcessQueueEvent =
  createEventType(QEvent::User + 1);

CxxQObject::CxxQObject(QObject* parent)
  : QObject(parent)
{}

CxxQObject::~CxxQObject() = default;

bool
CxxQObject::event(QEvent* event)
{
  if (event->type() == ProcessQueueEvent) {
    // New Rust-side events might come in while we are processing the queue.
    //
    // If we flip this flag before takeQueue then worst case we get an
    // extra event with nothing to actually process whereas if we do it
    // afterwards then we might miss a queue item to process.
    m_waitingForUpdate.store(false, std::memory_order_relaxed);

    for (const auto& item : takeQueue()) {
      item();
    }
    return true;
  }

  return QObject::event(event);
}

void
CxxQObject::runOnGUIThread(std::function<void()> functor)
{
  // Lock the queue, post the event, add to the queue
  // worst case we'll push an event that does nothing if takeQueue() is
  // waiting on the lock
  const std::lock_guard<std::mutex> guard(m_queueMutex);

  if (!m_waitingForUpdate.exchange(true, std::memory_order_relaxed)) {
    QCoreApplication::postEvent(this, new QEvent(ProcessQueueEvent));
  }

  m_queue.push_back(functor);
}

std::vector<std::function<void()>>
CxxQObject::takeQueue()
{
  const std::lock_guard<std::mutex> guard(m_queueMutex);
  std::vector<std::function<void()>> queue;
  std::swap(m_queue, queue);
  return queue;
}

#endif // NO_QT

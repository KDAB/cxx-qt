// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <algorithm>
#include <atomic>
#include <functional>
#include <memory>
#include <mutex>
#include <vector>

#include <QCoreApplication>
#include <QDebug>
#include <QEvent>
#include <QObject>
#include <QString>

#include "rust/cxx.h"

namespace CxxQt {

// NB: this is an "owning" pointer so if you get one of these you
// are responsible for calling cxxqt1$drop$variant drop on it once
// you are done with it.
//
// TODO: find a way to convince cxx to allow us to make Variant a
// class with an explicit destructor yet still allows use to return
// it from a Rust function similar to what it allows with its own
// rust::String type.
typedef void* Variant;

}

extern "C" void
cxxqt1$assign$variant$to$qvariant(const CxxQt::Variant& rust, QVariant& cpp);

extern "C" void
cxxqt1$drop$variant(CxxQt::Variant* self);

// TODO: we probably want to namespace these conversion functions too

inline QString
rustStringToQString(const rust::string& value)
{
  return QString::fromUtf8(value.data(), value.length());
}

inline QString
rustStrToQString(const rust::str& value)
{
  return QString::fromUtf8(value.data(), value.length());
}

namespace CxxQt {

inline QVariant
rustVariantToQVariant(CxxQt::Variant&& rust)
{
  QVariant cpp;
  cxxqt1$assign$variant$to$qvariant(rust, cpp);
  cxxqt1$drop$variant(&rust);
  return cpp;
}

}

enum QueueEvent
{
  EmitSignal,
  UpdatePropertyChange,
  UpdateState
};

class CxxQObject : public QObject
{
  Q_OBJECT

public:
  // TODO: we need to document the existence of UpdateStateEvent for users who
  // want to create custom classes that derive from CxxQObject so that they
  // know to avoid clashes with it.
  static const QEvent::Type ProcessQueueEvent;

public:
  CxxQObject(QObject* parent = nullptr)
    : QObject(parent)
  {}
  virtual ~CxxQObject() = default;

  void requestEmitSignal(std::function<void()> signalFunctor)
  {
    // Lock the queue, post the event, add to the queue
    // worst case we'll push an event that does nothing if takeQueue() is
    // waiting on the lock
    const std::lock_guard<std::mutex> guard(m_queueMutex);

    if (!m_waitingForUpdate.exchange(true, std::memory_order_relaxed)) {
      QCoreApplication::postEvent(this, new QEvent(ProcessQueueEvent));
    }

    m_queue.push_back(std::make_pair(QueueEvent::EmitSignal, signalFunctor));
  }

  void requestPropertyChange(std::function<void()> propertyFunctor)
  {
    // Lock the queue, post the event, add to the queue
    // worst case we'll push an event that does nothing if takeQueue() is
    // waiting on the lock
    const std::lock_guard<std::mutex> guard(m_queueMutex);

    if (!m_waitingForUpdate.exchange(true, std::memory_order_relaxed)) {
      QCoreApplication::postEvent(this, new QEvent(ProcessQueueEvent));
    }

    m_queue.push_back(
      std::make_pair(QueueEvent::UpdatePropertyChange, propertyFunctor));
  }

  void requestUpdate(std::function<void()> updateFunctor)
  {
    // Lock the queue, post the event, add to the queue
    // worst case we'll push an event that does nothing if takeQueue() is
    // waiting on the lock
    const std::lock_guard<std::mutex> guard(m_queueMutex);

    if (!m_waitingForUpdate.exchange(true, std::memory_order_relaxed)) {
      QCoreApplication::postEvent(this, new QEvent(ProcessQueueEvent));
    }

    // Compress request updates to only one
    //
    // TODO: should we compress events? what happens if we change a
    // property/emit signal, request update, change property/emit signal,
    // request update? The second request update won't happen?
    // Or should the request update always happen after the property/signal
    // changes?
    if (std::none_of(
          m_queue.cbegin(),
          m_queue.cend(),
          [](const std::pair<QueueEvent, std::function<void()>>& item) {
            return item.first == QueueEvent::UpdateState;
          })) {
      m_queue.push_back(std::make_pair(QueueEvent::UpdateState, updateFunctor));
    }
  }

  std::vector<std::pair<QueueEvent, std::function<void()>>> takeQueue()
  {
    const std::lock_guard<std::mutex> guard(m_queueMutex);
    std::vector<std::pair<QueueEvent, std::function<void()>>> queue;
    std::swap(m_queue, queue);
    return queue;
  }

  bool event(QEvent* event) override
  {
    if (event->type() == ProcessQueueEvent) {
      // New Rust-side events might come in while we are processing the queue.
      //
      // If we flip this flag before takeQueue then worst case we get an
      // extra event with nothing to actually process whereas if we do it
      // afterwards then we might miss a queue item to process.
      m_waitingForUpdate.store(false, std::memory_order_relaxed);

      for (auto item : takeQueue()) {
        item.second();
      }
      return true;
    }

    return false;
  }

private:
  std::atomic_bool m_waitingForUpdate{ false };
  std::vector<std::pair<QueueEvent, std::function<void()>>> m_queue;
  std::mutex m_queueMutex;
};

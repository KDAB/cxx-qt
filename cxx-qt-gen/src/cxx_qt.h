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
#include <variant>
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

struct QueueItem
{
  enum
  {
    EmitSignal,
    UpdatePropertyChange,
    UpdateState
  } type;
  std::variant<int, std::function<void()>> data;
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

    m_queue.push_back({ QueueItem::EmitSignal, { signalFunctor } });
  }

  void requestPropertyChange(int propertyId)
  {
    // Lock the queue, post the event, add to the queue
    // worst case we'll push an event that does nothing if takeQueue() is
    // waiting on the lock
    const std::lock_guard<std::mutex> guard(m_queueMutex);

    if (!m_waitingForUpdate.exchange(true, std::memory_order_relaxed)) {
      QCoreApplication::postEvent(this, new QEvent(ProcessQueueEvent));
    }

    m_queue.push_back({ QueueItem::UpdatePropertyChange, { propertyId } });
  }

  void requestUpdate()
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
          m_queue.cbegin(), m_queue.cend(), [](const QueueItem& item) {
            return item.type == QueueItem::UpdateState;
          })) {
      m_queue.push_back({ QueueItem::UpdateState, {} });
    }
  }

  std::vector<QueueItem> takeQueue()
  {
    const std::lock_guard<std::mutex> guard(m_queueMutex);
    std::vector<QueueItem> queue;
    std::swap(m_queue, queue);
    return queue;
  }

  bool event(QEvent* event) override
  {
    // TODO: later if CxxQObject is a mixin or member and knows about m_rustObj
    // can the locking for m_rustObj happen here so we only have one lock?
    // also would this change the virtual methods we have now?

    if (event->type() == ProcessQueueEvent) {
      // New Rust-side events might come in while we are processing the queue.
      //
      // If we flip this flag before takeQueue then worst case we get an
      // extra event with nothing to actually process whereas if we do it
      // afterwards then we might miss a queue item to process.
      m_waitingForUpdate.store(false, std::memory_order_relaxed);

      for (auto item : takeQueue()) {
        switch (item.type) {
          case QueueItem::EmitSignal: {
            std::get<std::function<void()>>(item.data)();
            break;
          }
          case QueueItem::UpdatePropertyChange: {
            updatePropertyChange(std::get<int>(item.data));
            break;
          }
          case QueueItem::UpdateState: {
            updateState();
            break;
          }
        }
      }
      return true;
    }

    return false;
  }

Q_SIGNALS:
  void changed();

protected:
  // TODO: once we have implemented code generation for updateState functions we
  // might want to consider making the function pure virtual. Objects that want
  // to opt out of the state mechanism should then instead derive from an
  // entirely different base class as to have less overhead overall.
  virtual void updatePropertyChange(int propertyId)
  {
    qWarning()
      << "An UpdatePropertyEvent event was posted to a CxxQObject that does "
         "not override updatePropertyChange(int), this likely indicates a bug.";
  }

  virtual void updateState()
  {
    qWarning()
      << "An UpdateStateEvent event was posted to a CxxQObject that does not "
         "override updateState(), this likely indicates a bug.";
  };

private:
  std::atomic_bool m_waitingForUpdate{ false };
  std::vector<QueueItem> m_queue;
  std::mutex m_queueMutex;
};

// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <atomic>
#include <memory>

#include <QCoreApplication>
#include <QDebug>
#include <QEvent>
#include <QObject>
#include <QString>

#include "rust/cxx.h"

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

class CxxQObject : public QObject
{
  Q_OBJECT

public:
  // TODO: we need to document the existence of UpdateStateEvent for users who
  // want to create custom classes that derive from CxxQObject so that they
  // know to avoid clashes with it.
  static constexpr QEvent::Type UpdateStateEvent =
    static_cast<QEvent::Type>(QEvent::User + 1);

public:
  CxxQObject(QObject* parent = nullptr)
    : QObject(parent)
  {}
  virtual ~CxxQObject() = default;

  void requestUpdate()
  {
    if (!m_waitingForUpdate.exchange(true, std::memory_order_relaxed))
      QCoreApplication::postEvent(this, new QEvent(UpdateStateEvent));
  }

  bool event(QEvent* event) override
  {
    if (event->type() == UpdateStateEvent) {
      // New Rust-side events might come in while we are processing a request
      // and request a new update while we are processing the queue.
      //
      // If we flip this flag before updateState then worst case we get an extra
      // event with nothing to actually process whereas if we do it afterwards
      // then we might miss an update request.
      m_waitingForUpdate.store(false, std::memory_order_relaxed);

      updateState();
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
  virtual void updateState()
  {
    qWarning()
      << "An UpdateStateEvent event was posted to a CxxQObject that does not "
         "override updateState(), this likely indicates a bug.";
  };

private:
  std::atomic_bool m_waitingForUpdate{ false };
};

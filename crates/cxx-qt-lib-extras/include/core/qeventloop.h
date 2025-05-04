// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QCoreApplication>
#include <QtCore/QEvent>
#include <QtCore/QEventLoop>
#include <climits>

#include "rust/cxx.h"

using QEventLoopProcessEventsFlag = QEventLoop::ProcessEventsFlag;
using QEventLoopProcessEventsFlags = QEventLoop::ProcessEventsFlags;

namespace rust {
namespace cxxqtlib1 {

template<typename T>
class QEventLoopClosureEvent : public QEvent
{
public:
  inline QEventLoopClosureEvent(QEventLoop& eventLoop,
                                T& context,
                                rust::Fn<void(T&)> closure)
    : QEvent(QEvent::User)
    , eventLoop(eventLoop)
    , context(context)
    , closure(closure) {};

  ~QEventLoopClosureEvent() override
  {
    (*closure)(context);
    eventLoop.quit();
  }

private:
  QEventLoop& eventLoop;
  T& context;
  rust::Fn<void(T&)> closure;
};

template<typename T>
int
qeventloopExecWith(QEventLoop& eventLoop,
                   T& context,
                   rust::Fn<void(T&)> closure)
{
  static QObject* receiver = new QObject();
  QEvent* event = new QEventLoopClosureEvent(eventLoop, context, closure);
  QCoreApplication::postEvent(receiver, event, INT_MAX);
  return eventLoop.exec();
}

}
}

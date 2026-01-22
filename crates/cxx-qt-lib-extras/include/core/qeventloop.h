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

namespace rust {
namespace cxxqtlib1 {

using QEventLoopProcessEventsFlag = QEventLoop::ProcessEventsFlag;
using QEventLoopProcessEventsFlags = QEventLoop::ProcessEventsFlags;

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
    QCoreApplication::postEvent(&eventLoop, new QEvent(QEvent::Quit), INT_MIN);
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
  QCoreApplication::postEvent(
    &eventLoop,
    new QEventLoopClosureEvent(eventLoop, context, closure),
    INT_MAX);
  return eventLoop.exec();
}

}
}

// clang-format off
// SPDX-FileCopyrightText: 2021 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Gerhard de Clercq <gerhard.declercq@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <atomic>
#include <functional>
#include <memory>
#include <mutex>
#include <vector>

#include <QCoreApplication>
#include <QDebug>
#include <QEvent>
#include <QObject>

#include "rust/cxx.h"

class CxxQObject : public QObject
{
  Q_OBJECT

public:
  // TODO: we need to document the existence of UpdateStateEvent for users who
  // want to create custom classes that derive from CxxQObject so that they
  // know to avoid clashes with it.
  static const QEvent::Type ProcessQueueEvent;

public:
  CxxQObject(QObject* parent = nullptr);
  virtual ~CxxQObject();

  bool event(QEvent* event) override;

  void runOnGUIThread(std::function<void()> functor);
  std::vector<std::function<void()>> takeQueue();

private:
  std::atomic_bool m_waitingForUpdate{ false };
  std::vector<std::function<void()>> m_queue;
  std::mutex m_queueMutex;
};

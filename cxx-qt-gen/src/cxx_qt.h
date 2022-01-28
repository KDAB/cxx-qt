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
#include <QString>
#include <QVariant>

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

#pragma once

#include <cxx-qt/locking.h>
#include <cxx-qt/maybelockguard.h>
#include <cxx-qt/signalhandler.h>
#include <cxx-qt/type.h>

namespace cxx_qt::my_object {
class MyObject;

} // namespace cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
using MyObjectCxxQtSignalHandlerready =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsready*>;
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
using MyObjectCxxQtSignalHandlerdataChanged =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsdataChanged*>;
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
using MyObjectCxxQtSignalHandlernewData =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsnewData*>;
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
using QTimerCxxQtSignalHandlertimeout =
  ::rust::cxxqt1::SignalHandler<struct QTimerCxxQtSignalParamstimeout*>;
} // namespace rust::cxxqtgen1::cxx_qt::my_object

#include "cxx-qt-gen/ffi.cxx.h"

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
QTimer_timeoutConnect(
  ::cxx_qt::my_object::QTimer& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::QTimerCxxQtSignalHandlertimeout closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
MyObject_readyConnect(
  ::cxx_qt::my_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlerready closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
MyObject_dataChangedConnect(
  ::cxx_qt::my_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlerdataChanged
    closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
MyObject_newDataConnect(
  ::cxx_qt::my_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlernewData
    closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace cxx_qt::my_object {
class MyObject
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<MyObjectRust>
  , public ::rust::cxxqt1::CxxQtLocking
{
  Q_OBJECT
public:
  virtual ~MyObject() = default;

public:
  Q_INVOKABLE void invokable();
  Q_SIGNAL void ready();
  Q_SIGNAL void dataChanged(::std::int32_t first,
                            ::std::unique_ptr<Opaque> second,
                            ::QPoint third,
                            ::QPoint const& fourth);
  explicit MyObject(QObject* parent = nullptr);

private:
  void invokableWrapper() noexcept;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)

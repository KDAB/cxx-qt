#pragma once

#include <cxx-qt/signalhandler.h>
#include <cxx-qt/trycatch.h>
#include <cxx-qt/type.h>

namespace cxx_qt::my_object {
class MyObject;

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlerready =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsready*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlerdata_changed =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsdata_changed*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlernewData =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsnewData*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using QTimerCxxQtSignalHandlertimeout =
  ::rust::cxxqt1::SignalHandler<struct QTimerCxxQtSignalParamstimeout*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

#include "directory/file_ident.cxx.h"

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
QTimer_timeoutConnect(
  cxx_qt::my_object::QTimer& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::QTimerCxxQtSignalHandlertimeout closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_readyConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_data_changedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerdata_changed
    closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_newDataConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlernewData
    closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object {
class MyObject
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<MyObjectRust>
{
  Q_OBJECT
public:
  virtual ~MyObject() = default;

public:
  Q_INVOKABLE void invokable() noexcept;
  Q_SIGNAL void ready();
  Q_SIGNAL void data_changed(::std::int32_t first,
                             ::std::unique_ptr<Opaque> second,
                             QPoint third,
                             QPoint const& fourth);
  explicit MyObject(QObject* parent = nullptr);
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)

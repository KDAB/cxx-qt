#pragma once

#include <cxx-qt/cxxqt_locking.h>
#include <cxx-qt/cxxqt_maybelockguard.h>
#include <cxx-qt/cxxqt_signalhandler.h>
#include <cxx-qt/cxxqt_type.h>

namespace cxx_qt::multi_object {
class MyObject;

} // namespace cxx_qt::multi_object

namespace rust::cxxqtgen1::cxx_qt::multi_object {
using MyObjectCxxQtSignalHandlerpropertyNameChanged =
  ::rust::cxxqt1::SignalHandler<
    struct MyObjectCxxQtSignalParamspropertyNameChanged*>;
} // namespace rust::cxxqtgen1::cxx_qt::multi_object

namespace rust::cxxqtgen1::cxx_qt::multi_object {
using MyObjectCxxQtSignalHandlerready =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsready*>;
} // namespace rust::cxxqtgen1::cxx_qt::multi_object

namespace second_object {
class SecondObject;

} // namespace second_object

namespace rust::cxxqtgen1::second_object {
using SecondObjectCxxQtSignalHandlerpropertyNameChanged =
  ::rust::cxxqt1::SignalHandler<
    struct SecondObjectCxxQtSignalParamspropertyNameChanged*>;
} // namespace rust::cxxqtgen1::second_object

namespace rust::cxxqtgen1::second_object {
using SecondObjectCxxQtSignalHandlerready =
  ::rust::cxxqt1::SignalHandler<struct SecondObjectCxxQtSignalParamsready*>;
} // namespace rust::cxxqtgen1::second_object

namespace rust::cxxqtgen1 {
using QPushButtonCxxQtSignalHandlerclicked =
  ::rust::cxxqt1::SignalHandler<struct QPushButtonCxxQtSignalParamsclicked*>;
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1::mynamespace {
using ExternObjectCxxQtSignalHandlerdataReady =
  ::rust::cxxqt1::SignalHandler<struct ExternObjectCxxQtSignalParamsdataReady*>;
} // namespace rust::cxxqtgen1::mynamespace

namespace rust::cxxqtgen1::mynamespace {
using ExternObjectCxxQtSignalHandlererrorOccurred =
  ::rust::cxxqt1::SignalHandler<
    struct ExternObjectCxxQtSignalParamserrorOccurred*>;
} // namespace rust::cxxqtgen1::mynamespace

#include "cxx-qt-gen/multi_object.cxx.h"

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QPushButton_clickedConnect(
  QPushButton& self,
  ::rust::cxxqtgen1::QPushButtonCxxQtSignalHandlerclicked closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1::mynamespace {
::QMetaObject::Connection
ExternObject_dataReadyConnect(
  ::mynamespace::ExternObjectCpp& self,
  ::rust::cxxqtgen1::mynamespace::ExternObjectCxxQtSignalHandlerdataReady
    closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::mynamespace

namespace rust::cxxqtgen1::mynamespace {
::QMetaObject::Connection
ExternObject_errorOccurredConnect(
  ::mynamespace::ExternObjectCpp& self,
  ::rust::cxxqtgen1::mynamespace::ExternObjectCxxQtSignalHandlererrorOccurred
    closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::mynamespace

namespace rust::cxxqtgen1::cxx_qt::multi_object {
::QMetaObject::Connection
MyObject_propertyNameChangedConnect(
  ::cxx_qt::multi_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::multi_object::
    MyObjectCxxQtSignalHandlerpropertyNameChanged closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::cxx_qt::multi_object

namespace rust::cxxqtgen1::cxx_qt::multi_object {
::QMetaObject::Connection
MyObject_readyConnect(
  ::cxx_qt::multi_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::multi_object::MyObjectCxxQtSignalHandlerready
    closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::cxx_qt::multi_object

namespace cxx_qt::multi_object {
class MyObject
  : public QStringListModel
  , public ::rust::cxxqt1::CxxQtType<MyObjectRust>
  , public ::rust::cxxqt1::CxxQtLocking
{
  Q_OBJECT
public:
  Q_PROPERTY(::std::int32_t propertyName READ getPropertyName WRITE
               setPropertyName NOTIFY propertyNameChanged)

  virtual ~MyObject() = default;

public:
  ::std::int32_t const& getPropertyName() const;
  Q_SLOT void setPropertyName(::std::int32_t const& value);
  Q_SIGNAL void propertyNameChanged();
  Q_INVOKABLE void invokableName();
  Q_SIGNAL void ready();
  explicit MyObject(QObject* parent = nullptr);

private:
  ::std::int32_t const& getPropertyNameWrapper() const noexcept;
  void setPropertyNameWrapper(::std::int32_t value) noexcept;
  void invokableNameWrapper() noexcept;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::multi_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::MyObject*)

namespace rust::cxxqtgen1::second_object {
::QMetaObject::Connection
SecondObject_propertyNameChangedConnect(
  ::second_object::SecondObject& self,
  ::rust::cxxqtgen1::second_object::
    SecondObjectCxxQtSignalHandlerpropertyNameChanged closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::second_object

namespace rust::cxxqtgen1::second_object {
::QMetaObject::Connection
SecondObject_readyConnect(
  ::second_object::SecondObject& self,
  ::rust::cxxqtgen1::second_object::SecondObjectCxxQtSignalHandlerready closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1::second_object

namespace second_object {
class SecondObject
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<SecondObjectRust>
{
  Q_OBJECT
public:
  Q_PROPERTY(::std::int32_t propertyName READ getPropertyName WRITE
               setPropertyName NOTIFY propertyNameChanged)

  virtual ~SecondObject() = default;

public:
  ::std::int32_t const& getPropertyName() const;
  Q_SLOT void setPropertyName(::std::int32_t const& value);
  Q_SIGNAL void propertyNameChanged();
  Q_INVOKABLE void invokableName();
  Q_SIGNAL void ready();
  explicit SecondObject(QObject* parent = nullptr);

private:
  ::std::int32_t const& getPropertyNameWrapper() const noexcept;
  void setPropertyNameWrapper(::std::int32_t value) noexcept;
  void invokableNameWrapper() noexcept;
};

static_assert(::std::is_base_of<QObject, SecondObject>::value,
              "SecondObject must inherit from QObject");
} // namespace second_object

Q_DECLARE_METATYPE(second_object::SecondObject*)

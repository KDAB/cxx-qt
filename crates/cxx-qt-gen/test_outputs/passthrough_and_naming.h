#pragma once

#include <cxx-qt/signalhandler.h>
#include <cxx-qt/type.h>

namespace cxx_qt::multi_object {
class MyObject;

} // namespace cxx_qt::multi_object

namespace cxx_qt::multi_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlerpropertyNameChanged =
  ::rust::cxxqt1::SignalHandler<
    struct MyObjectCxxQtSignalParamspropertyNameChanged*>;
} // namespace cxx_qt::multi_object::rust::cxxqtgen1

namespace cxx_qt::multi_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlerready =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsready*>;
} // namespace cxx_qt::multi_object::rust::cxxqtgen1

namespace second_object {
class SecondObject;

} // namespace second_object

namespace second_object::rust::cxxqtgen1 {
using SecondObjectCxxQtSignalHandlerpropertyNameChanged =
  ::rust::cxxqt1::SignalHandler<
    struct SecondObjectCxxQtSignalParamspropertyNameChanged*>;
} // namespace second_object::rust::cxxqtgen1

namespace second_object::rust::cxxqtgen1 {
using SecondObjectCxxQtSignalHandlerready =
  ::rust::cxxqt1::SignalHandler<struct SecondObjectCxxQtSignalParamsready*>;
} // namespace second_object::rust::cxxqtgen1

namespace my_namespace {
class MyCxxName;

} // namespace my_namespace

namespace rust::cxxqtgen1 {
using QPushButtonCxxQtSignalHandlerclicked =
  ::rust::cxxqt1::SignalHandler<struct QPushButtonCxxQtSignalParamsclicked*>;
} // namespace rust::cxxqtgen1

namespace mynamespace::rust::cxxqtgen1 {
using ExternObjectCxxQtSignalHandlerdataReady =
  ::rust::cxxqt1::SignalHandler<struct ExternObjectCxxQtSignalParamsdataReady*>;
} // namespace mynamespace::rust::cxxqtgen1

namespace mynamespace::rust::cxxqtgen1 {
using ExternObjectCxxQtSignalHandlererrorOccurred =
  ::rust::cxxqt1::SignalHandler<
    struct ExternObjectCxxQtSignalParamserrorOccurred*>;
} // namespace mynamespace::rust::cxxqtgen1

#include "directory/file_ident.cxx.h"

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QPushButton_clickedConnect(
  QPushButton& self,
  ::rust::cxxqtgen1::QPushButtonCxxQtSignalHandlerclicked closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1

namespace mynamespace::rust::cxxqtgen1 {
::QMetaObject::Connection
ExternObjectCpp_dataReadyConnect(
  mynamespace::ExternObjectCpp& self,
  ::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalHandlerdataReady
    closure,
  ::Qt::ConnectionType type);
} // namespace mynamespace::rust::cxxqtgen1

namespace mynamespace::rust::cxxqtgen1 {
::QMetaObject::Connection
ExternObjectCpp_errorOccurredConnect(
  mynamespace::ExternObjectCpp& self,
  ::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalHandlererrorOccurred
    closure,
  ::Qt::ConnectionType type);
} // namespace mynamespace::rust::cxxqtgen1

namespace cxx_qt::multi_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_propertyNameChangedConnect(
  cxx_qt::multi_object::MyObject& self,
  ::cxx_qt::multi_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlerpropertyNameChanged closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::multi_object::rust::cxxqtgen1

namespace cxx_qt::multi_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_readyConnect(
  cxx_qt::multi_object::MyObject& self,
  ::cxx_qt::multi_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready
    closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::multi_object::rust::cxxqtgen1

namespace cxx_qt::multi_object {
class MyObject
  : public QStringListModel
  , public ::rust::cxxqt1::CxxQtType<MyObjectRust>
{
  Q_OBJECT
public:
  Q_PROPERTY(::std::int32_t propertyName READ getPropertyName WRITE
               setPropertyName NOTIFY propertyNameChanged)

  virtual ~MyObject() = default;

public:
  ::std::int32_t const& getPropertyName() const noexcept;
  Q_SLOT void setPropertyName(::std::int32_t value) noexcept;
  Q_SIGNAL void propertyNameChanged();
  Q_INVOKABLE void invokableName() noexcept;
  Q_SIGNAL void ready();
  explicit MyObject(QObject* parent = nullptr);
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::multi_object

Q_DECLARE_METATYPE(cxx_qt::multi_object::MyObject*)

namespace second_object::rust::cxxqtgen1 {
::QMetaObject::Connection
SecondObject_propertyNameChangedConnect(
  second_object::SecondObject& self,
  ::second_object::rust::cxxqtgen1::
    SecondObjectCxxQtSignalHandlerpropertyNameChanged closure,
  ::Qt::ConnectionType type);
} // namespace second_object::rust::cxxqtgen1

namespace second_object::rust::cxxqtgen1 {
::QMetaObject::Connection
SecondObject_readyConnect(
  second_object::SecondObject& self,
  ::second_object::rust::cxxqtgen1::SecondObjectCxxQtSignalHandlerready closure,
  ::Qt::ConnectionType type);
} // namespace second_object::rust::cxxqtgen1

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
  ::std::int32_t const& getPropertyName() const noexcept;
  Q_SLOT void setPropertyName(::std::int32_t value) noexcept;
  Q_SIGNAL void propertyNameChanged();
  Q_INVOKABLE void invokableName() noexcept;
  Q_SIGNAL void ready();
  explicit SecondObject(QObject* parent = nullptr);
};

static_assert(::std::is_base_of<QObject, SecondObject>::value,
              "SecondObject must inherit from QObject");
} // namespace second_object

Q_DECLARE_METATYPE(second_object::SecondObject*)

namespace my_namespace {
class MyCxxName
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<ThirdObjectRust>
{
  Q_OBJECT
public:
  virtual ~MyCxxName() = default;

public:
  explicit MyCxxName(QObject* parent = nullptr);
};

static_assert(::std::is_base_of<QObject, MyCxxName>::value,
              "MyCxxName must inherit from QObject");
} // namespace my_namespace

Q_DECLARE_METATYPE(my_namespace::MyCxxName*)

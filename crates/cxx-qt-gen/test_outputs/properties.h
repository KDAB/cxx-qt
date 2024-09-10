#pragma once

#include <cxx-qt/signalhandler.h>
#include <cxx-qt/type.h>

namespace cxx_qt::my_object {
class MyObject;

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlerprimitiveChanged =
  ::rust::cxxqt1::SignalHandler<
    struct MyObjectCxxQtSignalParamsprimitiveChanged*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlertrivialChanged = ::rust::cxxqt1::SignalHandler<
  struct MyObjectCxxQtSignalParamstrivialChanged*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlercustomFunctionPropChanged =
  ::rust::cxxqt1::SignalHandler<
    struct MyObjectCxxQtSignalParamscustomFunctionPropChanged*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlermyOnChanged =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsmyOnChanged*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

#include "directory/file_ident.cxx.h"

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_primitiveChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlerprimitiveChanged closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_trivialChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlertrivialChanged
    closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_customFunctionPropChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlercustomFunctionPropChanged closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_myOnChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlermyOnChanged
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
  Q_PROPERTY(::std::int32_t primitive READ getPrimitive WRITE setPrimitive
               NOTIFY primitiveChanged)
  Q_PROPERTY(
    QPoint trivial READ getTrivial WRITE setTrivial NOTIFY trivialChanged)
  Q_PROPERTY(::std::int32_t customFunctionProp READ myGetter WRITE
               MyCustomSetter NOTIFY customFunctionPropChanged)
  Q_PROPERTY(::std::int32_t readonlyProp READ getReadonlyProp)
  Q_PROPERTY(::std::int32_t customOnChangedProp READ getCustomOnChangedProp
               WRITE setCustomOnChangedProp NOTIFY myOnChanged)
  Q_PROPERTY(::std::int32_t constProp READ getConstProp CONSTANT)
  Q_PROPERTY(::std::int32_t resettableProp READ getResettableProp WRITE
               setResettableProp RESET myResetFn)
  Q_PROPERTY(::std::int32_t requiredProp READ getRequiredProp WRITE
               setRequiredProp REQUIRED)
  Q_PROPERTY(
    ::std::int32_t finalProp READ getFinalProp WRITE setFinalProp FINAL)

  virtual ~MyObject() = default;

public:
  ::std::int32_t const& getPrimitive() const noexcept;
  Q_SLOT void setPrimitive(::std::int32_t value) noexcept;
  QPoint const& getTrivial() const noexcept;
  Q_SLOT void setTrivial(QPoint value) noexcept;
  ::std::int32_t const& getReadonlyProp() const noexcept;
  ::std::int32_t const& getCustomOnChangedProp() const noexcept;
  Q_SLOT void setCustomOnChangedProp(::std::int32_t value) noexcept;
  ::std::int32_t const& getConstProp() const noexcept;
  ::std::int32_t const& getResettableProp() const noexcept;
  Q_SLOT void setResettableProp(::std::int32_t value) noexcept;
  ::std::int32_t const& getRequiredProp() const noexcept;
  Q_SLOT void setRequiredProp(::std::int32_t value) noexcept;
  ::std::int32_t const& getFinalProp() const noexcept;
  Q_SLOT void setFinalProp(::std::int32_t value) noexcept;
  Q_SIGNAL void primitiveChanged();
  Q_SIGNAL void trivialChanged();
  Q_SIGNAL void customFunctionPropChanged();
  ::std::int32_t myGetter() const noexcept;
  void MyCustomSetter(::std::int32_t value) noexcept;
  void myResetFn() noexcept;
  Q_SIGNAL void myOnChanged();
  explicit MyObject(QObject* parent = nullptr);
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)

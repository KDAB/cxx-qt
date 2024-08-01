#pragma once

#include <cxx-qt/locking.h>
#include <cxx-qt/maybelockguard.h>
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

#include "cxx-qt-gen/ffi.cxx.h"

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
  , public ::rust::cxxqt1::CxxQtLocking
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
  ::std::int32_t const& getPrimitive() const;
  Q_SLOT void setPrimitive(::std::int32_t const& value);
  QPoint const& getTrivial() const;
  Q_SLOT void setTrivial(QPoint const& value);
  ::std::int32_t const& getReadonlyProp() const;
  ::std::int32_t const& getCustomOnChangedProp() const;
  Q_SLOT void setCustomOnChangedProp(::std::int32_t const& value);
  ::std::int32_t const& getConstProp() const;
  ::std::int32_t const& getResettableProp() const;
  Q_SLOT void setResettableProp(::std::int32_t const& value);
  ::std::int32_t const& getRequiredProp() const;
  Q_SLOT void setRequiredProp(::std::int32_t const& value);
  ::std::int32_t const& getFinalProp() const;
  Q_SLOT void setFinalProp(::std::int32_t const& value);
  Q_SIGNAL void primitiveChanged();
  Q_SIGNAL void trivialChanged();
  Q_SIGNAL void customFunctionPropChanged();
  ::std::int32_t myGetter() const;
  void MyCustomSetter(::std::int32_t value);
  void myResetFn();
  Q_SIGNAL void myOnChanged();
  explicit MyObject(QObject* parent = nullptr);

private:
  ::std::int32_t const& getPrimitiveWrapper() const noexcept;
  void setPrimitiveWrapper(::std::int32_t value) noexcept;
  QPoint const& getTrivialWrapper() const noexcept;
  void setTrivialWrapper(QPoint value) noexcept;
  ::std::int32_t const& getReadonlyPropWrapper() const noexcept;
  ::std::int32_t const& getCustomOnChangedPropWrapper() const noexcept;
  void setCustomOnChangedPropWrapper(::std::int32_t value) noexcept;
  ::std::int32_t const& getConstPropWrapper() const noexcept;
  ::std::int32_t const& getResettablePropWrapper() const noexcept;
  void setResettablePropWrapper(::std::int32_t value) noexcept;
  ::std::int32_t const& getRequiredPropWrapper() const noexcept;
  void setRequiredPropWrapper(::std::int32_t value) noexcept;
  ::std::int32_t const& getFinalPropWrapper() const noexcept;
  void setFinalPropWrapper(::std::int32_t value) noexcept;
  ::std::int32_t myGetterWrapper() const noexcept;
  void MyCustomSetterWrapper(::std::int32_t value) noexcept;
  void myResetFnWrapper() noexcept;
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)

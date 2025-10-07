#pragma once

#include <cxx-qt/casting.h>
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
using MyObjectCxxQtSignalHandlerpropAutoCxxNameChanged =
  ::rust::cxxqt1::SignalHandler<
    struct MyObjectCxxQtSignalParamspropAutoCxxNameChanged*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlercustomFunctionPropChanged =
  ::rust::cxxqt1::SignalHandler<
    struct MyObjectCxxQtSignalParamscustomFunctionPropChanged*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlerrenamedPropertyChanged =
  ::rust::cxxqt1::SignalHandler<
    struct MyObjectCxxQtSignalParamsrenamedPropertyChanged*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlernamed_prop_2Changed =
  ::rust::cxxqt1::SignalHandler<
    struct MyObjectCxxQtSignalParamsnamed_prop_2Changed*>;
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
using MyObjectCxxQtSignalHandlermy_on_changed =
  ::rust::cxxqt1::SignalHandler<struct MyObjectCxxQtSignalParamsmy_on_changed*>;
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
MyObject_propAutoCxxNameChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlerpropAutoCxxNameChanged closure,
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
MyObject_renamedPropertyChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlerrenamedPropertyChanged closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_named_prop_2ChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlernamed_prop_2Changed closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_my_on_changedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlermy_on_changed
    closure,
  ::Qt::ConnectionType type);
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object {
class MyObject
  : public QObject
  , private ::rust::cxxqt1::CxxQtType<MyObjectRust>
{
  Q_OBJECT
public:
  Q_PROPERTY(::std::int32_t primitive READ getPrimitive WRITE setPrimitive
               NOTIFY primitiveChanged)
  Q_PROPERTY(
    QPoint trivial READ getTrivial WRITE setTrivial NOTIFY trivialChanged)
  Q_PROPERTY(::std::int32_t propAutoCxxName READ getPropAutoCxxName WRITE
               setPropAutoCxxName NOTIFY propAutoCxxNameChanged)
  Q_PROPERTY(::std::int32_t customFunctionProp READ myGetter WRITE
               MyCustomSetter NOTIFY customFunctionPropChanged)
  Q_PROPERTY(::std::int32_t readonlyProp READ getReadonlyProp)
  Q_PROPERTY(::std::int32_t renamedProperty READ getRenamedProperty WRITE
               setRenamedProperty NOTIFY renamedPropertyChanged)
  Q_PROPERTY(::std::int32_t reusedSignalProp READ getReusedSignalProp WRITE
               setReusedSignalProp NOTIFY trivialChanged)
  Q_PROPERTY(::std::int32_t named_prop_2 READ getNamed_prop_2 WRITE
               setNamed_prop_2 NOTIFY named_prop_2Changed)
  Q_PROPERTY(::std::int32_t customOnChangedProp READ getCustomOnChangedProp
               WRITE setCustomOnChangedProp NOTIFY my_on_changed)
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
  ::std::int32_t const& getPropAutoCxxName() const noexcept;
  Q_SLOT void setPropAutoCxxName(::std::int32_t value) noexcept;
  ::std::int32_t const& getReadonlyProp() const noexcept;
  ::std::int32_t const& getRenamedProperty() const noexcept;
  Q_SLOT void setRenamedProperty(::std::int32_t value) noexcept;
  ::std::int32_t const& getReusedSignalProp() const noexcept;
  Q_SLOT void setReusedSignalProp(::std::int32_t value) noexcept;
  ::std::int32_t const& getNamed_prop_2() const noexcept;
  Q_SLOT void setNamed_prop_2(::std::int32_t value) noexcept;
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
  Q_SIGNAL void propAutoCxxNameChanged();
  Q_SIGNAL void customFunctionPropChanged();
  Q_SIGNAL void renamedPropertyChanged();
  Q_SIGNAL void named_prop_2Changed();
  ::std::int32_t myGetter() const noexcept;
  void MyCustomSetter(::std::int32_t value) noexcept;
  void myResetFn() noexcept;
  Q_SIGNAL void my_on_changed();
  explicit MyObject(QObject* parent = nullptr);

private:
  template<typename Inner, typename Outer>
  friend Inner& ::rust::cxxqt1::unsafeRustMut(Outer& outer);

  template<typename Inner, typename Outer>
  friend const Inner& ::rust::cxxqt1::unsafeRust(const Outer& outer);
};

static_assert(::std::is_base_of<QObject, MyObject>::value,
              "MyObject must inherit from QObject");
} // namespace cxx_qt::my_object

Q_DECLARE_METATYPE(cxx_qt::my_object::MyObject*)

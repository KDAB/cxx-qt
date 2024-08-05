#include "cxx-qt-gen/ffi.cxxqt.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalParamsprimitiveChanged*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_primitiveChanged(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamsprimitiveChanged*>::
operator()<cxx_qt::my_object::MyObject&>(cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_primitiveChanged(*this, self);
}

static_assert(
  alignof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                          MyObjectCxxQtSignalParamsprimitiveChanged*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                         MyObjectCxxQtSignalParamsprimitiveChanged*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_primitiveChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlerprimitiveChanged closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::primitiveChanged,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqt1::MaybeLockGuard<cxx_qt::my_object::MyObject> guard(
        self);
      closure.template operator()<cxx_qt::my_object::MyObject&>(self);
    },
    type);
}
} // namespace cxx_qt::my_object::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalParamstrivialChanged*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_trivialChanged(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamstrivialChanged*>::
operator()<cxx_qt::my_object::MyObject&>(cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_trivialChanged(*this, self);
}

static_assert(
  alignof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                          MyObjectCxxQtSignalParamstrivialChanged*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                         MyObjectCxxQtSignalParamstrivialChanged*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_trivialChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlertrivialChanged
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::trivialChanged,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqt1::MaybeLockGuard<cxx_qt::my_object::MyObject> guard(
        self);
      closure.template operator()<cxx_qt::my_object::MyObject&>(self);
    },
    type);
}
} // namespace cxx_qt::my_object::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamscustomFunctionPropChanged*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_customFunctionPropChanged(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamscustomFunctionPropChanged*>::
operator()<cxx_qt::my_object::MyObject&>(cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_customFunctionPropChanged(*this, self);
}

static_assert(
  alignof(
    SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                    MyObjectCxxQtSignalParamscustomFunctionPropChanged*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                    MyObjectCxxQtSignalParamscustomFunctionPropChanged*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_customFunctionPropChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlercustomFunctionPropChanged closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::customFunctionPropChanged,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqt1::MaybeLockGuard<cxx_qt::my_object::MyObject> guard(
        self);
      closure.template operator()<cxx_qt::my_object::MyObject&>(self);
    },
    type);
}
} // namespace cxx_qt::my_object::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsmyOnChanged*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_myOnChanged(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsmyOnChanged*>::
operator()<cxx_qt::my_object::MyObject&>(cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_myOnChanged(*this, self);
}

static_assert(alignof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                                      MyObjectCxxQtSignalParamsmyOnChanged*>) <=
                alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                                     MyObjectCxxQtSignalParamsmyOnChanged*>) ==
                sizeof(::std::size_t[2]),
              "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_myOnChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlermyOnChanged
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::myOnChanged,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqt1::MaybeLockGuard<cxx_qt::my_object::MyObject> guard(
        self);
      closure.template operator()<cxx_qt::my_object::MyObject&>(self);
    },
    type);
}
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object {
::std::int32_t const&
MyObject::getPrimitive() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return getPrimitiveWrapper();
}

void
MyObject::setPrimitive(::std::int32_t const& value)
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  setPrimitiveWrapper(value);
}

QPoint const&
MyObject::getTrivial() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return getTrivialWrapper();
}

void
MyObject::setTrivial(QPoint const& value)
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  setTrivialWrapper(value);
}

::std::int32_t const&
MyObject::getReadonlyProp() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return getReadonlyPropWrapper();
}

::std::int32_t const&
MyObject::getCustomOnChangedProp() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return getCustomOnChangedPropWrapper();
}

void
MyObject::setCustomOnChangedProp(::std::int32_t const& value)
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  setCustomOnChangedPropWrapper(value);
}

::std::int32_t const&
MyObject::getConstProp() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return getConstPropWrapper();
}

::std::int32_t const&
MyObject::getResettableProp() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return getResettablePropWrapper();
}

void
MyObject::setResettableProp(::std::int32_t const& value)
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  setResettablePropWrapper(value);
}

::std::int32_t const&
MyObject::getRequiredProp() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return getRequiredPropWrapper();
}

void
MyObject::setRequiredProp(::std::int32_t const& value)
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  setRequiredPropWrapper(value);
}

::std::int32_t const&
MyObject::getFinalProp() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return getFinalPropWrapper();
}

void
MyObject::setFinalProp(::std::int32_t const& value)
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  setFinalPropWrapper(value);
}

::std::int32_t
MyObject::myGetter() const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  return myGetterWrapper();
}

void
MyObject::MyCustomSetter(::std::int32_t value)
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  MyCustomSetterWrapper(value);
}

void
MyObject::myResetFn()
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  myResetFnWrapper();
}

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::createRs())
  , ::rust::cxxqt1::CxxQtLocking()
{
}

} // namespace cxx_qt::my_object

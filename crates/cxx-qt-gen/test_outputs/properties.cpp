#include "cxx-qt-gen/properties.cxxqt.h"

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
      closure.template operator()<cxx_qt::my_object::MyObject&>(self);
    },
    type);
}
} // namespace cxx_qt::my_object::rust::cxxqtgen1

namespace cxx_qt::my_object {
MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::createRs())
{
}

} // namespace cxx_qt::my_object

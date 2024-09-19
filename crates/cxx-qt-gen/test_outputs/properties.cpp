#include "directory/file_ident.cxxqt.h"

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
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalParamsrenamedPropertyChanged*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_renamedPropertyChanged(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamsrenamedPropertyChanged*>::
operator()<cxx_qt::my_object::MyObject&>(cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_renamedPropertyChanged(*this, self);
}

static_assert(
  alignof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                          MyObjectCxxQtSignalParamsrenamedPropertyChanged*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                         MyObjectCxxQtSignalParamsrenamedPropertyChanged*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_renamedPropertyChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlerrenamedPropertyChanged closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::renamedPropertyChanged,
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
    MyObjectCxxQtSignalParamsnamed_prop_2Changed*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_named_prop_2Changed(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamsnamed_prop_2Changed*>::
operator()<cxx_qt::my_object::MyObject&>(cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_named_prop_2Changed(*this, self);
}

static_assert(
  alignof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                          MyObjectCxxQtSignalParamsnamed_prop_2Changed*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                         MyObjectCxxQtSignalParamsnamed_prop_2Changed*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_named_prop_2ChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlernamed_prop_2Changed closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::named_prop_2Changed,
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
    MyObjectCxxQtSignalParamsmy_on_changed*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_my_on_changed(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamsmy_on_changed*>::
operator()<cxx_qt::my_object::MyObject&>(cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_my_on_changed(*this, self);
}

static_assert(
  alignof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                          MyObjectCxxQtSignalParamsmy_on_changed*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                         MyObjectCxxQtSignalParamsmy_on_changed*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_my_on_changedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlermy_on_changed
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::my_on_changed,
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
      ::cxx_qt::my_object::cxx_qt_MyObject::createRs())
{
}

} // namespace cxx_qt::my_object

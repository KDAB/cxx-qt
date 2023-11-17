#include "cxx-qt-gen/multi_object.cxxqt.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<::rust::cxxqtgen1::QPushButtonCxxQtSignalParamsclicked*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_QPushButton_signal_handler_clicked(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::rust::cxxqtgen1::QPushButtonCxxQtSignalParamsclicked*>::
operator()<QPushButton&, bool>(QPushButton& self, bool checked)
{
  call_QPushButton_signal_handler_clicked(*this, self, ::std::move(checked));
}

static_assert(
  alignof(
    SignalHandler<::rust::cxxqtgen1::QPushButtonCxxQtSignalParamsclicked*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<::rust::cxxqtgen1::QPushButtonCxxQtSignalParamsclicked*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QPushButton_clickedConnect(
  QPushButton& self,
  ::rust::cxxqtgen1::QPushButtonCxxQtSignalHandlerclicked closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &QPushButton::clicked,
    &self,
    [&, closure = ::std::move(closure)](bool checked) mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<QPushButton> guard(self);
      closure.template operator()<QPushButton&, bool>(self,
                                                      ::std::move(checked));
    },
    type);
}
} // namespace rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<
  ::rust::cxxqtgen1::mynamespace::ExternObjectCxxQtSignalParamsdataReady*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_ExternObject_signal_handler_dataReady(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::rust::cxxqtgen1::mynamespace::ExternObjectCxxQtSignalParamsdataReady*>::
operator()<::mynamespace::ExternObjectCpp&>(
  ::mynamespace::ExternObjectCpp& self)
{
  call_ExternObject_signal_handler_dataReady(*this, self);
}

static_assert(
  alignof(SignalHandler<::rust::cxxqtgen1::mynamespace::
                          ExternObjectCxxQtSignalParamsdataReady*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::rust::cxxqtgen1::mynamespace::
                         ExternObjectCxxQtSignalParamsdataReady*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::mynamespace {
::QMetaObject::Connection
ExternObject_dataReadyConnect(
  ::mynamespace::ExternObjectCpp& self,
  ::rust::cxxqtgen1::mynamespace::ExternObjectCxxQtSignalHandlerdataReady
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::mynamespace::ExternObjectCpp::dataReady,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::mynamespace::ExternObjectCpp>
        guard(self);
      closure.template operator()<::mynamespace::ExternObjectCpp&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1::mynamespace

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<
  ::rust::cxxqtgen1::mynamespace::ExternObjectCxxQtSignalParamserrorOccurred*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_ExternObject_signal_handler_errorOccurred(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::rust::cxxqtgen1::mynamespace::ExternObjectCxxQtSignalParamserrorOccurred*>::
operator()<::mynamespace::ExternObjectCpp&>(
  ::mynamespace::ExternObjectCpp& self)
{
  call_ExternObject_signal_handler_errorOccurred(*this, self);
}

static_assert(
  alignof(SignalHandler<::rust::cxxqtgen1::mynamespace::
                          ExternObjectCxxQtSignalParamserrorOccurred*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::rust::cxxqtgen1::mynamespace::
                         ExternObjectCxxQtSignalParamserrorOccurred*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::mynamespace {
::QMetaObject::Connection
ExternObject_errorOccurredConnect(
  ::mynamespace::ExternObjectCpp& self,
  ::rust::cxxqtgen1::mynamespace::ExternObjectCxxQtSignalHandlererrorOccurred
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::mynamespace::ExternObjectCpp::errorOccurred,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::mynamespace::ExternObjectCpp>
        guard(self);
      closure.template operator()<::mynamespace::ExternObjectCpp&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1::mynamespace

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<
  ::rust::cxxqtgen1::cxx_qt::multi_object::
    MyObjectCxxQtSignalParamspropertyNameChanged*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_propertyNameChanged(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::rust::cxxqtgen1::cxx_qt::multi_object::
                MyObjectCxxQtSignalParamspropertyNameChanged*>::
operator()<::cxx_qt::multi_object::MyObject&>(
  ::cxx_qt::multi_object::MyObject& self)
{
  call_MyObject_signal_handler_propertyNameChanged(*this, self);
}

static_assert(
  alignof(SignalHandler<::rust::cxxqtgen1::cxx_qt::multi_object::
                          MyObjectCxxQtSignalParamspropertyNameChanged*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::rust::cxxqtgen1::cxx_qt::multi_object::
                         MyObjectCxxQtSignalParamspropertyNameChanged*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::cxx_qt::multi_object {
::QMetaObject::Connection
MyObject_propertyNameChangedConnect(
  ::cxx_qt::multi_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::multi_object::
    MyObjectCxxQtSignalHandlerpropertyNameChanged closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::cxx_qt::multi_object::MyObject::propertyNameChanged,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::cxx_qt::multi_object::MyObject>
        guard(self);
      closure.template operator()<::cxx_qt::multi_object::MyObject&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1::cxx_qt::multi_object

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<::rust::cxxqtgen1::cxx_qt::multi_object::
                MyObjectCxxQtSignalParamsready*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_ready(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::rust::cxxqtgen1::cxx_qt::multi_object::MyObjectCxxQtSignalParamsready*>::
operator()<::cxx_qt::multi_object::MyObject&>(
  ::cxx_qt::multi_object::MyObject& self)
{
  call_MyObject_signal_handler_ready(*this, self);
}

static_assert(alignof(SignalHandler<::rust::cxxqtgen1::cxx_qt::multi_object::
                                      MyObjectCxxQtSignalParamsready*>) <=
                alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::cxx_qt::multi_object::
                                     MyObjectCxxQtSignalParamsready*>) ==
                sizeof(::std::size_t[2]),
              "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::cxx_qt::multi_object {
::QMetaObject::Connection
MyObject_readyConnect(
  ::cxx_qt::multi_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::multi_object::MyObjectCxxQtSignalHandlerready
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::cxx_qt::multi_object::MyObject::ready,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::cxx_qt::multi_object::MyObject>
        guard(self);
      closure.template operator()<::cxx_qt::multi_object::MyObject&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1::cxx_qt::multi_object

namespace cxx_qt::multi_object {
::std::int32_t const&
MyObject::getPropertyName() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return getPropertyNameWrapper();
}

void
MyObject::setPropertyName(::std::int32_t const& value)
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  setPropertyNameWrapper(value);
}

void
MyObject::invokableName()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableNameWrapper();
}

MyObject::MyObject(QObject* parent)
  : QStringListModel(parent)
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::multi_object::cxx_qt_my_object::createRs())
  , ::rust::cxxqtlib1::CxxQtLocking()
{
}

} // namespace cxx_qt::multi_object

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<::rust::cxxqtgen1::second_object::
                SecondObjectCxxQtSignalParamspropertyNameChanged*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_SecondObject_signal_handler_propertyNameChanged(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::rust::cxxqtgen1::second_object::
                SecondObjectCxxQtSignalParamspropertyNameChanged*>::
operator()<::second_object::SecondObject&>(::second_object::SecondObject& self)
{
  call_SecondObject_signal_handler_propertyNameChanged(*this, self);
}

static_assert(
  alignof(SignalHandler<::rust::cxxqtgen1::second_object::
                          SecondObjectCxxQtSignalParamspropertyNameChanged*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::rust::cxxqtgen1::second_object::
                         SecondObjectCxxQtSignalParamspropertyNameChanged*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::second_object {
::QMetaObject::Connection
SecondObject_propertyNameChangedConnect(
  ::second_object::SecondObject& self,
  ::rust::cxxqtgen1::second_object::
    SecondObjectCxxQtSignalHandlerpropertyNameChanged closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::second_object::SecondObject::propertyNameChanged,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::second_object::SecondObject>
        guard(self);
      closure.template operator()<::second_object::SecondObject&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1::second_object

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<::rust::cxxqtgen1::second_object::
                SecondObjectCxxQtSignalParamsready*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_SecondObject_signal_handler_ready(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::rust::cxxqtgen1::second_object::SecondObjectCxxQtSignalParamsready*>::
operator()<::second_object::SecondObject&>(::second_object::SecondObject& self)
{
  call_SecondObject_signal_handler_ready(*this, self);
}

static_assert(
  alignof(
    SignalHandler<
      ::rust::cxxqtgen1::second_object::SecondObjectCxxQtSignalParamsready*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<
      ::rust::cxxqtgen1::second_object::SecondObjectCxxQtSignalParamsready*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::second_object {
::QMetaObject::Connection
SecondObject_readyConnect(
  ::second_object::SecondObject& self,
  ::rust::cxxqtgen1::second_object::SecondObjectCxxQtSignalHandlerready closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::second_object::SecondObject::ready,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::second_object::SecondObject>
        guard(self);
      closure.template operator()<::second_object::SecondObject&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1::second_object

namespace second_object {
::std::int32_t const&
SecondObject::getPropertyName() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<SecondObject> guard(*this);
  return getPropertyNameWrapper();
}

void
SecondObject::setPropertyName(::std::int32_t const& value)
{
  const ::rust::cxxqtlib1::MaybeLockGuard<SecondObject> guard(*this);
  setPropertyNameWrapper(value);
}

void
SecondObject::invokableName()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<SecondObject> guard(*this);
  invokableNameWrapper();
}

SecondObject::SecondObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqtlib1::CxxQtType<SecondObjectRust>(
      ::second_object::cxx_qt_second_object::createRs())
{
}

} // namespace second_object

#include "directory/file_ident.cxxqt.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                QPushButtonCxxQtSignalParamsclicked*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_QPushButton_signal_handler_clicked(::std::move(*this));
}

template<>
template<>
void
SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                QPushButtonCxxQtSignalParamsclicked*>::
operator()<cxx_qt::multi_object::QPushButton&, bool>(
  cxx_qt::multi_object::QPushButton& self,
  bool checked)
{
  call_QPushButton_signal_handler_clicked(*this, self, ::std::move(checked));
}

static_assert(alignof(SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                                      QPushButtonCxxQtSignalParamsclicked*>) <=
                alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                                     QPushButtonCxxQtSignalParamsclicked*>) ==
                sizeof(::std::size_t[2]),
              "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::multi_object::rust::cxxqtgen1 {
::QMetaObject::Connection
QPushButton_clickedConnect(
  cxx_qt::multi_object::QPushButton& self,
  ::cxx_qt::multi_object::rust::cxxqtgen1::QPushButtonCxxQtSignalHandlerclicked
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::multi_object::QPushButton::clicked,
    &self,
    [&, closure = ::std::move(closure)](bool checked) mutable {
      closure.template operator()<cxx_qt::multi_object::QPushButton&, bool>(
        self, ::std::move(checked));
    },
    type);
}
} // namespace cxx_qt::multi_object::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalParamsdataReady*>::
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
  ::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalParamsdataReady*>::
operator()<mynamespace::ExternObjectCpp&>(mynamespace::ExternObjectCpp& self)
{
  call_ExternObject_signal_handler_dataReady(*this, self);
}

static_assert(
  alignof(SignalHandler<::mynamespace::rust::cxxqtgen1::
                          ExternObjectCxxQtSignalParamsdataReady*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::mynamespace::rust::cxxqtgen1::
                         ExternObjectCxxQtSignalParamsdataReady*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace mynamespace::rust::cxxqtgen1 {
::QMetaObject::Connection
ExternObjectCpp_dataReadyConnect(
  mynamespace::ExternObjectCpp& self,
  ::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalHandlerdataReady
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &mynamespace::ExternObjectCpp::dataReady,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<mynamespace::ExternObjectCpp&>(self);
    },
    type);
}
} // namespace mynamespace::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalParamserrorOccurred*>::
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
  ::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalParamserrorOccurred*>::
operator()<mynamespace::ExternObjectCpp&>(mynamespace::ExternObjectCpp& self)
{
  call_ExternObject_signal_handler_errorOccurred(*this, self);
}

static_assert(
  alignof(SignalHandler<::mynamespace::rust::cxxqtgen1::
                          ExternObjectCxxQtSignalParamserrorOccurred*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::mynamespace::rust::cxxqtgen1::
                         ExternObjectCxxQtSignalParamserrorOccurred*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace mynamespace::rust::cxxqtgen1 {
::QMetaObject::Connection
ExternObjectCpp_errorOccurredConnect(
  mynamespace::ExternObjectCpp& self,
  ::mynamespace::rust::cxxqtgen1::ExternObjectCxxQtSignalHandlererrorOccurred
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &mynamespace::ExternObjectCpp::errorOccurred,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<mynamespace::ExternObjectCpp&>(self);
    },
    type);
}
} // namespace mynamespace::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::cxx_qt::multi_object::rust::cxxqtgen1::
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
SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamspropertyNameChanged*>::
operator()<cxx_qt::multi_object::MyObject&>(
  cxx_qt::multi_object::MyObject& self)
{
  call_MyObject_signal_handler_propertyNameChanged(*this, self);
}

static_assert(
  alignof(SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                          MyObjectCxxQtSignalParamspropertyNameChanged*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                         MyObjectCxxQtSignalParamspropertyNameChanged*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::multi_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_propertyNameChangedConnect(
  cxx_qt::multi_object::MyObject& self,
  ::cxx_qt::multi_object::rust::cxxqtgen1::
    MyObjectCxxQtSignalHandlerpropertyNameChanged closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::multi_object::MyObject::propertyNameChanged,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<cxx_qt::multi_object::MyObject&>(self);
    },
    type);
}
} // namespace cxx_qt::multi_object::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
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
  ::cxx_qt::multi_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsready*>::
operator()<cxx_qt::multi_object::MyObject&>(
  cxx_qt::multi_object::MyObject& self)
{
  call_MyObject_signal_handler_ready(*this, self);
}

static_assert(alignof(SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                                      MyObjectCxxQtSignalParamsready*>) <=
                alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(SignalHandler<::cxx_qt::multi_object::rust::cxxqtgen1::
                                     MyObjectCxxQtSignalParamsready*>) ==
                sizeof(::std::size_t[2]),
              "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::multi_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_readyConnect(
  cxx_qt::multi_object::MyObject& self,
  ::cxx_qt::multi_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::multi_object::MyObject::ready,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<cxx_qt::multi_object::MyObject&>(self);
    },
    type);
}
} // namespace cxx_qt::multi_object::rust::cxxqtgen1

namespace cxx_qt::multi_object {
MyObject::MyObject(QObject* parent)
  : QStringListModel(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(
      ::cxx_qt::multi_object::cxx_qt_my_object::createRs())
{
}

} // namespace cxx_qt::multi_object

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<::second_object::rust::cxxqtgen1::
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
SignalHandler<::second_object::rust::cxxqtgen1::
                SecondObjectCxxQtSignalParamspropertyNameChanged*>::
operator()<second_object::SecondObject&>(second_object::SecondObject& self)
{
  call_SecondObject_signal_handler_propertyNameChanged(*this, self);
}

static_assert(
  alignof(SignalHandler<::second_object::rust::cxxqtgen1::
                          SecondObjectCxxQtSignalParamspropertyNameChanged*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<::second_object::rust::cxxqtgen1::
                         SecondObjectCxxQtSignalParamspropertyNameChanged*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace second_object::rust::cxxqtgen1 {
::QMetaObject::Connection
SecondObject_propertyNameChangedConnect(
  second_object::SecondObject& self,
  ::second_object::rust::cxxqtgen1::
    SecondObjectCxxQtSignalHandlerpropertyNameChanged closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &second_object::SecondObject::propertyNameChanged,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<second_object::SecondObject&>(self);
    },
    type);
}
} // namespace second_object::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<::second_object::rust::cxxqtgen1::
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
  ::second_object::rust::cxxqtgen1::SecondObjectCxxQtSignalParamsready*>::
operator()<second_object::SecondObject&>(second_object::SecondObject& self)
{
  call_SecondObject_signal_handler_ready(*this, self);
}

static_assert(
  alignof(
    SignalHandler<
      ::second_object::rust::cxxqtgen1::SecondObjectCxxQtSignalParamsready*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<
      ::second_object::rust::cxxqtgen1::SecondObjectCxxQtSignalParamsready*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace second_object::rust::cxxqtgen1 {
::QMetaObject::Connection
SecondObject_readyConnect(
  second_object::SecondObject& self,
  ::second_object::rust::cxxqtgen1::SecondObjectCxxQtSignalHandlerready closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &second_object::SecondObject::ready,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<second_object::SecondObject&>(self);
    },
    type);
}
} // namespace second_object::rust::cxxqtgen1

namespace second_object {
SecondObject::SecondObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<SecondObjectRust>(
      ::second_object::cxx_qt_second_object::createRs())
{
}

} // namespace second_object

namespace my_namespace {
MyCxxName::MyCxxName(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<ThirdObjectRust>(
      ::my_namespace::cxx_qt_my_rust_name::createRs())
{
}

} // namespace my_namespace

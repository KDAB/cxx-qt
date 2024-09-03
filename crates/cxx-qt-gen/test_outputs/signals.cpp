#include "cxx-qt-gen/ffi.cxxqt.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                QTimerCxxQtSignalParamstimeout*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_QTimer_signal_handler_timeout(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::cxx_qt::my_object::rust::cxxqtgen1::QTimerCxxQtSignalParamstimeout*>::
operator()<cxx_qt::my_object::QTimer&>(cxx_qt::my_object::QTimer& self)
{
  call_QTimer_signal_handler_timeout(*this, self);
}

static_assert(
  alignof(
    SignalHandler<
      ::cxx_qt::my_object::rust::cxxqtgen1::QTimerCxxQtSignalParamstimeout*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<
      ::cxx_qt::my_object::rust::cxxqtgen1::QTimerCxxQtSignalParamstimeout*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
QTimer_timeoutConnect(
  cxx_qt::my_object::QTimer& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::QTimerCxxQtSignalHandlertimeout closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::QTimer::timeout,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<cxx_qt::my_object::QTimer&>(self);
    },
    type);
}
} // namespace cxx_qt::my_object::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
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
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsready*>::
operator()<cxx_qt::my_object::MyObject&>(cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_ready(*this, self);
}

static_assert(
  alignof(
    SignalHandler<
      ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsready*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<
      ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsready*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_readyConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerready closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::ready,
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
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_dataChanged(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsdataChanged*>::
operator()<cxx_qt::my_object::MyObject&,
           ::std::int32_t,
           ::std::unique_ptr<Opaque>,
           QPoint,
           QPoint const&>(cxx_qt::my_object::MyObject& self,
                          ::std::int32_t first,
                          ::std::unique_ptr<Opaque> second,
                          QPoint third,
                          QPoint const& fourth)
{
  call_MyObject_signal_handler_dataChanged(*this,
                                           self,
                                           ::std::move(first),
                                           ::std::move(second),
                                           ::std::move(third),
                                           ::std::move(fourth));
}

static_assert(alignof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                                      MyObjectCxxQtSignalParamsdataChanged*>) <=
                alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                                     MyObjectCxxQtSignalParamsdataChanged*>) ==
                sizeof(::std::size_t[2]),
              "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_dataChangedConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlerdataChanged
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::dataChanged,
    &self,
    [&, closure = ::std::move(closure)](::std::int32_t first,
                                        ::std::unique_ptr<Opaque> second,
                                        QPoint third,
                                        QPoint const& fourth) mutable {
      closure.template operator()<cxx_qt::my_object::MyObject&,
                                  ::std::int32_t,
                                  ::std::unique_ptr<Opaque>,
                                  QPoint,
                                  QPoint const&>(self,
                                                 ::std::move(first),
                                                 ::std::move(second),
                                                 ::std::move(third),
                                                 ::std::move(fourth));
    },
    type);
}
} // namespace cxx_qt::my_object::rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                MyObjectCxxQtSignalParamsnewData*>::~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_MyObject_signal_handler_newData(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalParamsnewData*>::
operator()<cxx_qt::my_object::MyObject&,
           ::std::int32_t,
           ::std::unique_ptr<Opaque>,
           QPoint,
           QPoint const&>(cxx_qt::my_object::MyObject& self,
                          ::std::int32_t first,
                          ::std::unique_ptr<Opaque> second,
                          QPoint third,
                          QPoint const& fourth)
{
  call_MyObject_signal_handler_newData(*this,
                                       self,
                                       ::std::move(first),
                                       ::std::move(second),
                                       ::std::move(third),
                                       ::std::move(fourth));
}

static_assert(alignof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                                      MyObjectCxxQtSignalParamsnewData*>) <=
                alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(SignalHandler<::cxx_qt::my_object::rust::cxxqtgen1::
                                     MyObjectCxxQtSignalParamsnewData*>) ==
                sizeof(::std::size_t[2]),
              "unexpected size");
} // namespace rust::cxxqt1

namespace cxx_qt::my_object::rust::cxxqtgen1 {
::QMetaObject::Connection
MyObject_newDataConnect(
  cxx_qt::my_object::MyObject& self,
  ::cxx_qt::my_object::rust::cxxqtgen1::MyObjectCxxQtSignalHandlernewData
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &cxx_qt::my_object::MyObject::newData,
    &self,
    [&, closure = ::std::move(closure)](::std::int32_t first,
                                        ::std::unique_ptr<Opaque> second,
                                        QPoint third,
                                        QPoint const& fourth) mutable {
      closure.template operator()<cxx_qt::my_object::MyObject&,
                                  ::std::int32_t,
                                  ::std::unique_ptr<Opaque>,
                                  QPoint,
                                  QPoint const&>(self,
                                                 ::std::move(first),
                                                 ::std::move(second),
                                                 ::std::move(third),
                                                 ::std::move(fourth));
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

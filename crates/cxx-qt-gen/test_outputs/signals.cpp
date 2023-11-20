#include "cxx-qt-gen/ffi.cxxqt.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<::rust::cxxqtgen1::cxx_qt::my_object::
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
  ::rust::cxxqtgen1::cxx_qt::my_object::QTimerCxxQtSignalParamstimeout*>::
operator()<::cxx_qt::my_object::QTimer&>(::cxx_qt::my_object::QTimer& self)
{
  call_QTimer_signal_handler_timeout(*this, self);
}

static_assert(
  alignof(
    SignalHandler<
      ::rust::cxxqtgen1::cxx_qt::my_object::QTimerCxxQtSignalParamstimeout*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<
      ::rust::cxxqtgen1::cxx_qt::my_object::QTimerCxxQtSignalParamstimeout*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
QTimer_timeoutConnect(
  ::cxx_qt::my_object::QTimer& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::QTimerCxxQtSignalHandlertimeout closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::cxx_qt::my_object::QTimer::timeout,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::cxx_qt::my_object::QTimer>
        guard(self);
      closure.template operator()<::cxx_qt::my_object::QTimer&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1::cxx_qt::my_object

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<::rust::cxxqtgen1::cxx_qt::my_object::
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
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalParamsready*>::
operator()<::cxx_qt::my_object::MyObject&>(::cxx_qt::my_object::MyObject& self)
{
  call_MyObject_signal_handler_ready(*this, self);
}

static_assert(
  alignof(
    SignalHandler<
      ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalParamsready*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<
      ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalParamsready*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
MyObject_readyConnect(
  ::cxx_qt::my_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlerready closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::cxx_qt::my_object::MyObject::ready,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::cxx_qt::my_object::MyObject>
        guard(self);
      closure.template operator()<::cxx_qt::my_object::MyObject&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1::cxx_qt::my_object

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalParamsdataChanged*>::
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
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalParamsdataChanged*>::
operator()<::cxx_qt::my_object::MyObject&,
           ::std::int32_t,
           ::std::unique_ptr<Opaque>,
           QPoint,
           QPoint const&>(::cxx_qt::my_object::MyObject& self,
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

static_assert(alignof(SignalHandler<::rust::cxxqtgen1::cxx_qt::my_object::
                                      MyObjectCxxQtSignalParamsdataChanged*>) <=
                alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::cxx_qt::my_object::
                                     MyObjectCxxQtSignalParamsdataChanged*>) ==
                sizeof(::std::size_t[2]),
              "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
MyObject_dataChangedConnect(
  ::cxx_qt::my_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlerdataChanged
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::cxx_qt::my_object::MyObject::dataChanged,
    &self,
    [&, closure = ::std::move(closure)](::std::int32_t first,
                                        ::std::unique_ptr<Opaque> second,
                                        QPoint third,
                                        QPoint const& fourth) mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::cxx_qt::my_object::MyObject>
        guard(self);
      closure.template operator()<::cxx_qt::my_object::MyObject&,
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
} // namespace rust::cxxqtgen1::cxx_qt::my_object

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqtlib1 {
template<>
SignalHandler<::rust::cxxqtgen1::cxx_qt::my_object::
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
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalParamsnewData*>::
operator()<::cxx_qt::my_object::MyObject&,
           ::std::int32_t,
           ::std::unique_ptr<Opaque>,
           QPoint,
           QPoint const&>(::cxx_qt::my_object::MyObject& self,
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

static_assert(alignof(SignalHandler<::rust::cxxqtgen1::cxx_qt::my_object::
                                      MyObjectCxxQtSignalParamsnewData*>) <=
                alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(SignalHandler<::rust::cxxqtgen1::cxx_qt::my_object::
                                     MyObjectCxxQtSignalParamsnewData*>) ==
                sizeof(::std::size_t[2]),
              "unexpected size");
} // namespace rust::cxxqtlib1

namespace rust::cxxqtgen1::cxx_qt::my_object {
::QMetaObject::Connection
MyObject_newDataConnect(
  ::cxx_qt::my_object::MyObject& self,
  ::rust::cxxqtgen1::cxx_qt::my_object::MyObjectCxxQtSignalHandlernewData
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &::cxx_qt::my_object::MyObject::newData,
    &self,
    [&, closure = ::std::move(closure)](::std::int32_t first,
                                        ::std::unique_ptr<Opaque> second,
                                        QPoint third,
                                        QPoint const& fourth) mutable {
      const ::rust::cxxqtlib1::MaybeLockGuard<::cxx_qt::my_object::MyObject>
        guard(self);
      closure.template operator()<::cxx_qt::my_object::MyObject&,
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
} // namespace rust::cxxqtgen1::cxx_qt::my_object

namespace cxx_qt::my_object {
void
MyObject::invokable()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableWrapper();
}

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::createRs())
  , ::rust::cxxqtlib1::CxxQtLocking()
{
}

} // namespace cxx_qt::my_object

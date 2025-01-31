#include "directory/file_ident.cxxqt.h"

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::rust::cxxqtgen1::QObjectExternEnabledCxxQtSignalParamssignal_enabled1*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_QObjectExternEnabled_signal_handler_signal_enabled1(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::rust::cxxqtgen1::QObjectExternEnabledCxxQtSignalParamssignal_enabled1*>::
operator()<QObjectExternEnabled&>(QObjectExternEnabled& self)
{
  call_QObjectExternEnabled_signal_handler_signal_enabled1(*this, self);
}

static_assert(
  alignof(
    SignalHandler<::rust::cxxqtgen1::
                    QObjectExternEnabledCxxQtSignalParamssignal_enabled1*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<::rust::cxxqtgen1::
                    QObjectExternEnabledCxxQtSignalParamssignal_enabled1*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QObjectExternEnabled_signal_enabled1Connect(
  QObjectExternEnabled& self,
  ::rust::cxxqtgen1::QObjectExternEnabledCxxQtSignalHandlersignal_enabled1
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &QObjectExternEnabled::signal_enabled1,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<QObjectExternEnabled&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::rust::cxxqtgen1::QObjectExternDisabledCxxQtSignalParamssignal_enabled2*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_QObjectExternDisabled_signal_handler_signal_enabled2(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::rust::cxxqtgen1::QObjectExternDisabledCxxQtSignalParamssignal_enabled2*>::
operator()<QObjectExternDisabled&>(QObjectExternDisabled& self)
{
  call_QObjectExternDisabled_signal_handler_signal_enabled2(*this, self);
}

static_assert(
  alignof(
    SignalHandler<::rust::cxxqtgen1::
                    QObjectExternDisabledCxxQtSignalParamssignal_enabled2*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(
    SignalHandler<::rust::cxxqtgen1::
                    QObjectExternDisabledCxxQtSignalParamssignal_enabled2*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QObjectExternDisabled_signal_enabled2Connect(
  QObjectExternDisabled& self,
  ::rust::cxxqtgen1::QObjectExternDisabledCxxQtSignalHandlersignal_enabled2
    closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &QObjectExternDisabled::signal_enabled2,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<QObjectExternDisabled&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1

// Define namespace otherwise we hit a GCC bug
// https://gcc.gnu.org/bugzilla/show_bug.cgi?id=56480
namespace rust::cxxqt1 {
template<>
SignalHandler<
  ::rust::cxxqtgen1::QObjectEnabledCxxQtSignalParamssignal_enabled*>::
  ~SignalHandler() noexcept
{
  if (data[0] == nullptr && data[1] == nullptr) {
    return;
  }

  drop_QObjectEnabled_signal_handler_signal_enabled(::std::move(*this));
}

template<>
template<>
void
SignalHandler<
  ::rust::cxxqtgen1::QObjectEnabledCxxQtSignalParamssignal_enabled*>::
operator()<QObjectEnabled&>(QObjectEnabled& self)
{
  call_QObjectEnabled_signal_handler_signal_enabled(*this, self);
}

static_assert(
  alignof(SignalHandler<
          ::rust::cxxqtgen1::QObjectEnabledCxxQtSignalParamssignal_enabled*>) <=
    alignof(::std::size_t),
  "unexpected aligment");
static_assert(
  sizeof(SignalHandler<
         ::rust::cxxqtgen1::QObjectEnabledCxxQtSignalParamssignal_enabled*>) ==
    sizeof(::std::size_t[2]),
  "unexpected size");
} // namespace rust::cxxqt1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QObjectEnabled_signal_enabledConnect(
  QObjectEnabled& self,
  ::rust::cxxqtgen1::QObjectEnabledCxxQtSignalHandlersignal_enabled closure,
  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    &self,
    &QObjectEnabled::signal_enabled,
    &self,
    [&, closure = ::std::move(closure)]() mutable {
      closure.template operator()<QObjectEnabled&>(self);
    },
    type);
}
} // namespace rust::cxxqtgen1

QObjectEnabled::QObjectEnabled(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<QObjectEnabledRust>(
      ::cxx_qt_QObjectEnabled::createRs())
{
}

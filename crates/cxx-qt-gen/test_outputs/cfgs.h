#pragma once

#include <cstdint>
#include <cxx-qt/include/casting.h>
#include <cxx-qt/include/signalhandler.h>
#include <cxx-qt/include/type.h>

class QObjectEnabled;

namespace rust::cxxqtgen1 {
using QObjectEnabledCxxQtSignalHandlersignal_enabled =
  ::rust::cxxqt1::SignalHandler<
    struct QObjectEnabledCxxQtSignalParamssignal_enabled*>;
} // namespace rust::cxxqtgen1

enum class EnumEnabled1 : ::std::int32_t
{
  A
};

enum class EnumEnabled2 : ::std::int32_t
{
  A
};

namespace rust::cxxqtgen1 {
using QObjectExternEnabledCxxQtSignalHandlersignal_enabled1 =
  ::rust::cxxqt1::SignalHandler<
    struct QObjectExternEnabledCxxQtSignalParamssignal_enabled1*>;
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1 {
using QObjectExternDisabledCxxQtSignalHandlersignal_enabled2 =
  ::rust::cxxqt1::SignalHandler<
    struct QObjectExternDisabledCxxQtSignalParamssignal_enabled2*>;
} // namespace rust::cxxqtgen1

#include "directory/file_ident.cxx.h"

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QObjectExternEnabled_signal_enabled1Connect(
  QObjectExternEnabled& self,
  ::rust::cxxqtgen1::QObjectExternEnabledCxxQtSignalHandlersignal_enabled1
    closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QObjectExternDisabled_signal_enabled2Connect(
  QObjectExternDisabled& self,
  ::rust::cxxqtgen1::QObjectExternDisabledCxxQtSignalHandlersignal_enabled2
    closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1

namespace rust::cxxqtgen1 {
::QMetaObject::Connection
QObjectEnabled_signal_enabledConnect(
  QObjectEnabled& self,
  ::rust::cxxqtgen1::QObjectEnabledCxxQtSignalHandlersignal_enabled closure,
  ::Qt::ConnectionType type);
} // namespace rust::cxxqtgen1

class QObjectEnabled
  : public QObject
  , public ::rust::cxxqt1::CxxQtType<QObjectEnabledRust>
{
  Q_OBJECT
public:
#ifdef Q_MOC_RUN
  enum class EnumEnabled1 : ::std::int32_t{ A };
  Q_ENUM(EnumEnabled1)
#else
  using EnumEnabled1 = ::EnumEnabled1;
  Q_ENUM(EnumEnabled1)
#endif

  virtual ~QObjectEnabled() = default;

public:
  Q_INVOKABLE void invokable_enabled() const noexcept;
  Q_SIGNAL void signal_enabled();
  template<class... Args>
  void inherit_enabledCxxQtInherit(Args... args) const
  {
    return QObject::inherit_enabled(args...);
  }
  explicit QObjectEnabled(QObject* parent = nullptr);
};

static_assert(::std::is_base_of<QObject, QObjectEnabled>::value,
              "QObjectEnabled must inherit from QObject");

Q_DECLARE_METATYPE(QObjectEnabled*)

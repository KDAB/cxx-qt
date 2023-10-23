#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {
void
MyObject::invokable()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableWrapper();
}

::QMetaObject::Connection
MyObject::readyConnect(::rust::Fn<void(MyObject&)> func,
                       ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::ready,
    this,
    [&, func = ::std::move(func)]() {
      const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
      func(*this);
    },
    type);
}

::QMetaObject::Connection
MyObject::dataChangedConnect(::rust::Fn<void(MyObject&,
                                             ::std::int32_t first,
                                             ::std::unique_ptr<Opaque> second,
                                             QPoint third,
                                             QPoint const& fourth)> func,
                             ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::dataChanged,
    this,
    [&, func = ::std::move(func)](::std::int32_t first,
                                  ::std::unique_ptr<Opaque> second,
                                  QPoint third,
                                  QPoint const& fourth) {
      const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
      func(*this,
           ::std::move(first),
           ::std::move(second),
           ::std::move(third),
           ::std::move(fourth));
    },
    type);
}

::QMetaObject::Connection
MyObject::newDataConnect(::rust::Fn<void(MyObject&,
                                         ::std::int32_t first,
                                         ::std::unique_ptr<Opaque> second,
                                         QPoint third,
                                         QPoint const& fourth)> func,
                         ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::newData,
    this,
    [&, func = ::std::move(func)](::std::int32_t first,
                                  ::std::unique_ptr<Opaque> second,
                                  QPoint third,
                                  QPoint const& fourth) {
      const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
      func(*this,
           ::std::move(first),
           ::std::move(second),
           ::std::move(third),
           ::std::move(fourth));
    },
    type);
}

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::createRs())
  , ::rust::cxxqtlib1::CxxQtLocking()
{
}

} // namespace cxx_qt::my_object

#include "moc_signals.cpp"

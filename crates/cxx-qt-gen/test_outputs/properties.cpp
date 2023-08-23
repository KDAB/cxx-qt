#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {
::std::int32_t const&
MyObject::getPrimitive() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return getPrimitiveWrapper();
}

void
MyObject::setPrimitive(::std::int32_t const& value)
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  setPrimitiveWrapper(value);
}

QPoint const&
MyObject::getTrivial() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return getTrivialWrapper();
}

void
MyObject::setTrivial(QPoint const& value)
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  setTrivialWrapper(value);
}

::QMetaObject::Connection
MyObject::primitiveChangedConnect(::rust::Fn<void(MyObject&)> func,
                                  ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::primitiveChanged,
    this,
    [&, func = ::std::move(func)]() {
      const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
      func(*this);
    },
    type);
}

::QMetaObject::Connection
MyObject::trivialChangedConnect(::rust::Fn<void(MyObject&)> func,
                                ::Qt::ConnectionType type)
{
  return ::QObject::connect(
    this,
    &MyObject::trivialChanged,
    this,
    [&, func = ::std::move(func)]() {
      const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
      func(*this);
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

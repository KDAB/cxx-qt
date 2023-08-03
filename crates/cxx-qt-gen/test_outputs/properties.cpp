#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::~MyObject() {}

MyObjectRust const&
MyObject::unsafeRust() const
{
  return *m_rustObj;
}

MyObjectRust&
MyObject::unsafeRustMut()
{
  return *m_rustObj;
}

::std::int32_t const&
MyObject::getPrimitive() const
{
  const auto guard = unsafeRustLock();
  return getPrimitiveWrapper();
}

void
MyObject::setPrimitive(::std::int32_t const& value)
{
  const auto guard = unsafeRustLock();
  setPrimitiveWrapper(value);
}

QPoint const&
MyObject::getTrivial() const
{
  const auto guard = unsafeRustLock();
  return getTrivialWrapper();
}

void
MyObject::setTrivial(QPoint const& value)
{
  const auto guard = unsafeRustLock();
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
      const auto guard = unsafeRustLock();
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
      const auto guard = unsafeRustLock();
      func(*this);
    },
    type);
}

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(::cxx_qt::my_object::cxx_qt_my_object::createRs())
{
}

} // namespace cxx_qt::my_object

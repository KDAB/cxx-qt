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
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return m_rustObj->getPrimitive(*this);
}

void
MyObject::setPrimitive(::std::int32_t const& value)
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setPrimitive(*this, value);
}

QPoint const&
MyObject::getTrivial() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return m_rustObj->getTrivial(*this);
}

void
MyObject::setTrivial(QPoint const& value)
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setTrivial(*this, value);
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
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
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
      const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
      func(*this);
    },
    type);
}

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
{
}

} // namespace cxx_qt::my_object

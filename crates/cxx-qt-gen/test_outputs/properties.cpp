#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
{
}

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

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
::std::unique_ptr<MyObject>
newCppObject()
{
  return ::std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object

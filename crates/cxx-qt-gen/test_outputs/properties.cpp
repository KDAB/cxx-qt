#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , m_rustObj(cxx_qt::my_object::cxx_qt_my_object::createRs())
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
  , m_cxxQtThreadObj(
      ::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(
        this))
{
}

MyObject::~MyObject()
{
  const auto guard = ::std::unique_lock(m_cxxQtThreadObj->mutex);
  m_cxxQtThreadObj->ptr = nullptr;
}

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

::std::unique_ptr<MyObjectCxxQtThread>
MyObject::qtThread() const
{
  return ::std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj,
                                                 m_rustObjMutex);
}

::std::int32_t const&
MyObject::getPrimitive() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t const&,
                                           ::std::int32_t const&>{}(
    m_rustObj->getPrimitive(*this));
}

void
MyObject::setPrimitive(::std::int32_t const& value)
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setPrimitive(
    *this,
    ::rust::cxxqtlib1::cxx_qt_convert<::std::int32_t, ::std::int32_t const&>{}(
      value));
}

QPoint const&
MyObject::getTrivial() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return ::rust::cxxqtlib1::cxx_qt_convert<QPoint const&, QPoint const&>{}(
    m_rustObj->getTrivial(*this));
}

void
MyObject::setTrivial(QPoint const& value)
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setTrivial(
    *this, ::rust::cxxqtlib1::cxx_qt_convert<QPoint, QPoint const&>{}(value));
}

Value const&
MyObject::getOpaque() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return ::rust::cxxqtlib1::cxx_qt_convert<Value const&,
                                           ::std::unique_ptr<Opaque> const&>{}(
    m_rustObj->getOpaque(*this));
}

void
MyObject::setOpaque(Value const& value)
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->setOpaque(
    *this,
    ::rust::cxxqtlib1::cxx_qt_convert<::std::unique_ptr<Opaque>,
                                      Value const&>{}(value));
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
::std::unique_ptr<MyObject>
newCppObject()
{
  return ::std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object

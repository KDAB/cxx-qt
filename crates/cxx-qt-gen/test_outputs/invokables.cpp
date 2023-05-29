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

void
MyObject::invokable() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableWrapper(*this);
}

void
MyObject::invokableMutable()
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableMutableWrapper(*this);
}

void
MyObject::invokableParameters(QColor const& opaque,
                              QPoint const& trivial,
                              ::std::int32_t primitive) const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableParametersWrapper(*this, opaque, trivial, primitive);
}

::std::unique_ptr<Opaque>
MyObject::invokableReturnOpaque()
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return m_rustObj->invokableReturnOpaqueWrapper(*this);
}

QPoint
MyObject::invokableReturnTrivial()
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  return m_rustObj->invokableReturnTrivialWrapper(*this);
}

void
MyObject::invokableFinal() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableFinalWrapper(*this);
}

void
MyObject::invokableOverride() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableOverrideWrapper(*this);
}

void
MyObject::invokableVirtual() const
{
  const ::std::lock_guard<::std::recursive_mutex> guard(*m_rustObjMutex);
  m_rustObj->invokableVirtualWrapper(*this);
}

::std::unique_ptr<MyObjectCxxQtThread>
MyObject::qtThread() const
{
  return ::std::make_unique<MyObjectCxxQtThread>(m_cxxQtThreadObj,
                                                 m_rustObjMutex);
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object::cxx_qt_my_object {
::std::unique_ptr<MyObject>
newCppObject()
{
  return ::std::make_unique<MyObject>();
}
} // namespace cxx_qt::my_object::cxx_qt_my_object

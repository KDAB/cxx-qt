#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {

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

static_assert(alignof(MyObjectCxxQtThread) <= alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(MyObjectCxxQtThread) == sizeof(::std::size_t[4]),
              "unexpected size");

MyObjectCxxQtThread
MyObject::qtThread() const
{
  return MyObjectCxxQtThread(m_cxxQtThreadObj, m_rustObjMutex);
}

MyObject::MyObject(::std::int32_t arg0, QObject* arg1)
  : MyObject(
      cxx_qt::my_object::cxx_qt_my_object::routeArguments0(std::move(arg0),
                                                           std::move(arg1)))
{
}

MyObject::MyObject(
  cxx_qt::my_object::cxx_qt_my_object::CxxQtConstructorArguments0&& args)
  : QObject(std::move(args.baseArguments.arg0))
  , m_rustObj(
      cxx_qt::my_object::cxx_qt_my_object::newRs0(std::move(args.newArguments)))
  , m_rustObjMutex(::std::make_shared<::std::recursive_mutex>())
  , m_cxxQtThreadObj(
      ::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(
        this))
{
  cxx_qt::my_object::cxx_qt_my_object::initialize0(
    *this, std::move(args.initializeArguments));
}

} // namespace cxx_qt::my_object

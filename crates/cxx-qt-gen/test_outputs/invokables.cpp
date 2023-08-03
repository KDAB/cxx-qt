#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {

MyObject::~MyObject()
{
  const auto guard = ::std::unique_lock(m_cxxQtThreadObj->mutex);
  m_cxxQtThreadObj->ptr = nullptr;
}

void
MyObject::cppMethod() const
{
  const auto guard = unsafeRustLock();
  cppMethodWrapper();
}

void
MyObject::invokable() const
{
  const auto guard = unsafeRustLock();
  invokableWrapper();
}

void
MyObject::invokableMutable()
{
  const auto guard = unsafeRustLock();
  invokableMutableWrapper();
}

void
MyObject::invokableParameters(QColor const& opaque,
                              QPoint const& trivial,
                              ::std::int32_t primitive) const
{
  const auto guard = unsafeRustLock();
  invokableParametersWrapper(opaque, trivial, primitive);
}

::std::unique_ptr<Opaque>
MyObject::invokableReturnOpaque()
{
  const auto guard = unsafeRustLock();
  return invokableReturnOpaqueWrapper();
}

QPoint
MyObject::invokableReturnTrivial()
{
  const auto guard = unsafeRustLock();
  return invokableReturnTrivialWrapper();
}

void
MyObject::invokableFinal() const
{
  const auto guard = unsafeRustLock();
  invokableFinalWrapper();
}

void
MyObject::invokableOverride() const
{
  const auto guard = unsafeRustLock();
  invokableOverrideWrapper();
}

void
MyObject::invokableVirtual() const
{
  const auto guard = unsafeRustLock();
  invokableVirtualWrapper();
}

void
MyObject::invokableResultTuple() const
{
  const auto guard = unsafeRustLock();
  invokableResultTupleWrapper();
}

::rust::String
MyObject::invokableResultType() const
{
  const auto guard = unsafeRustLock();
  return invokableResultTypeWrapper();
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

MyObject::MyObject(::std::int32_t arg0, QString const& arg1)
  : MyObject(
      ::cxx_qt::my_object::cxx_qt_my_object::routeArguments0(::std::move(arg0),
                                                             ::std::move(arg1)))
{
}

MyObject::MyObject()
  : MyObject(::cxx_qt::my_object::cxx_qt_my_object::routeArguments1())
{
}

MyObject::MyObject(
  ::cxx_qt::my_object::cxx_qt_my_object::CxxQtConstructorArguments0&& args)
  : QObject(::std::move(args.base.arg0))
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::newRs0(::std::move(args.new_)))
  , m_cxxQtThreadObj(
      ::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(
        this))
{
  ::cxx_qt::my_object::cxx_qt_my_object::initialize0(
    *this, ::std::move(args.initialize));
}

MyObject::MyObject(
  ::cxx_qt::my_object::cxx_qt_my_object::CxxQtConstructorArguments1&& args)
  : QObject()
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::newRs1(::std::move(args.new_)))
  , m_cxxQtThreadObj(
      ::std::make_shared<::rust::cxxqtlib1::CxxQtGuardedPointer<MyObject>>(
        this))
{
  ::cxx_qt::my_object::cxx_qt_my_object::initialize1(
    *this, ::std::move(args.initialize));
}

} // namespace cxx_qt::my_object

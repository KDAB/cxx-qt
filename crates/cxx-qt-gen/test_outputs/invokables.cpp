#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {
void
MyObject::cppMethod() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  cppMethodWrapper();
}

void
MyObject::invokable() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableWrapper();
}

void
MyObject::invokableMutable()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableMutableWrapper();
}

void
MyObject::invokableParameters(QColor const& opaque,
                              QPoint const& trivial,
                              ::std::int32_t primitive) const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableParametersWrapper(opaque, trivial, primitive);
}

::std::unique_ptr<Opaque>
MyObject::invokableReturnOpaque()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return invokableReturnOpaqueWrapper();
}

QPoint
MyObject::invokableReturnTrivial()
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return invokableReturnTrivialWrapper();
}

void
MyObject::invokableFinal() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableFinalWrapper();
}

void
MyObject::invokableOverride() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableOverrideWrapper();
}

void
MyObject::invokableVirtual() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableVirtualWrapper();
}

void
MyObject::invokableResultTuple() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  invokableResultTupleWrapper();
}

::rust::String
MyObject::invokableResultType() const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  return invokableResultTypeWrapper();
}

static_assert(alignof(MyObjectCxxQtThread) <= alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(MyObjectCxxQtThread) == sizeof(::std::size_t[4]),
              "unexpected size");

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
  , ::rust::cxxqtlib1::CxxQtThreading<MyObject>(this)
{
  ::cxx_qt::my_object::cxx_qt_my_object::initialize0(
    *this, ::std::move(args.initialize));
}

MyObject::MyObject(
  ::cxx_qt::my_object::cxx_qt_my_object::CxxQtConstructorArguments1&& args)
  : QObject()
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::newRs1(::std::move(args.new_)))
  , ::rust::cxxqtlib1::CxxQtThreading<MyObject>(this)
{
  ::cxx_qt::my_object::cxx_qt_my_object::initialize1(
    *this, ::std::move(args.initialize));
}

} // namespace cxx_qt::my_object

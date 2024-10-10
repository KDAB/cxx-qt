#include "directory/file_ident.cxxqt.h"

namespace cxx_qt::my_object {
static_assert(alignof(MyObjectCxxQtThread) <= alignof(::std::size_t),
              "unexpected aligment");
static_assert(sizeof(MyObjectCxxQtThread) == sizeof(::std::size_t[2]),
              "unexpected size");

MyObject::MyObject(::std::int32_t arg0, QString const& arg1)
  : MyObject(
      ::cxx_qt::my_object::cxx_qt_MyObject::routeArguments0(::std::move(arg0),
                                                            ::std::move(arg1)))
{
}

MyObject::MyObject()
  : MyObject(::cxx_qt::my_object::cxx_qt_MyObject::routeArguments1())
{
}

MyObject::MyObject(
  ::cxx_qt::my_object::cxx_qt_MyObject::CxxQtConstructorArguments0&& args)
  : QObject(::std::move(args.base.arg0))
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_MyObject::newRs0(::std::move(args.new_)))
  , ::rust::cxxqt1::CxxQtThreading<MyObject>(this)
{
  ::cxx_qt::my_object::cxx_qt_MyObject::initialize0(
    *this, ::std::move(args.initialize));
}

MyObject::MyObject(
  ::cxx_qt::my_object::cxx_qt_MyObject::CxxQtConstructorArguments1&& args)
  : QObject()
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_MyObject::newRs1(::std::move(args.new_)))
  , ::rust::cxxqt1::CxxQtThreading<MyObject>(this)
{
  ::cxx_qt::my_object::cxx_qt_MyObject::initialize1(
    *this, ::std::move(args.initialize));
}

} // namespace cxx_qt::my_object

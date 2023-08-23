#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {
void
MyObject::myInvokable(::cxx_qt::my_object::MyEnum qenum,
                      ::cxx_qt::my_object::MyOtherEnum other_qenum) const
{
  const ::rust::cxxqtlib1::MaybeLockGuard<MyObject> guard(*this);
  myInvokableWrapper(qenum, other_qenum);
}

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqtlib1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::createRs())
  , ::rust::cxxqtlib1::CxxQtLocking()
{
}

} // namespace cxx_qt::my_object

#include "cxx-qt-gen/ffi.cxxqt.h"

namespace cxx_qt::my_object {
void
MyObject::myInvokable(cxx_qt::my_object::MyEnum qenum,
                      my_namespace::MyOtherEnum other_qenum) const
{
  const ::rust::cxxqt1::MaybeLockGuard<MyObject> guard(*this);
  myInvokableWrapper(qenum, other_qenum);
}

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::createRs())
  , ::rust::cxxqt1::CxxQtLocking()
{
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object {
CxxName::CxxName(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<InternalObject>(
      ::cxx_qt::my_object::cxx_qt_my_renamed_object::createRs())
  , ::rust::cxxqt1::CxxQtLocking()
{
}

} // namespace cxx_qt::my_object

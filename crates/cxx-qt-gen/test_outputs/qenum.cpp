#include "directory/file_ident.cxxqt.h"

namespace cxx_qt::my_object {
MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_MyObject::createRs())
{
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object {
CxxName::CxxName(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<InternalObject>(
      ::cxx_qt::my_object::cxx_qt_MyRenamedObject::createRs())
{
}

} // namespace cxx_qt::my_object

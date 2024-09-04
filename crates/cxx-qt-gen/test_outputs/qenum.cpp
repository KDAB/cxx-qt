#include "directory/file_ident.cxxqt.h"

namespace cxx_qt::my_object {
MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(
      ::cxx_qt::my_object::cxx_qt_my_object::createRs())
{
}

} // namespace cxx_qt::my_object

namespace cxx_qt::my_object {
CxxName::CxxName(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<InternalObject>(
      ::cxx_qt::my_object::cxx_qt_my_renamed_object::createRs())
{
}

} // namespace cxx_qt::my_object

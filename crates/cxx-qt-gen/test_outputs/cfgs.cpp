#include "directory/file_ident.cxxqt.h"

MyObject::MyObject(QObject* parent)
  : QObject(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(::cxx_qt_MyObject::createRs())
{
}

#include "directory/file_ident.cxxqt.h"

MyObject::MyObject(QObject* parent)
  : QAbstractItemModel(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(::cxx_qt_MyObject::createRs())
{
}

#include "cxx-qt-gen/inheritance.cxxqt.h"

MyObject::MyObject(QObject* parent)
  : QAbstractItemModel(parent)
  , ::rust::cxxqt1::CxxQtType<MyObjectRust>(::cxx_qt_my_object::createRs())
{
}
